use core::panic;
use std::collections::HashMap;

use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "nsconf.pest"]
struct NSConfParser;

pub fn parse(data: &str) -> Result<HashMap<String, HashMap<String, String>>, Box<dyn std::error::Error>> {
    let parse = NSConfParser::parse(Rule::file, data)?
        .next()
        .unwrap();

    let mut properties: HashMap<String, HashMap<String, String>> = HashMap::new();

    for line in parse.into_inner() {
        match line.as_rule() {
            Rule::entry => {
                let mut entry = line.into_inner();
                let entry_name = entry.next().unwrap().as_str();
                let property_list = entry.next().unwrap();
                let mut entry_properties = HashMap::new();
                for property in property_list.into_inner() {
                    let mut pair = property.into_inner();
                    let prop_name = pair.next().unwrap().as_str();
                    let prop_value = pair.next().unwrap().as_str();
                    entry_properties.insert(prop_name.to_owned(), prop_value.to_owned());
                }
                properties.insert(entry_name.to_owned(), entry_properties);
            }
            _ => unreachable!(),
        }
    }
    
    Ok(properties)
}