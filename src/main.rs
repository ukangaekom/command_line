use clap::{Arg, App};

fn main() {
    
    let matches = App::new("AI StaticSol")
        .version("1.0")
        .author("Zealynx Sec")
        .about("An AI Powered Auditing Static Analyzer Command Line Demo!")
        .arg(Arg::new("framework")
            .long("framework")
            .value_name("project_type")
            // .about("Which type of solana project are you auditing?")
            .takes_value(true),
        )
        .get_matches();

        // TODO: select frameworks (options: [native_rust, anchor,])



        if let Some(i) = matches.value_of("framework"){

            match i {
                "anchor" => println!("Traversing anchor project"),
                "native_rust" => println!("Traversing native rust projects"),

                _ => println!("The option {} isn't supported by the commandline tool",i),
            }
            
        }

}
