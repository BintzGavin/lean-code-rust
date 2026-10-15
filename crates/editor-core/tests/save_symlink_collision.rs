#![cfg(unix)]

mod support;

#[test]
fn save_preserves_an_existing_temporary_symlink_and_its_target() {
    support::assert_collision_preserved(|collision, unrelated| {
        std::os::unix::fs::symlink(unrelated, collision).unwrap();
    });
}
