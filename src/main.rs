use combu::ActionResult;
use rara::cmd;

fn main() {
    let _r = cmd::new().run_with_auto_arg_collect();
    if let Ok(ActionResult::Result(cmd, ctx)) = _r {
        cmd::parse_ctx_and_run(cmd, ctx);
    }
}
