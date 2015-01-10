#![feature(phase)]
#![feature(globs)]
#[phase(link, plugin)] extern crate log;
#[phase(link, plugin)] extern crate peg_syntax_ext;
#[phase(link, plugin)] extern crate static_templater;

extern crate time;

#[templater_from_file(path="data/test.rs.html")]
mod example_templater {
    use time::Tm;

    type TimeType = Tm;
}


fn main() {
    use std::os;
    use time::now;

    let username = match os::args().as_slice() {
        [_, ref username] => username.clone(),
        _ => "%username%".to_string(),
    };

    print!("{}", example_templater::render(example_templater::Args {
        user: username,
        time: now(),
    }));
}
