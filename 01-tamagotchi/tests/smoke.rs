use gtest::{Log, Program, System};
use tmg1_io::{TmgAction, TmgEvent};

#[test]
fn smoke_test() {
    let sys = System::new();
    sys.init_logger();
    let program = Program::current(&sys);

    // init
    let birth = sys.block_timestamp();
    let res = program.send(2, String::from("babygear"));
    assert!(!res.main_failed());

    // get name
    let res = program.send(4, TmgAction::Name);
    let expected_res: Log = Log::success_builder(gtest::SuccessReplyReason::Manual)
        .source(1)
        .dest(4)
        .payload(TmgEvent::Name(String::from("babygear")));
    assert!(res.contains(&expected_res));

    // get age
    let res = program.send(2, TmgAction::Age);
    let expected_res = Log::success_builder(gtest::SuccessReplyReason::Manual)
        .source(1)
        .dest(2)
        .payload(TmgEvent::Age(0));
    assert!(res.contains(&expected_res));

    // grow
    sys.spend_blocks(100);
    let now = sys.block_timestamp();

    // get age
    let res = program.send(2, TmgAction::Age);
    let expected_res = Log::success_builder(gtest::SuccessReplyReason::Manual)
        .source(1)
        .dest(2)
        .payload(TmgEvent::Age(now - birth));

    assert!(res.contains(&expected_res));
}
