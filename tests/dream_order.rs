use selmem::dream::NIGHT_PASSES;

#[test]
fn night_passes_stay_in_scientific_order() {
    assert_eq!(
        NIGHT_PASSES,
        &["weather", "rewrite", "merge", "ladder", "release"]
    );
}
