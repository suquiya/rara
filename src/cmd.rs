use combu::{
    action_result, alias, checks, commands, copyright, crate_authors, crate_description,
    crate_name, crate_version, done, flags, license, output_help, vector::flag::FlagSearch,
    Command, Context, Flag, FlagValue,
};

use crate::pwgen;

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
                [length]=>[>int?20,="password length option. default: 20",-l,--len]
            ],
            [
                [number]=>[>int?7,="password number option. default: 7",-n,--num]
            ],
            [
                [include]=>[>String,="chooses chars indicate type password include. Types are alphabet(a),number(n),symbol(s) and default(d). You can specify multiple chars(ex: an). default=ans",-i,?"d"]
            ],
            [
                [exclude]=>[
                    >String?"",
                    ="Password will generate without chars that is specidied by this flag",-e
                ]
            ],
            [
                [custom]=>[
                    >String?"",
                    ="Password will generate with only chars that is specidied by this flag. this option is inputted, include option is disabled. default: disable",-c,-m,--cstm
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

pub fn parse_ctx_and_run(cmd: Command, ctx: Context) -> action_result!() {
    let l = {
        let lf = cmd.l_flags.find("length").unwrap();
        match ctx
            .get_flag_value_of(&lf.name, &cmd)
            .unwrap()
            .get_int_unwrap()
        {
            x if x > 0 => x,
            _ => lf.default_value.get_int_unwrap(),
        }
    } as usize;
    let n = {
        let nf = cmd.l_flags.find("number").unwrap();
        match ctx
            .get_flag_value_of(&nf.name, &cmd)
            .unwrap()
            .get_int_unwrap()
        {
            x if x > 0 => x,
            _ => nf.default_value.get_int_unwrap(),
        }
    } as usize;
    let include_str = match ctx.get_flag_value_of("custom", &cmd).unwrap().get_string() {
        x if x.is_empty() => {
            let include = ctx.get_flag_value_of("include", &cmd).unwrap().get_string();
            if include.contains('d') {
                String::from(pwgen::str_list::get_alphabets())
                    + pwgen::str_list::get_numbers()
                    + pwgen::str_list::get_symbols()
            } else {
                let mut str = String::new();
                if include.contains('a') {
                    str = str + pwgen::str_list::get_alphabets();
                }
                if include.contains('n') {
                    str = str + pwgen::str_list::get_numbers();
                }
                if include.contains('s') {
                    str = str + pwgen::str_list::get_symbols();
                }
                str
            }
        }
        x => x,
    };

    let include_chars: Vec<char> = match ctx.get_inputted_local_flag_value_of("exclude") {
        None | Some(FlagValue::None) => include_str.chars().collect(),
        Some(FlagValue::String(val)) if val.is_empty() => include_str.chars().collect(),
        Some(ex) => {
            let ex = ex.get_string();
            include_str.chars().filter(|c| !ex.contains(*c)).collect()
        }
    };

    if include_chars.is_empty() {
        println!("No char can use in password!\r\n");
        output_help!(&cmd, &ctx);
        return done!();
    }

    let password_list = pwgen::pwgen(l, n, &include_chars);

    println!("---generated passwords begin---");
    for password in password_list {
        println!("{}", password);
    }
    println!("---generated passwords end---");

    done!()
}
