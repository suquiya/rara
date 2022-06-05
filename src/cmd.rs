use combu::{
    action_result, alias, checks, commands, copyright, crate_authors, crate_description,
    crate_name, crate_version, flags, license, result, Command, Context, Flag, FlagValue,
};

pub fn new() -> Command {
    Command::with_all_field(
        crate_name!().into(),
        Some(act),
        crate_authors!().into(),
        copyright!(from_crate, 2022),
        license!("MIT License",file_path=>"../LICENSE"),
        Some(crate_description!().into()),
        "rara [OPTIONS]".into(),
        flags![
            [
                [length]=>[>int?20,="password length option. default: 20",-l]
            ],
            [
                [number]=>[>int?7,="password number option. default: 7",-n]
            ],
            [
                [include]=>[>bool?"default",="chooses string type password include. Types are alphabet,number,symbol and default, that can combinate by '+'. default: alphabet+number+symbol",-i]
            ],
            [
                [exclude]=>[
                    >String?"",
                    ="Password will generate without chars that is specidied by this flag"
                ]
            ],
            [
                [custom]=>[
                    >String?"",
                    ="Password will generate only chars that is specidied by this flag. this option is inputted, include option is disabled. default: disable",-c,-m,--cstm
                ]
            ]
        ],
        flags!(help, version, license),
        alias!(),
        crate_version!().into(),
        commands!(),
    )
}

fn act(cmd: Command, ctx: Context) -> action_result!() {
    checks!(cmd, ctx, [error, help, version, license]);
    parse_ctx_and_run(cmd, ctx)
}

fn parse_ctx_and_run(cmd: Command, ctx: Context) -> action_result!() {
    let l = {
        let FlagValue::Int(a) = ctx.get_flag_value_of("length", &cmd).unwrap();
    };

    result!(cmd, ctx)
}
