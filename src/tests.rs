use gtest::{Program, System};

#[test]
fn it_works() {
    let system = System::new();
    system.init_logger();

    let program = Program::current(&system);

    program.send_bytes(0, "let's goooo!");

    let res = program.send(42, FlipperAction::Flip);
    assert_eq!(res.main_failed(), false);

    let res = program.send(69, FlipperAction::Flip);
    assert_eq!(res.main_failed(), false);

    let res = program.send(1337, FlipperAction::Flip);
    assert_eq!(res.main_failed(), false);
}
