use flate2::read::GzDecoder;
use semver::Version;
use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fs::{self, File};
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use std::process::Command;
use tar::Archive;
use tempfile::TempDir;
use zip::ZipArchive;

const RELEASES_API_URL: &str = "https://api.github.com/repos/nosukeuehara/bitpet/releases";
const USER_AGENT: &str = concat!("bitpet/", env!("CARGO_PKG_VERSION"));

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UpdateOutcome {
    UpToDate { current: String },
    Available { current: String, latest: String },
    Updated { previous: String, current: String },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UpdateError {
    CurrentVersion(String),
    Network(String),
    GitHub(String),
    NoStableRelease,
    UnsupportedPlatform {
        os: &'static str,
        arch: &'static str,
    },
    MissingAsset {
        target: String,
    },
    MissingChecksum {
        asset: String,
    },
    ChecksumMismatch,
    Archive(String),
    ExecutableNotFound,
    Validation(String),
    Install(String),
}

impl Display for UpdateError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CurrentVersion(version) => {
                write!(
                    formatter,
                    "BitPet couldn't parse its current version: {version}"
                )
            }
            Self::Network(message) => {
                write!(
                    formatter,
                    "BitPet couldn't reach GitHub Releases: {message}"
                )
            }
            Self::GitHub(message) => {
                write!(formatter, "GitHub Releases returned an error: {message}")
            }
            Self::NoStableRelease => write!(formatter, "BitPet couldn't find a stable release."),
            Self::UnsupportedPlatform { os, arch } => write!(
                formatter,
                "Self-update is not available for this platform: {os}/{arch}"
            ),
            Self::MissingAsset { target } => {
                write!(
                    formatter,
                    "BitPet couldn't find a release asset for {target}."
                )
            }
            Self::MissingChecksum { asset } => {
                write!(formatter, "BitPet couldn't find checksum file for {asset}.")
            }
            Self::ChecksumMismatch => {
                write!(
                    formatter,
                    "Downloaded BitPet archive did not match its checksum."
                )
            }
            Self::Archive(message) => {
                write!(formatter, "BitPet couldn't extract the update: {message}")
            }
            Self::ExecutableNotFound => {
                write!(
                    formatter,
                    "BitPet couldn't find the executable in the update archive."
                )
            }
            Self::Validation(message) => {
                write!(
                    formatter,
                    "Downloaded BitPet executable could not be validated: {message}"
                )
            }
            Self::Install(message) => {
                write!(formatter, "BitPet couldn't install the update: {message}")
            }
        }
    }
}

impl Error for UpdateError {}

