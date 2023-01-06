use gtest::{Program, System};

#[test]
fn it_works() {
    let system = System::new();
    system.init_logger();

    let program = Program::current(&system);

    fn init_program(prog: &Program) {
        prog.send_bytes(0, "init");
    }
    init_program(&program);

    let res = program.send(42, Action::Flip);
    assert_eq!(res.main_failed(), false);

    let res = program.send(69, Action::Flip);
    assert_eq!(res.main_failed(), false);

    let res = program.send(1337, Action::Flip);
    assert_eq!(res.main_failed(), false);
}
