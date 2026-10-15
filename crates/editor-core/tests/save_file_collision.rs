mod support;

#[test]
fn save_preserves_an_existing_temporary_file_and_document_state() {
    support::assert_collision_preserved(|collision, _| {
        std::fs::write(collision, b"preexisting collision data").unwrap();
    });
}
