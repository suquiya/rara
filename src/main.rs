#![warn(missing_docs)]
//! Rara is a password generator. This is also an sample project using [combu](https://github.com/suquiya/combu).
//! For more information, see the [repository](https://github.com/suquiya/rara).

use rara::cmd;
fn main() {
    let _r = cmd::new().run_with_auto_arg_collect();
}
