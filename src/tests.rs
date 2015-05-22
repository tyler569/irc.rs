
test]
fn test_parser() {
    let test1 = "PING :Hello World".to_string();

    let output1 = parser::Line::parse(test1);

    assert_eq!(output1.raw, test1);
    assert_eq!(output1.command, "PING");
    assert_eq!(output1.params, vec!["Hello World"]);
}
