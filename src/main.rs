// Ceci est un commentaire de base en Rust
/* Ceci est un commentaire multi-ligne de base en Rust
 * Je
 * Suis
 * Libre
 * !
 */
fn main() {
  // Pas besoin de convertir en string pour utiliser le println! avec un format
  let x = format!("Hello from {}!", "Texas").to_string();
  let y = format!("Hello, {}!", "World");

  // println!("{} ---------  {}", x, y);

  // println!("{3} {2} {0} {1}", "Un", "Deux", "Trois", "Quatre");

  // println!("{name} {age}", name="Ousmane", age=10);
  
  println!("Hello Rusty");
  print!("Hello Rust");
  println("Hello Ousmane");
}
