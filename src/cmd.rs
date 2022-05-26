use combu::{
    action_result, alias, checks, commands, copyright, crate_authors, crate_description,
    crate_name, crate_version, flags, license, ActionResult, Command, Context, Flag,
};

pub fn new_executable() -> Command {
    Command::with_all_field(
        crate_name!().into(),
        Some(action),
        crate_authors!().into(),
        copyright!(from_crate, 2022),
        license!("MIT License",file_path=>"../LICENSE"),
        Some(crate_description!().into()),
        "rara [OPTIONS]".into(),
        flags!(None),
        flags!(help, version, license),
        alias!(),
        crate_version!().into(),
        commands!(),
    )
}

fn action(cmd: Command, ctx: Context) -> action_result!() {
    checks!(cmd, ctx, [error, help, version, license]);
    Ok(ActionResult::Result(cmd, ctx))
}
