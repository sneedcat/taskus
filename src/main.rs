use taskus::gen_opts;
use taskus::SubCommand::*;
fn main() {
    let opts = gen_opts();

    match opts.subcmd {
        Add(a) => taskus::add::add(a),
        Init(i) => taskus::init::init(i),
        List(l) => taskus::list::list(l),
        Delete(d) => taskus::delete::delete(d),
        Complete(c) => taskus::complete::complete(c),
    }
}
