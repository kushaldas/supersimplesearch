#[macro_use]
extern crate rocket;
extern crate chrono;
extern crate serde;
extern crate serde_json;

use chrono::prelude::*;
use rocket::State;
use rocket_dyn_templates::Template;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

#[derive(Debug, Default)]
struct LinksCache {
    links: HashMap<String, DateTime<Local>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Incoming {
    date: String,
    link: String,
    words: Vec<String>,
}

#[get("/")]
fn index() -> Template {
    let context = HashMap::<String, String>::new();
    Template::render("index", context)
}

#[get("/?<q>")]
fn search(q: Option<String>, cache: &State<HashMap<String, LinksCache>>) -> Template {
    match q {
        Some(qr) => {
            let query = qr.to_lowercase().trim().to_string();
            let mut context = HashMap::<String, Vec<(String, String)>>::new();
            let mut links: Vec<(String, DateTime<Local>)> = Vec::new();
            let final_links: Vec<(String, String)> = Vec::new();
            context.insert("links".to_string(), final_links.clone());
            let mut final_links: Vec<(String, String)> = Vec::new();
            if query != "" {
                if cache.contains_key(&query) {
                    let linkitem = cache.get(&query).unwrap();
                    for (k, v) in linkitem.links.iter() {
                        links.push((k.clone(), v.clone()));
                    }
                    // Now first sort the links vector
                    links.sort_by(|a, b| b.1.cmp(&a.1));

                    // let us create a string based vector for the template
                    for link in links.iter() {
                        final_links.push((link.0.clone(), link.1.format("%b %e %Y").to_string()))
                    }
                    context.insert("links".to_string(), final_links);
                }
                return Template::render("results", context);
            } else {
                return Template::render("results", context);
            }
        }
        None => (),
    }
    let context = HashMap::<String, String>::new();
    Template::render("index", context)
}

fn loaddata() -> HashMap<String, LinksCache> {
    let mut cache: HashMap<String, LinksCache> = HashMap::new();
    let mut contents = String::new();
    let mut file = File::open("db.json").unwrap();
    file.read_to_string(&mut contents).unwrap();

    let data: Vec<Incoming> = serde_json::from_str(&contents).unwrap();
    for item in &data {
        for word in &item.words {
            let lc = cache.entry(word.to_string()).or_insert(LinksCache {
                links: HashMap::new(),
            });
            let ctime = item.date.parse::<DateTime<Local>>().unwrap();
            let _ = *lc.links.entry(item.link.to_string()).or_insert(ctime);
        }
    }
    cache
}

#[launch]
fn rocket() -> _ {
    let cache = loaddata();
    rocket::build()
        .manage(cache)
        .mount("/", routes![index])
        .mount("/search", routes![search])
}
