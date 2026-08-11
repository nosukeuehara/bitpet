use bitpet::ascii::monsters::art_by_id;
use bitpet::ascii::pets::pet_art;
use bitpet::domain::evolution::GrowthStage;
use bitpet::domain::monster::SpeciesId;
use bitpet::domain::Pet;

#[test]
fn ascii_lookup_finds_species_art() {
    let art = art_by_id("mofflet").expect("mofflet art should exist");

    assert_eq!(art.name, "Mofflet");
    assert_eq!(art.family, "fuzz");
    assert_eq!(art.stage, "stage1");
    assert!(!art.art.trim().is_empty());
}

#[test]
fn pet_art_uses_species_id() {
    let mut pet = Pet::new("Mochi".to_string(), 2, 10, 72, 72, 72);
    pet.stage = GrowthStage::Stage1;
    pet.species_id = SpeciesId::Mofflet;

    assert_eq!(pet_art(&pet), art_by_id("mofflet").unwrap().art);
}
