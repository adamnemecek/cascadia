fn test() -> Result<(), String> {
    if true {
        Err("fdsa".to_owned())?
    }
    Ok(())
}

mod tests {
    #[test]
    fn test_binomial() {
        use num_integer::IterBinomial;

        let mut i = IterBinomial::new(10);

        for e in i {
            println!("{:?}", e);
        }
    }

    #[test]
    fn test_err() {
        let q = super::test();
        println!("{:?}", q);
    }
}
