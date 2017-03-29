extern crate tfidf;

#[cfg(test)]
mod tests {
    use tfidf::tfidf::TfIdf;
    use tfidf::tfidf::Term;

    fn setup<'a>() -> TfIdf<'a> {
        let mut tfidf = TfIdf::new();

        tfidf.add("China has a strong economy that is growing at a rapid pace. However politically it differs greatly from the US Economy.");
        tfidf.add("At last, China seems serious about confronting an endemic problem: domestic violence and corruption.");
        tfidf.add("Japan's prime minister, Shinzo Abe, is working towards healing the economic turmoil in his own country for his view on the future of his people.");
        tfidf.add("Vladimir Putin is working hard to fix the economy in Russia as the Ruble has tumbled.");
        tfidf.add("What's the future of Abenomics? We asked Shinzo Abe for his views");
        tfidf.add("Obama has eased sanctions on Cuba while accelerating those against the Russian Economy, even as the Ruble's value falls almost daily.");
        tfidf.add("Vladimir Putin is riding a horse while hunting deer. Vladimir Putin always seems so serious about things - even riding horses. Is he crazy?");

        tfidf
    }

    #[test]
    fn test_similarities() {
        let tfidf = setup();

        println!("** China: {:?}", tfidf.similarities(&Term("China")));
        println!("** economy: {:?}", tfidf.similarities(&Term("economy")));
        println!("** his: {:?}", tfidf.similarities(&Term("his")));
        println!("** he: {:?}", tfidf.similarities(&Term("he")));
        println!("** in: {:?}", tfidf.similarities(&Term("in")));
        println!("** Putin: {:?}", tfidf.similarities(&Term("Putin")));
        println!("** Shinzo: {:?}", tfidf.similarities(&Term("Shinzo")));
        println!("** the: {:?}", tfidf.similarities(&Term("the")));

        //assert_eq!(tfidf.tf(&Term("hello"), 0), 1.4771212f32);
    }
}
