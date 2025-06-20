use std::collections::HashMap;

/// The root element of a MathML document
pub struct Math {
    pub attributes: HashMap<String, String>,
    pub content: Element,
}

pub struct Mrow {
    pub terms: Vec<Element>,
}
pub struct Mi {
    pub identifier: String,
}
pub struct Mn {
    pub number: String,
}
pub struct Mo {
    pub operator: String,
}
pub struct Msub {
    pub base: Box<Element>,
    pub subscript: Box<Element>,
}
pub struct Msup {
    pub base: Box<Element>,
    pub superscript: Box<Element>,
}
pub struct Mfrac {
    pub numerator: Box<Element>,
    pub denominator: Box<Element>,
}
pub struct Mtext {
    pub text: String,
}
// Mspace,
// Ms,
pub struct Msqrt {
    pub term: Box<Element>,
}
pub struct Mroot {
    pub base: Box<Element>,
    pub index: Option<Box<Element>>,
}
/// Types of presentation elements
pub enum Element {
    Mrow(Mrow),
    Mi(Mi),
    Mn(Mn),
    Mo(Mo),
    Msub(Msub),
    Msup(Msup),
    Mfrac(Mfrac),
    Mtext(Mtext),
    // Mspace,
    // Ms,
    // Msqrt(Msqrt),
    Mroot(Mroot),
    // Mstyle,
    // Merror,
    // Mpadded,
    // Mphantom,
    // Mfenced,
    // Menclose,
    // Msubsup,
    // Munder,
    // Mover,
    // Munderover,
    // Mmultiscripts,
    // Mtable,
    // Mtr,
    // Mtd,
    // Maligngroup,
    // Malignmark,
    // Mglyph,
    // Mlongdiv,
}

pub fn mrow(terms: Vec<Element>) -> Element {
    Element::Mrow(Mrow { terms })
}
pub fn mi(identifier: &str) -> Element {
    let identifier = identifier.into();
    Element::Mi(Mi { identifier })
}
pub fn mo(operator: &str) -> Element {
    let operator = operator.into();
    Element::Mo(Mo { operator })
}
pub fn mn(number: &str) -> Element {
    let number = number.into();
    Element::Mn(Mn { number })
}
pub fn msup(base: Element, superscript: Element) -> Element {
    let base = Box::new(base);
    let superscript = Box::new(superscript);
    Element::Msup(Msup { base, superscript })
}
pub fn msub(base: Element, subscript: Element) -> Element {
    let base = base.into();
    let subscript = subscript.into();
    Element::Msub(Msub { base, subscript })
}
pub fn mfrac(numerator: Element, denominator: Element) -> Element {
    let numerator = numerator.into();
    let denominator = denominator.into();
    Element::Mfrac(Mfrac {
        numerator,
        denominator,
    })
}
pub fn mtext(text: &str) -> Element {
    let text = text.into();
    Element::Mtext(Mtext { text })
}
// pub fn msqrt(term: Element) -> Element {
//     let term = term.into();
//     Element::Msqrt(Msqrt { term })
// }
pub fn mroot(base: Element, index: Option<Element>) -> Element {
    let base = base.into();
    let index = index.map(|i| i.into());
    Element::Mroot(Mroot { base, index })
}
