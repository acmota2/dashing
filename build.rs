fn main() {
    println!("Embeding templates for use with minijinja");
    minijinja_embed::embed_templates!("./templates");
}
