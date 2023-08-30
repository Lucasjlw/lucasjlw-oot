mod server;
use lucasjlw_oot::run;
use pollster;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 && &args[1] == "serve" {
        server::serve("src/base.html");
    } else {
        pollster::block_on(run());
    }
}