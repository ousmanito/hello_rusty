// Hello Rusty



struct Maison {
  nom: String,
  surface: i8,
  composition: Vec<&str>
}

impl Maison {
  fn hello_world(&self) -> String {
    println!("Bonjour et bienvenue dans la {:?}, la surface est de {:?} mètres carrées. {:#?}",
             self.nom,
             self.surface,
             self.composition);
    return String::from("Ousmane")
  }
}


fn main() {
  // Pas besoin de convertir en string pour utiliser le println! avec un format
  // let x = format!("Hello from {}!", "Texas").to_string();
  // let y = format!("Hello, {}!", "World");

  // println!("{} ---------  {}", x, y);

  // println!("{3} {2} {0} {1}", "Un", "Deux", "Trois", "Quatre");

  // println!("{name} {age}", name="Ousmane", age=10);
  let house = Maison {
      nom: String::from("nature"),
      surface: 777,
      composition: vec!["Le chat", "L'escargot", "La pie", "Le corbeau", "Les abeilles"]
  };
  println!("{}", house.hello_world());
    
}
