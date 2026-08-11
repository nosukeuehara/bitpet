use std::process::Command as ProcessCommand;

fn bitpet(args: &[&str]) -> String {
    let output = ProcessCommand::new(env!("CARGO_BIN_EXE_bitpet"))
        .args(args)
        .output()
        .expect("bitpet binary should run");

    assert!(
        output.status.success(),
        "bitpet should exit successfully: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    String::from_utf8(output.stdout).expect("stdout should be valid utf-8")
}

#[test]
fn help_flag_shows_commands_and_options() {
    for flag in ["--help", "-h"] {
        let output = bitpet(&[flag]);

        assert!(output.contains("BitPet - a tiny CLI pet"));
        assert!(output.contains("Usage:"));
        assert!(output.contains("status    Show your BitPet"));
        assert!(output.contains("feed      Feed your BitPet"));
        assert!(output.contains("play      Play with your BitPet"));
        assert!(output.contains("go        Send your BitPet on an expedition"));
        assert!(output.contains("report    Show today's activity report"));
        assert!(output.contains("streak    Show your login streak"));
        assert!(output.contains("help      Show help for a command"));
        assert!(output.contains("-h, --help"));
        assert!(output.contains("-V, --version"));
    }
}

#[test]
fn subcommand_help_flags_show_command_help() {
    let cases = [
        ("status", "Show your BitPet"),
        ("feed", "Feed your BitPet"),
        ("play", "Play with your BitPet"),
        ("go", "Send your BitPet on an expedition"),
        ("report", "Show today's activity report"),
        ("streak", "Show your login streak"),
    ];

    for (command, description) in cases {
        let output = bitpet(&[command, "--help"]);

        assert!(output.contains(description));
        assert!(output.contains("Usage:"));
        assert!(output.contains(&format!("bitpet {command}")));
        assert!(output.contains("-h, --help"));
    }
}

#[test]
fn version_flags_use_package_version() {
    for flag in ["--version", "-V"] {
        let output = bitpet(&[flag]);

        assert_eq!(output.trim(), env!("CARGO_PKG_VERSION"));
    }
}
