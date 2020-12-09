use crate::*;
#[test]
fn tag_works() {
    let parsed = HtmlTag::parse(r#"<p class=abc def="ghi jkl" mno>Hello World!</p>"#);
    assert_eq!(
        parsed,
        vec![
            HtmlTag::OpeningTag(
                "p",
                vec![
                    ("class", Some("abc")),
                    ("def", Some("ghi jkl")),
                    ("mno", None)
                ]
            ),
            HtmlTag::Unparsable("Hello World!"),
            HtmlTag::ClosingTag("p")
        ]
    )
}
#[test]
fn element_works() {
    let parsed = ElementContent::parse(tag_optimize(HtmlTag::parse(
        r#"<p class=abc>Hello!<img src="abc" def></p><title>This is a test</title>"#,
    )));
    assert_eq!(
        parsed,
        Ok(vec![
            ElementContent::HtmlElement(Box::new(HtmlElement {
                name: "p",
                attributes: vec![("class", Some("abc"))],
                tag_state: ElementTagState::BothTag,
                content: vec![
                    ElementContent::LiteralContent("Hello!"),
                    ElementContent::HtmlElement(Box::new(HtmlElement {
                        name: "img",
                        attributes: vec![("src", Some("abc")), ("def", None)],
                        tag_state: ElementTagState::BothTag,
                        content: Vec::new()
                    }))
                ]
            })),
            ElementContent::HtmlElement(Box::new(HtmlElement {
                name: "title",
                attributes: Vec::new(),
                tag_state: ElementTagState::BothTag,
                content: vec![ElementContent::LiteralContent("This is a test")]
            }))
        ])
    );
}
#[test]
fn almost_all() {
    let source = include_str!("../benches/example.html");
    ElementContent::parse(tag_optimize(HtmlTag::parse(source))).unwrap();
}
#[test]
fn unicode_characters() {
    let parsed = ElementContent::parse(tag_optimize(HtmlTag::parse(
        r#"<p class=❤>Hello!<img src="abc"></p><title>Löwe 老虎 Léopard</title>"#,
    )));
    assert_eq!(
        parsed,
        Ok(vec![
            ElementContent::HtmlElement(Box::new(HtmlElement {
                name: "p",
                attributes: vec![("class", Some("❤"))],
                tag_state: ElementTagState::BothTag,
                content: vec![
                    ElementContent::LiteralContent("Hello!"),
                    ElementContent::HtmlElement(Box::new(HtmlElement {
                        name: "img",
                        attributes: vec![("src", Some("abc"))],
                        tag_state: ElementTagState::BothTag,
                        content: Vec::new()
                    }))
                ]
            })),
            ElementContent::HtmlElement(Box::new(HtmlElement {
                name: "title",
                attributes: Vec::new(),
                tag_state: ElementTagState::BothTag,
                content: vec![ElementContent::LiteralContent("Löwe 老虎 Léopard")]
            }))
        ])
    );
}
