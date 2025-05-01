use csv::{ReaderBuilder, StringRecord};
use std::collections::HashMap;
use std::{fs, vec};

const FILE_PATH: &str = "history.csv";
const FIRST_TAG: &str = "INICIO";

struct History {
  data_type: String,
  tag: String,
  text: String,
  life: i32,
  options: Vec<History>,
}

impl History {
  fn new(row: StringRecord) -> History {
    let life = row.get(3).unwrap().trim();
    let life: i32 = life.parse().unwrap_or(0);

    return History {
      data_type: row.get(0).unwrap().trim().to_string(),
      tag: row.get(1).unwrap().trim().to_string(),
      text: row.get(2).unwrap().trim().to_string(),
      life,
      options: vec![],
    };
  }
}

fn main() {
  let mut life = 100;
  let mut current_tag = FIRST_TAG;

  let mut last_record: String = "".to_string();

  let mut data_history: HashMap<String, History> = HashMap::new();

  let content = fs::read_to_string(FILE_PATH).expect("Failed to read file");
  let mut reader = ReaderBuilder::new()
    .delimiter(b';')
    .from_reader(content.as_bytes());

  for result in reader.records() {
    let result = result.unwrap();
    let data = History::new(result);

    if data.data_type == "SITUACION" {
      let record_tag = data.tag.clone();
      data_history.insert(record_tag.clone(), data);
      last_record = record_tag;
    } else if data.data_type == "OPCION" {
      if let Some(data_h) = data_history.get_mut(&last_record) {
        (*data_h).options.push(data);
      }
    }
  }

  loop {
    println!("You have {} life", life);

    if let Some(data_h) = data_history.get(current_tag) {
      println!("{}", data_h.text);

      for (i, option) in data_h.options.iter().enumerate() {
        println!("[{}]: {}", i, option.text);
      }

      let mut input = String::new();
      std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
      let input: usize = input.trim().parse().unwrap_or(0);

      if let Some(option) = data_h.options.get(input) {
        current_tag = &option.tag;
      } else {
        println!("Invalid option");
      }

      life += data_h.life;
      print!("Life: {}", life);
    } else {
      println!("Game Over");
      break;
    }

    if life <= 0 {
      println!("You died");
      break;
    }
  }
}
