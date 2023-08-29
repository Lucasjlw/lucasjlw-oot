mod server;
use lucasjlw_oot::run;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if &args[1] == "serve" {
        println!("this is running");
        server::serve("base.html");
    } else {
        run();
    }
}