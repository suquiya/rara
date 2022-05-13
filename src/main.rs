use combu::{
    action_result, alias, commands, copyright, crate_authors, crate_description, crate_name,
    crate_version, flags, help_flag, license, vector, version_flag, ActionResult, Command, Context,
    Flag,
};
fn main() {
    let _r = cmd().run_with_auto_arg_collect();
}

fn cmd() -> Command {
    Command::with_all_field(
        crate_name!().into(),
        Some(action),
        crate_authors!().into(),
        copyright!(from_crate, 2022),
        license!("MIT License",file_path=>"../LICENSE"),
        Some(crate_description!().into()),
        "rara [OPTIONS]".into(),
        flags!(None),
        flags!([help], [version], [license]),
        alias!(),
        crate_version!().into(),
        commands!(),
    )
}

fn action(cmd: Command, ctx: Context) -> action_result!() {
    Ok(ActionResult::Result(cmd, ctx))
}
