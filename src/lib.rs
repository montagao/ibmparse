use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub tablename: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments, expected 3");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            tablename: query,
            filename,
            case_sensitive,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents =
        fs::read_to_string(config.filename).expect("Something went wrong reading the file");

    let mut results = find_table(&contents);

    results.sort_by(|a, b| a.order.parse::<i32>().unwrap().cmp( &b.order.parse::<i32>().unwrap()));
    let p_key = &results[0].variable;

    println!("DROP TABLE IF EXISTS {};" , config.tablename); 
    println!("CREATE TABLE {} (", config.tablename);

    for row in &results {
        println!("\t{}", row.string());
    }
    println!(")");
    println!("SORTKEY({});", p_key);

    Ok(())
}

pub fn find_table<'a>(contents: &'a str) -> Vec<Row> {
    let mut results = Vec::new();
    let mut found_table = false;

    let mut split_0_len = 0;

    for line in contents.lines() {
        // println!("found_table:{}", found_table);
        let split_line = line.split_whitespace();

        if let Some(word) = split_line.clone().find(|&s| s == "Variable" || s == "#") {
            found_table = true;
            continue;
        } 

        if split_line.clone().count() == 0 {
            split_0_len += 1;
        } else {
            split_0_len = 0;
        }

        if split_0_len > 10 {
            return results;
        }

        // println!("split {:?}", split_line.clone().collect::<Vec<&str>>());
        // println!("len of split {}", split_line.clone().count());

        if found_table && split_line.clone().count() > 4 
            && split_line.clone().count() < 10 {
            // println!("Parsing table row...");
            // println!("{}", line);
            let row = match self::Row::new(split_line.collect()) {
                Ok(r) => results.push(r),
                Err(e) => continue
            };
        }
    }
    results
}

enum VarType {
    Num,
    Char,
}

pub struct Row {
    order: String,
    variable: String,
    vartype: String,
    length: String,
}

impl Row {
    pub fn new(fields: Vec<&str>) -> Result<Row, &str> {
        if fields.len() < 4 {
            return Err("Not enough fields, expected 4");
        }
        let order = fields[0].clone().to_lowercase();
        let variable = fields[1].clone().to_lowercase();
        let vartype = match fields[2].clone().to_lowercase().as_str() {
            "char" => "varchar",
            "num" => "int",
            _ => "bad"
        };
        if vartype == "bad" {
            return Err("bad fields");
        }

        let length = fields[3].clone().to_lowercase();
        // println!("{:?}", fields);

        Ok(Row {
            order: String::from(order),
            variable: String::from(variable),
            vartype: String::from(vartype),
            length: String::from(length),
        })
    }

    pub fn string(&self) -> String {
        match self.vartype.as_str() {
            "varchar" => format!("{} {}({}),", self.variable, self.vartype, self.length),
            _ => format!("{} {},", self.variable, self.vartype),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "#    Variable            Type    Length    Format          Label
        \
        0 ADMDATE Num   4
        1 AGE      Char  6
safe, fast, productive.
Pick three.";

        // assert_eq!(vec!["admdate int", "age varchar(6)"], get_sql(contents));
    }
}