pub type UpdateResult<T> = Result<T, UpdateError>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Release {
    pub tag_name: String,
    pub version: Version,
    pub assets: Vec<ReleaseAsset>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseAsset {
    pub name: String,
    pub download_url: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArchiveKind {
    TarGz,
    Zip,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlatformTarget {
    pub target: &'static str,
    pub archive: ArchiveKind,
    pub executable_name: &'static str,
}

impl PlatformTarget {
    fn archive_suffix(self) -> &'static str {
        match self.archive {
            ArchiveKind::TarGz => ".tar.gz",
            ArchiveKind::Zip => ".zip",
        }
    }

    fn asset_suffix(self) -> String {
        format!("-{}{}", self.target, self.archive_suffix())
    }
}

pub trait ReleaseClient {
    fn stable_releases(&self) -> UpdateResult<Vec<Release>>;
    fn download_to(&self, url: &str, destination: &Path) -> UpdateResult<()>;
}

pub trait PlatformDetector {
    fn detect(&self) -> UpdateResult<PlatformTarget>;
}

pub trait ArchiveExtractor {
    fn extract_executable(
        &self,
        archive_path: &Path,
        platform: PlatformTarget,
        destination_dir: &Path,
    ) -> UpdateResult<PathBuf>;
}

pub trait ExecutableInstaller {
    fn install(&self, executable_path: &Path, expected_version: &Version) -> UpdateResult<()>;
}

pub struct UpdateService<C, P, A, I> {
    client: C,
    platform: P,
    extractor: A,
    installer: I,
}

impl<C, P, A, I> UpdateService<C, P, A, I>
where
    C: ReleaseClient,
    P: PlatformDetector,
    A: ArchiveExtractor,
    I: ExecutableInstaller,
{
    pub const fn new(client: C, platform: P, extractor: A, installer: I) -> Self {
        Self {
            client,
            platform,
            extractor,
            installer,
        }
    }

    pub fn check(&self, current_version: &str) -> UpdateResult<UpdateOutcome> {
        let current = parse_version(current_version)?;
        let latest = self.latest_release()?;

        if latest.version > current {
            return Ok(UpdateOutcome::Available {
                current: format_version(&current),
                latest: format_version(&latest.version),
            });
        }

        Ok(UpdateOutcome::UpToDate {
            current: format_version(&current),
        })
    }

    pub fn update(&self, current_version: &str) -> UpdateResult<UpdateOutcome> {
        let current = parse_version(current_version)?;
        let latest = self.latest_release()?;

        if latest.version <= current {
            return Ok(UpdateOutcome::UpToDate {
                current: format_version(&current),
            });
        }

        let platform = self.platform.detect()?;
        let asset = resolve_release_asset(&latest, platform)?;
        let checksum_asset = resolve_checksum_asset(&latest, asset)?;
        let workspace = TempDir::new().map_err(install_io_error)?;
        let archive_path = workspace.path().join(&asset.name);
        let checksum_path = workspace.path().join(&checksum_asset.name);

        self.client
            .download_to(&asset.download_url, &archive_path)?;
        self.client
            .download_to(&checksum_asset.download_url, &checksum_path)?;
        verify_checksum_file(&archive_path, &checksum_path)?;

        let extract_dir = workspace.path().join("extract");
        fs::create_dir_all(&extract_dir).map_err(archive_io_error)?;
        let executable_path =
            self.extractor
                .extract_executable(&archive_path, platform, &extract_dir)?;

        self.installer.install(&executable_path, &latest.version)?;

        Ok(UpdateOutcome::Updated {
            previous: format_version(&current),
            current: format_version(&latest.version),
        })
    }

    fn latest_release(&self) -> UpdateResult<Release> {
        self.client
            .stable_releases()?
            .into_iter()
            .filter(|release| release.version.pre.is_empty())
            .max_by(|left, right| left.version.cmp(&right.version))
            .ok_or(UpdateError::NoStableRelease)
    }
}

pub fn check_for_updates(current_version: &str) -> UpdateResult<UpdateOutcome> {
    default_service().check(current_version)
}

pub fn update(current_version: &str) -> UpdateResult<UpdateOutcome> {
    default_service().update(current_version)
}

fn default_service() -> UpdateService<
    GithubReleaseClient,
    NativePlatformDetector,
    NativeArchiveExtractor,
    CurrentExecutableInstaller<SystemVersionValidator>,
> {
    UpdateService::new(
        GithubReleaseClient,
        NativePlatformDetector,
        NativeArchiveExtractor,
        CurrentExecutableInstaller::new(SystemVersionValidator),
    )
}

#[derive(Debug, Clone, Copy)]
pub struct GithubReleaseClient;

impl ReleaseClient for GithubReleaseClient {
    fn stable_releases(&self) -> UpdateResult<Vec<Release>> {
        let response = ureq::get(RELEASES_API_URL)
            .set("Accept", "application/vnd.github+json")
            .set("User-Agent", USER_AGENT)
            .call()
            .map_err(github_error)?;

        let releases: Vec<GithubRelease> = response
            .into_json()
            .map_err(|error| UpdateError::GitHub(error.to_string()))?;

        Ok(releases
            .into_iter()
            .filter(|release| !release.draft && !release.prerelease)
            .filter_map(|release| release.try_into_release())
            .collect())
    }

    fn download_to(&self, url: &str, destination: &Path) -> UpdateResult<()> {
        let response = ureq::get(url)
            .set("Accept", "application/octet-stream")
            .set("User-Agent", USER_AGENT)
            .call()
            .map_err(github_error)?;
        let mut reader = response.into_reader();
        let mut file = File::create(destination).map_err(download_io_error)?;
        io::copy(&mut reader, &mut file).map_err(download_io_error)?;
        Ok(())
    }
}

#[derive(Debug, Deserialize)]
struct GithubRelease {
    tag_name: String,
    draft: bool,
    prerelease: bool,
    assets: Vec<GithubAsset>,
}

impl GithubRelease {
    fn try_into_release(self) -> Option<Release> {
        let version = parse_version(&self.tag_name).ok()?;
        let assets = self
            .assets
            .into_iter()
            .map(|asset| ReleaseAsset {
                name: asset.name,
                download_url: asset.browser_download_url,
            })
            .collect();

        Some(Release {
            tag_name: self.tag_name,
            version,
            assets,
        })
    }
}

#[derive(Debug, Deserialize)]
struct GithubAsset {
    name: String,
    browser_download_url: String,
}

#[derive(Debug, Clone, Copy)]
pub struct NativePlatformDetector;

impl PlatformDetector for NativePlatformDetector {
    fn detect(&self) -> UpdateResult<PlatformTarget> {
        platform_target(std::env::consts::OS, std::env::consts::ARCH)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct NativeArchiveExtractor;

impl ArchiveExtractor for NativeArchiveExtractor {
    fn extract_executable(
        &self,
        archive_path: &Path,
        platform: PlatformTarget,
        destination_dir: &Path,
    ) -> UpdateResult<PathBuf> {
        match platform.archive {
            ArchiveKind::TarGz => {
                extract_tar_gz_executable(archive_path, platform, destination_dir)
            }
            ArchiveKind::Zip => extract_zip_executable(archive_path, platform, destination_dir),
        }
    }
}

pub trait ExecutableVersionValidator {
    fn validate(&self, executable_path: &Path, expected_version: &Version) -> UpdateResult<()>;
}

#[derive(Debug, Clone, Copy)]
pub struct SystemVersionValidator;

impl ExecutableVersionValidator for SystemVersionValidator {
    fn validate(&self, executable_path: &Path, expected_version: &Version) -> UpdateResult<()> {
        let output = Command::new(executable_path)
            .arg("--version")
            .output()
            .map_err(|error| UpdateError::Validation(error.to_string()))?;

        if !output.status.success() {
            return Err(UpdateError::Validation(format!(
                "version command exited with {}",
                output.status
            )));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let actual = parse_version(stdout.trim())
            .map_err(|_| UpdateError::Validation(stdout.trim().to_string()))?;

        if actual != *expected_version {
            return Err(UpdateError::Validation(format!(
                "expected {}, got {}",
                format_version(expected_version),
                format_version(&actual)
            )));
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct CurrentExecutableInstaller<V> {
    current_exe: PathBuf,
    validator: V,
}

impl<V> CurrentExecutableInstaller<V>
where
    V: ExecutableVersionValidator,
{
    pub fn new(validator: V) -> Self {
        let current_exe = std::env::current_exe().unwrap_or_else(|_| PathBuf::from("bitpet"));
        Self {
            current_exe,
            validator,
        }
    }

    #[cfg(test)]
    fn with_current_exe(current_exe: PathBuf, validator: V) -> Self {
        Self {
            current_exe,
            validator,
        }
    }
}

impl<V> ExecutableInstaller for CurrentExecutableInstaller<V>
where
    V: ExecutableVersionValidator,
{
    fn install(&self, executable_path: &Path, expected_version: &Version) -> UpdateResult<()> {
        self.validator.validate(executable_path, expected_version)?;
        replace_executable_safely(&self.current_exe, executable_path, |installed| {
            self.validator.validate(installed, expected_version)
        })
    }
}

pub fn platform_target(os: &'static str, arch: &'static str) -> UpdateResult<PlatformTarget> {
    match (os, arch) {
        ("macos", "aarch64") => Ok(PlatformTarget {
            target: "aarch64-apple-darwin",
            archive: ArchiveKind::TarGz,
            executable_name: "bitpet",
        }),
        ("macos", "x86_64") => Ok(PlatformTarget {
            target: "x86_64-apple-darwin",
            archive: ArchiveKind::TarGz,
            executable_name: "bitpet",
        }),
        ("linux", "x86_64") => Ok(PlatformTarget {
            target: "x86_64-unknown-linux-gnu",
            archive: ArchiveKind::TarGz,
            executable_name: "bitpet",
        }),
        ("windows", "x86_64") => Ok(PlatformTarget {
            target: "x86_64-pc-windows-msvc",
            archive: ArchiveKind::Zip,
            executable_name: "bitpet.exe",
        }),
        _ => Err(UpdateError::UnsupportedPlatform { os, arch }),
    }
}

pub fn resolve_release_asset(
    release: &Release,
    platform: PlatformTarget,
) -> UpdateResult<&ReleaseAsset> {
    let suffix = platform.asset_suffix();
    release
        .assets
        .iter()
        .find(|asset| asset.name.ends_with(&suffix) && !asset.name.ends_with(".sha256"))
        .ok_or_else(|| UpdateError::MissingAsset {
            target: platform.target.to_string(),
        })
}

pub fn resolve_checksum_asset<'a>(
    release: &'a Release,
    archive_asset: &ReleaseAsset,
) -> UpdateResult<&'a ReleaseAsset> {
    let checksum_name = format!("{}.sha256", archive_asset.name);
    release
        .assets
        .iter()
        .find(|asset| asset.name == checksum_name)
        .ok_or_else(|| UpdateError::MissingChecksum {
            asset: archive_asset.name.clone(),
        })
}

pub fn verify_checksum_file(archive_path: &Path, checksum_path: &Path) -> UpdateResult<()> {
    let expected = expected_checksum(checksum_path)?;
    let actual = file_sha256(archive_path)?;

    if actual.eq_ignore_ascii_case(&expected) {
        Ok(())
    } else {
        Err(UpdateError::ChecksumMismatch)
    }
}

fn expected_checksum(checksum_path: &Path) -> UpdateResult<String> {
    let contents = fs::read_to_string(checksum_path).map_err(download_io_error)?;
    let checksum = contents
        .split_whitespace()
        .next()
        .ok_or(UpdateError::ChecksumMismatch)?;

    if checksum.len() == 64 && checksum.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        Ok(checksum.to_string())
    } else {
        Err(UpdateError::ChecksumMismatch)
    }
}

fn file_sha256(path: &Path) -> UpdateResult<String> {
    let mut file = File::open(path).map_err(download_io_error)?;
    let mut hasher = Sha256::new();
    let mut buffer = [0; 8192];

    loop {
        let read = file.read(&mut buffer).map_err(download_io_error)?;
        if read == 0 {
            break;
        }
        hasher.update(&buffer[..read]);
    }

    Ok(format!("{:x}", hasher.finalize()))
}

fn extract_tar_gz_executable(
    archive_path: &Path,
    platform: PlatformTarget,
    destination_dir: &Path,
) -> UpdateResult<PathBuf> {
    let archive_file = File::open(archive_path).map_err(archive_io_error)?;
    let decoder = GzDecoder::new(archive_file);
    let mut archive = Archive::new(decoder);
    archive.unpack(destination_dir).map_err(archive_io_error)?;
    find_executable(destination_dir, platform.executable_name)
}

fn extract_zip_executable(
    archive_path: &Path,
    platform: PlatformTarget,
    destination_dir: &Path,
) -> UpdateResult<PathBuf> {
    let archive_file = File::open(archive_path).map_err(archive_io_error)?;
    let mut archive =
        ZipArchive::new(archive_file).map_err(|error| UpdateError::Archive(error.to_string()))?;

    for index in 0..archive.len() {
        let mut file = archive
            .by_index(index)
            .map_err(|error| UpdateError::Archive(error.to_string()))?;
        let enclosed = file
            .enclosed_name()
            .ok_or_else(|| UpdateError::Archive("zip entry escapes destination".to_string()))?
            .to_path_buf();
        let output_path = destination_dir.join(enclosed);

        if file.is_dir() {
            fs::create_dir_all(&output_path).map_err(archive_io_error)?;
            continue;
        }

        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent).map_err(archive_io_error)?;
        }

        let mut output = File::create(&output_path).map_err(archive_io_error)?;
        io::copy(&mut file, &mut output).map_err(archive_io_error)?;
    }

    find_executable(destination_dir, platform.executable_name)
}

fn find_executable(directory: &Path, executable_name: &str) -> UpdateResult<PathBuf> {
    let mut stack = vec![directory.to_path_buf()];

    while let Some(path) = stack.pop() {
        for entry in fs::read_dir(&path).map_err(archive_io_error)? {
            let entry = entry.map_err(archive_io_error)?;
            let entry_path = entry.path();

            if entry_path.is_dir() {
                stack.push(entry_path);
            } else if entry_path
                .file_name()
                .is_some_and(|name| name == executable_name)
            {
                ensure_executable_permissions(&entry_path)?;
                return Ok(entry_path);
            }
        }
    }

    Err(UpdateError::ExecutableNotFound)
}

fn replace_executable_safely(
    current_exe: &Path,
    new_exe: &Path,
    validate_installed: impl FnOnce(&Path) -> UpdateResult<()>,
) -> UpdateResult<()> {
    let parent = current_exe.parent().ok_or_else(|| {
        UpdateError::Install("current executable has no parent directory".to_string())
    })?;
    let file_name = current_exe
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| UpdateError::Install("current executable name is not UTF-8".to_string()))?;
    let process_id = std::process::id();
    let backup = parent.join(format!("{file_name}.old-{process_id}"));
    let staging = parent.join(format!("{file_name}.new-{process_id}"));

    if backup.exists() {
        fs::remove_file(&backup).map_err(install_io_error)?;
    }
    if staging.exists() {
        fs::remove_file(&staging).map_err(install_io_error)?;
    }

    fs::copy(new_exe, &staging).map_err(install_io_error)?;
    copy_executable_permissions(current_exe, &staging)?;

    if let Err(error) = fs::rename(current_exe, &backup) {
        let _ = fs::remove_file(&staging);
        return Err(install_io_error(error));
    }

    if let Err(error) = fs::rename(&staging, current_exe) {
        let _ = fs::rename(&backup, current_exe);
        let _ = fs::remove_file(&staging);
        return Err(install_io_error(error));
    }

    if let Err(error) = validate_installed(current_exe) {
        let _ = fs::remove_file(current_exe);
        let _ = fs::rename(&backup, current_exe);
        return Err(error);
    }

    fs::remove_file(&backup).map_err(install_io_error)?;
    Ok(())
}

fn parse_version(value: &str) -> UpdateResult<Version> {
    Version::parse(value.trim().trim_start_matches('v'))
        .map_err(|_| UpdateError::CurrentVersion(value.to_string()))
}

fn format_version(version: &Version) -> String {
    format!("v{version}")
}

fn github_error(error: ureq::Error) -> UpdateError {
    match error {
        ureq::Error::Status(403, response) => {
            let remaining = response.header("x-ratelimit-remaining").unwrap_or_default();
            if remaining == "0" {
                UpdateError::GitHub("GitHub API rate limit exceeded. Try again later.".to_string())
            } else {
                UpdateError::GitHub("request was forbidden".to_string())
            }
        }
        ureq::Error::Status(status, response) => {
            UpdateError::GitHub(format!("HTTP {status}: {}", response.status_text()))
        }
        ureq::Error::Transport(error) => UpdateError::Network(error.to_string()),
    }
}

fn download_io_error(error: io::Error) -> UpdateError {
    UpdateError::Network(error.to_string())
}

fn archive_io_error(error: io::Error) -> UpdateError {
    UpdateError::Archive(error.to_string())
}

fn install_io_error(error: io::Error) -> UpdateError {
    UpdateError::Install(error.to_string())
}

#[cfg(unix)]
fn ensure_executable_permissions(path: &Path) -> UpdateResult<()> {
    use std::os::unix::fs::PermissionsExt;

    let mut permissions = fs::metadata(path).map_err(archive_io_error)?.permissions();
    permissions.set_mode(permissions.mode() | 0o755);
    fs::set_permissions(path, permissions).map_err(archive_io_error)
}

#[cfg(not(unix))]
fn ensure_executable_permissions(_path: &Path) -> UpdateResult<()> {
    Ok(())
}

#[cfg(unix)]
fn copy_executable_permissions(from: &Path, to: &Path) -> UpdateResult<()> {
    let permissions = fs::metadata(from).map_err(install_io_error)?.permissions();
    fs::set_permissions(to, permissions).map_err(install_io_error)
}

#[cfg(not(unix))]
fn copy_executable_permissions(_from: &Path, _to: &Path) -> UpdateResult<()> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;
    use std::collections::HashMap;
    use std::rc::Rc;

    #[derive(Clone)]
    struct FakeClient {
        releases: Vec<Release>,
        downloads: HashMap<String, Vec<u8>>,
        fail_download: Option<String>,
    }

    impl FakeClient {
        fn new(releases: Vec<Release>) -> Self {
            Self {
                releases,
                downloads: HashMap::new(),
                fail_download: None,
            }
        }

        fn with_download(mut self, url: &str, contents: Vec<u8>) -> Self {
            self.downloads.insert(url.to_string(), contents);
            self
        }

        fn with_download_failure(mut self, url: &str) -> Self {
            self.fail_download = Some(url.to_string());
            self
        }
    }

    impl ReleaseClient for FakeClient {
        fn stable_releases(&self) -> UpdateResult<Vec<Release>> {
            Ok(self.releases.clone())
        }

        fn download_to(&self, url: &str, destination: &Path) -> UpdateResult<()> {
            if self.fail_download.as_deref() == Some(url) {
                return Err(UpdateError::Network("download failed".to_string()));
            }
            let contents = self
                .downloads
                .get(url)
                .ok_or_else(|| UpdateError::Network(format!("missing fake download: {url}")))?;
            fs::write(destination, contents).map_err(download_io_error)
        }
    }

    #[derive(Clone)]
    struct FakePlatform(UpdateResult<PlatformTarget>);

    impl PlatformDetector for FakePlatform {
        fn detect(&self) -> UpdateResult<PlatformTarget> {
            self.0.clone()
        }
    }

    #[derive(Clone)]
    struct FakeExtractor {
        executable: PathBuf,
    }

    impl ArchiveExtractor for FakeExtractor {
        fn extract_executable(
            &self,
            _archive_path: &Path,
            _platform: PlatformTarget,
            _destination_dir: &Path,
        ) -> UpdateResult<PathBuf> {
            Ok(self.executable.clone())
        }
    }

    #[derive(Clone)]
    struct FakeInstaller {
        calls: Rc<Cell<u32>>,
        result: UpdateResult<()>,
    }

    impl ExecutableInstaller for FakeInstaller {
        fn install(
            &self,
            _executable_path: &Path,
            _expected_version: &Version,
        ) -> UpdateResult<()> {
            self.calls.set(self.calls.get() + 1);
            self.result.clone()
        }
    }

    #[derive(Clone)]
    struct StaticValidator {
        pre: UpdateResult<()>,
        post: UpdateResult<()>,
        calls: Rc<Cell<u32>>,
    }

    impl ExecutableVersionValidator for StaticValidator {
        fn validate(
            &self,
            _executable_path: &Path,
            _expected_version: &Version,
        ) -> UpdateResult<()> {
            let call = self.calls.get();
            self.calls.set(call + 1);
            if call == 0 {
                self.pre.clone()
            } else {
                self.post.clone()
            }
        }
    }

    fn release(tag: &str, assets: Vec<&str>) -> Release {
        Release {
            tag_name: tag.to_string(),
            version: parse_version(tag).expect("test version should parse"),
            assets: assets
                .into_iter()
                .map(|name| ReleaseAsset {
                    name: name.to_string(),
                    download_url: format!("https://example.test/{name}"),
                })
                .collect(),
        }
    }

    fn linux_target() -> PlatformTarget {
        platform_target("linux", "x86_64").expect("linux target should be supported")
    }

    fn archive_bytes() -> Vec<u8> {
        b"archive".to_vec()
    }

    fn checksum_bytes(bytes: &[u8], filename: &str) -> Vec<u8> {
        format!("{:x}  {filename}\n", Sha256::digest(bytes)).into_bytes()
    }

    fn service(
        client: FakeClient,
        installer: FakeInstaller,
        executable: PathBuf,
    ) -> UpdateService<FakeClient, FakePlatform, FakeExtractor, FakeInstaller> {
        UpdateService::new(
            client,
            FakePlatform(Ok(linux_target())),
            FakeExtractor { executable },
            installer,
        )
    }

    #[test]
    fn already_latest_reports_up_to_date() {
        let client = FakeClient::new(vec![release("v1.1.0", vec![])]);
        let installer = FakeInstaller {
            calls: Rc::new(Cell::new(0)),
            result: Ok(()),
        };
        let service = service(client, installer.clone(), PathBuf::from("bitpet"));

        assert_eq!(
            service.check("1.1.0"),
            Ok(UpdateOutcome::UpToDate {
                current: "v1.1.0".to_string()
            })
        );
        assert_eq!(installer.calls.get(), 0);
    }

    #[test]
    fn newer_stable_release_available() {
        let client = FakeClient::new(vec![release("v1.1.0", vec![])]);
        let installer = FakeInstaller {
            calls: Rc::new(Cell::new(0)),
            result: Ok(()),
        };
        let service = service(client, installer, PathBuf::from("bitpet"));

        assert_eq!(
            service.check("1.0.0"),
            Ok(UpdateOutcome::Available {
                current: "v1.0.0".to_string(),
                latest: "v1.1.0".to_string()
            })
        );
    }

    #[test]
    fn older_or_equal_versions_do_not_update() {
        let client = FakeClient::new(vec![release("v1.0.0", vec![]), release("v0.9.0", vec![])]);
        let installer = FakeInstaller {
            calls: Rc::new(Cell::new(0)),
            result: Ok(()),
        };
        let service = service(client, installer, PathBuf::from("bitpet"));

        assert!(matches!(
            service.check("1.0.0"),
            Ok(UpdateOutcome::UpToDate { .. })
        ));
        assert!(matches!(
            service.check("1.1.0"),
            Ok(UpdateOutcome::UpToDate { .. })
        ));
    }

    #[test]
    fn prerelease_versions_are_not_update_targets() {
        let client = FakeClient::new(vec![
            release("v1.2.0-beta.1", vec![]),
            release("v1.2.0-rc.1", vec![]),
            release("v1.1.0", vec![]),
        ]);
        let installer = FakeInstaller {
            calls: Rc::new(Cell::new(0)),
            result: Ok(()),
        };
        let service = service(client, installer, PathBuf::from("bitpet"));

        assert_eq!(
            service.check("1.0.0"),
            Ok(UpdateOutcome::Available {
                current: "v1.0.0".to_string(),
                latest: "v1.1.0".to_string()
            })
        );
    }

    #[test]
    fn platform_maps_to_release_targets() {
        assert_eq!(
            platform_target("macos", "aarch64").expect("target").target,
            "aarch64-apple-darwin"
        );
        assert_eq!(
            platform_target("macos", "x86_64").expect("target").target,
            "x86_64-apple-darwin"
        );
        assert_eq!(
            platform_target("linux", "x86_64").expect("target").target,
            "x86_64-unknown-linux-gnu"
        );
        assert_eq!(
            platform_target("windows", "x86_64").expect("target").target,
            "x86_64-pc-windows-msvc"
        );
    }

    #[test]
    fn unsupported_platform_returns_error() {
        assert!(matches!(
            platform_target("linux", "aarch64"),
            Err(UpdateError::UnsupportedPlatform { .. })
        ));
    }

    #[test]
    fn resolves_assets_by_workflow_target_suffix() {
        let release = release(
            "v1.1.0",
            vec![
                "bitpet-v1.1.0-x86_64-unknown-linux-gnu.tar.gz",
                "bitpet-v1.1.0-x86_64-unknown-linux-gnu.tar.gz.sha256",
            ],
        );
        let asset = resolve_release_asset(&release, linux_target()).expect("asset should resolve");
        let checksum = resolve_checksum_asset(&release, asset).expect("checksum should resolve");

        assert_eq!(asset.name, "bitpet-v1.1.0-x86_64-unknown-linux-gnu.tar.gz");
        assert_eq!(
            checksum.name,
            "bitpet-v1.1.0-x86_64-unknown-linux-gnu.tar.gz.sha256"
        );
    }

    #[test]
    fn missing_asset_returns_error() {
        let release = release("v1.1.0", vec!["bitpet-v1.1.0-aarch64-apple-darwin.tar.gz"]);

        assert!(matches!(
            resolve_release_asset(&release, linux_target()),
            Err(UpdateError::MissingAsset { .. })
        ));
    }

    #[test]
    fn download_failure_stops_update() {
        let archive = "bitpet-v1.1.0-x86_64-unknown-linux-gnu.tar.gz";
        let checksum = format!("{archive}.sha256");
        let client = FakeClient::new(vec![release("v1.1.0", vec![archive, &checksum])])
            .with_download_failure(&format!("https://example.test/{archive}"));
        let installer = FakeInstaller {
            calls: Rc::new(Cell::new(0)),
            result: Ok(()),
        };
        let service = service(client, installer.clone(), PathBuf::from("bitpet"));

        assert!(matches!(
            service.update("1.0.0"),
            Err(UpdateError::Network(_))
        ));
        assert_eq!(installer.calls.get(), 0);
    }

    #[test]
    fn checksum_success_accepts_archive() {
        let dir = tempfile::tempdir().expect("tempdir");
        let archive = dir.path().join("archive.tar.gz");
        let checksum = dir.path().join("archive.tar.gz.sha256");
        let bytes = archive_bytes();
        fs::write(&archive, &bytes).expect("archive");
        fs::write(&checksum, checksum_bytes(&bytes, "archive.tar.gz")).expect("checksum");

        assert_eq!(verify_checksum_file(&archive, &checksum), Ok(()));
    }

    #[test]
    fn checksum_mismatch_returns_error() {
        let dir = tempfile::tempdir().expect("tempdir");
        let archive = dir.path().join("archive.tar.gz");
        let checksum = dir.path().join("archive.tar.gz.sha256");
        fs::write(&archive, archive_bytes()).expect("archive");
        fs::write(
            &checksum,
            b"0000000000000000000000000000000000000000000000000000000000000000  archive.tar.gz\n",
        )
        .expect("checksum");

        assert_eq!(
            verify_checksum_file(&archive, &checksum),
            Err(UpdateError::ChecksumMismatch)
        );
    }

    #[test]
    fn install_failure_is_reported() {
        let archive = "bitpet-v1.1.0-x86_64-unknown-linux-gnu.tar.gz";
        let checksum = format!("{archive}.sha256");
        let archive_bytes = archive_bytes();
        let client = FakeClient::new(vec![release("v1.1.0", vec![archive, &checksum])])
            .with_download(
                &format!("https://example.test/{archive}"),
                archive_bytes.clone(),
            )
            .with_download(
                &format!("https://example.test/{checksum}"),
                checksum_bytes(&archive_bytes, archive),
            );
        let installer = FakeInstaller {
            calls: Rc::new(Cell::new(0)),
            result: Err(UpdateError::Install("permission denied".to_string())),
        };
        let service = service(client, installer, PathBuf::from("bitpet"));

        assert!(matches!(
            service.update("1.0.0"),
            Err(UpdateError::Install(_))
        ));
    }

    #[test]
    fn current_binary_preserved_when_final_validation_fails() {
        let dir = tempfile::tempdir().expect("tempdir");
        let current = dir.path().join("bitpet");
        let new = dir.path().join("new-bitpet");
        fs::write(&current, b"current").expect("current");
        fs::write(&new, b"new").expect("new");
        let calls = Rc::new(Cell::new(0));
        let validator = StaticValidator {
            pre: Ok(()),
            post: Err(UpdateError::Validation("bad install".to_string())),
            calls,
        };
        let installer = CurrentExecutableInstaller::with_current_exe(current.clone(), validator);

        assert!(matches!(
            installer.install(&new, &parse_version("1.1.0").expect("version")),
            Err(UpdateError::Validation(_))
        ));
        assert_eq!(
            fs::read(&current).expect("current should remain"),
            b"current"
        );
    }

    #[test]
    fn check_does_not_modify_files_or_call_installer() {
        let dir = tempfile::tempdir().expect("tempdir");
        let save_dir = dir.path().join(".bitpet");
        fs::create_dir_all(&save_dir).expect("save dir");
        let save = save_dir.join("save.json");
        fs::write(&save, b"save").expect("save");
        let client = FakeClient::new(vec![release("v1.1.0", vec![])]);
        let calls = Rc::new(Cell::new(0));
        let installer = FakeInstaller {
            calls: Rc::clone(&calls),
            result: Ok(()),
        };
        let service = service(client, installer, PathBuf::from("bitpet"));

        let _ = service.check("1.0.0").expect("check should succeed");

        assert_eq!(calls.get(), 0);
        assert_eq!(fs::read(&save).expect("save untouched"), b"save");
    }
}
