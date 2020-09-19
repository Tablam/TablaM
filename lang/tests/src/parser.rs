use tablam_lang::parser::Parser;

#[test]
fn test_collections() {
    let input = "let empty := []";
    let mut parser = Parser::new(input);
    let result = parser.parse();
    assert_eq!(
        result.expect("not getting expression").to_string(),
        String::from("let empty := Vec[it:Any;]")
    );

    let input = "let n := [9; 8; 10]";
    let mut parser = Parser::new(input);
    let result = parser.parse();
    assert_eq!(
        result.expect("not getting expression").to_string(),
        String::from("let n := Vec[it:Int; 9; 8; 10]")
    );

    let input = "let complex := [real:Dec, img:Int; 1d,3; 3d,4; 4d,5;]";
    let mut parser = Parser::new(input);
    let result = parser.parse();
    assert_eq!(
        result.expect("not getting expression").to_string(),
        String::from("let complex := Vec[real:Dec, img:Int; 1, 3; 3, 4; 4, 5]")
    );
}

#[test]
fn test_assignment() {
    let input = "let t := 1";
    let mut parser = Parser::new(input);
    let result = parser.parse();
    assert_eq!(
        result.expect("not getting expression").to_string(),
        String::from("let t := 1")
    );

    let input = "var y = 1d";
    let mut parser = Parser::new(input);
    let result = parser.parse();
    assert_eq!(
        result
            .expect_err("erroneous assignment operator.")
            .to_string(),
        String::from(
            "Syntax error => Unexpected token. It found: =, it was expected: :=. (Line 1 |6..7|)"
        )
    );

    let input = "let t := b";
    let mut parser = Parser::new(input);
    let result = parser.parse();
    assert_eq!(
        result.expect("not getting expression").to_string(),
        String::from("let t := b")
    );

    let input = "let t := b + 1";
    let mut parser = Parser::new(input);
    let result = parser.parse();
    assert_eq!(
        result.expect("not getting expression").to_string(),
        String::from("let t := b + 1")
    );

    let input = "let t := a and b";
    let mut parser = Parser::new(input);
    let result = parser.parse();
    assert_eq!(
        result.expect("not getting expression").to_string(),
        String::from("let t := a and b")
    );

    let input = "let t := a and b or 1 <> 2";
    let mut parser = Parser::new(input);
    let result = parser.parse();
    assert_eq!(
        result.expect("not getting expression").to_string(),
        String::from("let t := a and b or 1 <> 2")
    );

    let input = "let empty := []";
    let mut parser = Parser::new(input);
    let result = parser.parse();
    assert_eq!(
        result.expect("not getting expression").to_string(),
        String::from("let empty := Vec[it:Any;]")
    );

    let input = "let n := [9; 8; 10]";
    let mut parser = Parser::new(input);
    let result = parser.parse();
    assert_eq!(
        result.expect("not getting expression").to_string(),
        String::from("let n := Vec[it:Int; 9; 8; 10]")
    );

    let input = "let complex := [real:Dec, img:Int; 1d,3; 3d,4; 4d,5;]";
    let mut parser = Parser::new(input);
    let result = parser.parse();
    assert_eq!(
        result.expect("not getting expression").to_string(),
        String::from("let complex := Vec[real:Dec, img:Int; 1, 3; 3, 4; 4, 5]")
    );
}

#[test]
fn test_arithmetic() {
    let input = "1+2";
    let mut parser = Parser::new(input);
    let result = parser.parse();
    assert_eq!(
        result.expect("not getting expression").to_string(),
        String::from("1 + 2")
    );

    let input = "1+2-1";
    let mut parser = Parser::new(input);
    let result = parser.parse();
    assert_eq!(
        result.expect("not getting expression").to_string(),
        String::from("1 + 2 - 1")
    );
}

#[test]
fn test_call_function() {
    let input = r#"print("world")"#;
    let mut parser = Parser::new(input);
    let result = parser.parse();
    assert_eq!(
        result.expect("not getting expression").to_string(),
        String::from(r#"print( := 'world')"#)
    );

    let input = r#"print("world")"#;
    let mut parser = Parser::new(input);
    let result = parser.parse();
    assert_eq!(
        result.expect("not getting expression").to_string(),
        String::from(r#"print( := 'world')"#)
    );

    let input = r#"print("world", "hello", 2, 5)"#;
    let mut parser = Parser::new(input);
    let result = parser.parse();
    assert_eq!(
        result.expect("not getting expression").to_string(),
        String::from("print( := 'world',  := 'hello',  := 2,  := 5)")
    );
}

#[test]
fn test_relational_ops() {
    let input = "complex ?select #name";
    let mut parser = Parser::new(input);
    let result = parser.parse();
    assert_eq!(
        result.expect("not getting expression").to_string(),
        String::from("complex ?select #name")
    );

    let input = "complex ?select #name, #ln as #last_name";
    let mut parser = Parser::new(input);
    let result = parser.parse();
    assert_eq!(
        result.expect("not getting expression").to_string(),
        String::from("complex ?select #name, #ln as #last_name")
    );

    let input = "complex ?select #img, #real as #r ?where #1 > 20";
    let mut parser = Parser::new(input);
    let result = parser.parse();
    assert_eq!(
        result.expect("not getting expression").to_string(),
        String::from("complex ?select #img, #real as #r ?where #1 > 20")
    );

    let input = "complex ?deselect #img ?skip 3 ?limit 6 ?distinct";
    let mut parser = Parser::new(input);
    let result = parser.parse();
    assert_eq!(
        result.expect("not getting expression").to_string(),
        String::from("complex ?deselect #img ?skip 3 ?limit 6 ?distinct")
    );
}
