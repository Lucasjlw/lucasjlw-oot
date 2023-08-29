mod server;
use lucasjlw_oot::run;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if &args[1] == "serve" {
        server::serve("src/base.html");
    } else {
        run();
    }
}