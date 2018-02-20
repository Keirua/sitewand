#[macro_use] extern crate tera;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate structopt;
#[macro_use] extern crate serde_derive;

extern crate serde_json;

use std::fs::File;
use std::path::PathBuf;
use std::io::prelude::*;
use serde_json::{Value, to_value, from_value};

use std::collections::HashMap;
use structopt::StructOpt;
use tera::{Tera, Context, Result, GlobalFn};

#[derive(StructOpt, Debug)]
#[structopt(name = "sitewand", about = "a dynamic static  site generator.")]
struct Opt {
    #[structopt(short = "d", long = "debug", help = "debug mode")]
    debug: bool,
    #[structopt(help = "Input configuration file", parse(from_os_str))]
    config_file: PathBuf,
    #[structopt(help = "Output directory", parse(from_os_str), default_value = "www/")]
    output_directory: PathBuf,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
struct JsonConfig {
    pages: HashMap<String, Page>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Page {
    template: String,
    url: String
}

fn make_url_for(urls: HashMap<String, Page>) -> GlobalFn {
    // args is a HashMap<String, Value>
    Box::new(move |args| -> Result<Value> {
        match args.get("name") {
            Some(val) => match from_value::<String>(val.clone()) {
                Ok(v) =>  {
                    match urls.get(&v) {
                        Some(page) => { 
                            Ok(to_value(&page.url).unwrap())
                        },
                        _ => Err(format!("route {} is unknown!", v).into()),
                    }
                },
                Err(_) => Err("Oops!".into()),
            },
            None => Err("parameter [name] was not provided !".into()),
        }
    })
}

fn main() {
    let opt = Opt::from_args();

    let mut file = File::open(opt.config_file).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let json_config: JsonConfig = serde_json::from_str(&contents).unwrap();

    let mut tera = compile_templates!("templates/**/*");
    tera.autoescape_on(vec!["j2"]);
    tera.register_global_function("path", make_url_for(json_config.pages.clone()));

    let context = Context::new();

    for (_, page) in json_config.pages.iter() {
        match tera.render(&page.template, &context) {
            Ok(s) => {
                let mut path = opt.output_directory.clone();
                path.push(page.url.clone());
                let mut file = File::create(path).unwrap();
                let _ = file.write(s.as_bytes());
            },
            Err(e) => {
                println!("Error: {}", e);
                for e in e.iter().skip(1) {
                    println!("Reason: {}", e);
                }
            }
        };
    }
}