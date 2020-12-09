#[cfg(test)]
mod tests;

pub fn tag_optimize<'a>(mut content: Vec<HtmlTag<'a>>) -> Vec<HtmlTag<'a>> {
    let mut offset = 0;
    // There should be a better way to do this...
    // Despreated.
    let _ = |x| match x {
        HtmlTag::OpeningTag(i, j) => {
            let mut a = j
                .iter()
                .map(|x| {
                    if let Some(i) = x.1 {
                        format!(" {}={}", x.0, i)
                    } else {
                        format!(" {}", x.0)
                    }
                })
                .fold(format!("<{}", i), |a, b| {
                    let mut a = a;
                    a.push_str(&b);
                    a
                });
            a.push('>');
            a
        }
        HtmlTag::ClosingTag(i) => format!("</{}>", i),
        HtmlTag::Unparsable(i) => i.to_string(),
    };
    // TODO: implement `template`
    // TODO: implement `head`, `body` omition
    // This is only a subset of the full rule. More rules should be added to make it complete.
    for i in 0..content.len() {
        if let HtmlTag::OpeningTag(name, _) = content[i + offset] {
            match name {
                "area" | "base" | "br" | "col" | "embed" | "hr" | "img" | "input" | "link"
                | "meta" | "param" | "source" | "track" | "wbr" => {
                    content.insert(i + offset + 1, HtmlTag::ClosingTag(name));
                    offset += 1;
                }
                "li" | "dd" | "dt" | "rt" | "rp" | "optgroup" | "tr" | "td" | "th"=> {
                    if let HtmlTag::OpeningTag(name_c, _) = content[i + offset + 1] {
                        if name_c == name {
                            content.insert(i + offset + 1, HtmlTag::ClosingTag(name));
                            offset += 1;
                        }
                    }
                }
                "p" => {
                    // TODO: "if there is no more content in the parent element and the parent
                    // element is an HTML element that is not an a, audio, del, ins, map, noscript,
                    // or video element, or an autonomous custom element."
                    if let HtmlTag::OpeningTag(name_c, _) = content[i + offset + 1] {
                        match name_c {
                            "address" | "article" | "aside" | "blockquote" | "details" | "div"
                            | "dl" | "fieldset" | "figcaption" | "figure" | "footer" | "form"
                            | "h1" | "h2" | "h3" | "h4" | "h5" | "h6" | "header" | "hgroup"
                            | "hr" | "main" | "menu" | "nav" | "ol" | "p" | "pre" | "section"
                            | "table" | "ul" => {
                                content.insert(i + offset + 1, HtmlTag::ClosingTag("p"));
                                offset += 1;
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }
    }

    content
}

#[derive(PartialEq, Debug)]
pub enum ElementTagState {
    OnlyStartTag,
    OnlyEndTag,
    BothTag,
}
#[derive(PartialEq, Debug)]
pub enum ElementContent<'a> {
    HtmlElement(Box<HtmlElement<'a>>),
    LiteralContent(&'a str),
}
#[derive(PartialEq, Debug)]
pub struct HtmlElement<'a> {
    pub name: &'a str,
    pub attributes: Vec<(&'a str, Option<&'a str>)>,
    pub tag_state: ElementTagState,
    pub content: Vec<ElementContent<'a>>,
}
impl<'a> ElementContent<'a> {
    pub fn parse(content: Vec<HtmlTag<'a>>) -> Result<Vec<Self>, ()> {
        let mut constructed = Vec::new();
        for i in content {
            match i {
                HtmlTag::OpeningTag(i, j) => {
                    constructed.push(Self::HtmlElement(Box::new(HtmlElement {
                        name: i,
                        attributes: j,
                        tag_state: ElementTagState::OnlyStartTag,
                        content: Vec::new(),
                    })))
                }
                HtmlTag::ClosingTag(i) => {
                    let mut tag_content = Vec::new();
                    while constructed.len() != 0 {
                        if let Self::HtmlElement(k) = &constructed[constructed.len() - 1] {
                            if k.name == i {
                                break;
                            }
                        }
                        tag_content.push(constructed.remove(constructed.len() - 1));
                    }
                    if constructed.len() == 0 {
                        return Err(());
                    }
                    let mut last_ref = if let Some(i) = constructed.last_mut() {
                        if let Self::HtmlElement(i) = i {
                            i
                        } else {
                            unsafe { core::hint::unreachable_unchecked() }
                        }
                    } else {
                        unsafe { core::hint::unreachable_unchecked() }
                    };
                    tag_content.reverse();
                    last_ref.content.append(&mut tag_content);
                    last_ref.tag_state = ElementTagState::BothTag;
                }
                HtmlTag::Unparsable(i) => constructed.push(Self::LiteralContent(i)),
            }
        }
        Ok(constructed)
    }
}

#[derive(PartialEq, Debug)]
pub enum HtmlTag<'a> {
    OpeningTag(&'a str, Vec<(&'a str, Option<&'a str>)>),
    ClosingTag(&'a str),
    Unparsable(&'a str),
}
impl<'a> HtmlTag<'a> {
    pub fn parse(content: &'a str) -> Vec<Self> {
        let mut last_splitn = 0;
        let mut constructed = Vec::new();
        let unparsable_content_push = |index, last_splitn, constructed: &mut Vec<_>| {
            if last_splitn != 0 && !content[last_splitn + 1..index].trim().is_empty() {
                constructed.push(Self::Unparsable(&content[last_splitn + 1..index]))
            }
        };
        let mut ignore_parsing = None;
        for (index, i) in content.char_indices() {
            if i == '<' {
                if ignore_parsing.is_none() {
                unparsable_content_push(index, last_splitn, &mut constructed);
                }
                last_splitn = index;
            } else if i == '>' {
                let tag = &content[last_splitn..index];
                if tag.chars().nth(0).unwrap() != '<' {
                    continue;
                }
                let tag = &tag[1..].trim_start();
                let constru = if tag.chars().nth(0) == Some('/') {
                    if let Some((i, j)) = ignore_parsing {
                        if i == &tag[1..] {
                            ignore_parsing = None;
                            constructed.push(HtmlTag::Unparsable(&content[j..last_splitn]));
                        } else {
                            continue
                        }
                    }
                    Self::ClosingTag(&tag[1..])
                } else {
                    if ignore_parsing.is_some() {
                        continue
                    }
                    let parsed = Self::parse_opening_tag_content(tag);
                    if (parsed.0 == "script" ) | (parsed.0 == "style") | (parsed.0 == "textarea") | (parsed.0 == "title") {
                        ignore_parsing = Some((parsed.0, index + 1));
                    }
                    Self::OpeningTag(parsed.0, parsed.1)
                };
                constructed.push(constru);
                last_splitn = index;
            }
        }
        constructed
    }
    fn parse_opening_tag_content(content: &'a str) -> (&'a str, Vec<(&'a str, Option<&'a str>)>) {
        let content = content.trim();
        #[derive(PartialEq)]
        enum QuoteStatus {
            NoQuote,
            SingleQuote,
            DoubleQuote,
        };
        let mut current_quotation = QuoteStatus::NoQuote;
        let mut splitted_content = Vec::new();
        let mut space_position = 0;
        let mut is_empty = true;
        let length = content.chars().count();
        for (index, i) in content.char_indices() {
            if i == ' ' && current_quotation == QuoteStatus::NoQuote && !is_empty {
                // This is appropriate split position
                if space_position != 0 {
                    space_position += 1;
                }
                //println!("{} {}", space_position, index);
                splitted_content.push(&content[space_position..index]);
                is_empty = true;
                space_position = index;
            } else if index + 1 == length {
                splitted_content.push(&content[space_position..].trim_start());
                space_position = index + 1;
            } else if (i == '"') | (i == '\'') {
                current_quotation = match current_quotation {
                    QuoteStatus::NoQuote => {
                        if i == '"' {
                            QuoteStatus::DoubleQuote
                        } else {
                            QuoteStatus::SingleQuote
                        }
                    }
                    QuoteStatus::DoubleQuote | QuoteStatus::SingleQuote => QuoteStatus::NoQuote,
                };
            }
            if i != ' ' {
                is_empty = false;
            }
        }
        if splitted_content.len() == 0 {
            unreachable!()
        }
        let name = splitted_content.remove(0);
        let splitted_content = splitted_content
            .iter_mut()
            .map(|x| {
                let equal_sign = x.rfind('=');
                match equal_sign {
                    Some(i) => (
                        &x[..i],
                        Some(x[i + 1..].trim_matches(|c| (c == '"') | (c == '\''))),
                    ),
                    None => (&x[..], None),
                }
            })
            .collect();
        (name, splitted_content)
    }
}
