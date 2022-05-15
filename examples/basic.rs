use textopia::parsers::tokens;

fn main() {
    let example = "This is an example. He's doing some work with the possessives. I have tried adding parsing of tokens like this! Or did I? This is a full stop. Hello, how's the comma?";
    println!("{:#?}", tokens(example).unwrap());
    // tokens(example).unwrap();
}
