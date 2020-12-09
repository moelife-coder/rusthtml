#[cfg(test)]
mod tests;
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
        for (index, i) in content.char_indices() {
            if i == '<' {
                unparsable_content_push(index, last_splitn, &mut constructed);
                last_splitn = index;
            } else if i == '>' {
                let tag = &content[last_splitn..index];
                if tag.chars().nth(0).unwrap() != '<' {
                    continue;
                }
                let tag = &tag[1..].trim_start();
                constructed.push(if tag.chars().nth(0) == Some('/') {
                    Self::ClosingTag(&tag[1..])
                } else {
                    let parsed = Self::parse_opening_tag_content(tag);
                    Self::OpeningTag(parsed.0, parsed.1)
                });
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
