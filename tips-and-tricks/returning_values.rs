struct Foo {
    bar: String,
}

struct Baz {
    biz: String,
}

impl Foo {
    // Bad: Instead of this ...
    fn get_bar(&self) -> &str {
        return &self.bar;
    }
}

impl Baz {
    // Good: Rust will automatically return the value
    fn get_biz(&self) -> &str {
        &self.biz
    }
}

// Particularly useful for closures ...

fn bad_return_for_words_length() {
    let words = vec!["hello", "world"];

    // Bad
    let words_len: Vec<_> = words.into_iter().map(|w| {
        return w.len();
    }).collect();

    println!("A bad return for words length: {:?}", words_len);
}

fn good_return_for_words_length() {
    let words = vec!["hello", "world"];

    // Good
    let words_len: Vec<_> = words.into_iter().map(|w| w.len()).collect();

    println!("A good return for words length: {:?}", words_len);
}

fn main() {
    let foo = Foo {
        bar: "FooBar".to_string()
    };

    let baz = Baz {
        biz: "BazBiz".to_string()
    };

    println!("Foo's bar: {}", foo.get_bar());
    println!("Baz's biz: {}", baz.get_biz());

    bad_return_for_words_length();
    good_return_for_words_length();
}
