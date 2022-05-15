use textopia::parsers::tokens;
use textopia::replace;

fn main() {
    let example = "This is an example. He's doing some work with the possessives. I have tried adding parsing of tokens like this! Or did I? This is a full stop. Hello, how's the comma? Let's test the replacements too! Mr. Benedict Cumberbatch.";
    let example = example.to_lowercase();

    let result = replace(tokens(&example).unwrap().1);
    println!("{:#?}", result)
}
