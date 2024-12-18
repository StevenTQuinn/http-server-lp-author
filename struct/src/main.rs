#[derive(Debug)]
#[derive(PartialEq)]
enum Lang {
  English,
  Spanish,
  Chinese,
  Texan,
  Italian,
  Japanese
}

struct Greeting {
    message: String,
    lang: Lang,
}

trait GreetTrait {
  fn greet(&self) -> String;
}

impl GreetTrait for Greeting {
  fn greet(&self) -> String {
      format!("Greeting in {:?} is \"{}\"", self.lang, self.message)
  }
}

fn main() {
  let mut v :Vec<Greeting> = Vec::new();

  let g : Greeting = Greeting { lang: Lang::English, message: String::from("Hello WasmEdge!") };
  v.push(g);
  let g : Greeting = Greeting { lang: Lang::Spanish, message: String::from("Hola WasmEdge!") };
  v.push(g);
  let g : Greeting = Greeting { lang: Lang::Texan, message: String::from("Howdy WasmEdge!") };
  v.push(g);
  let g : Greeting = Greeting { lang: Lang::Chinese, message: String::from("WasmEdge 你好!") };
  v.push(g);
  let g : Greeting = Greeting { lang: Lang::Italian, message: String::from("Ciao WasmEdge!") };
  v.push(g);
  let g : Greeting = Greeting { lang: Lang::Japanese, message: String::from("こんにちは WasmEdge!") };
  v.push(g);

  println!("Available greetings:");
  for e in &v {
    println!("{:?} {}", e.lang, e.message);
  }

  println!();
  let query_lang = Lang::Japanese;
  for e in &v {
    if e.lang == query_lang {
      println!("{}", e.greet());
      continue;
    }
  }
}
