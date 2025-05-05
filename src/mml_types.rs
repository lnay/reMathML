use std::collections::HashMap;

use crate::render::Render;

/// The root element of a MathML document
// pub struct Math {
//     pub attributes: HashMap<String, String>,
//     pub content: Element,
// }

/// Types of semantic elements
// pub enum SemanticType {
//     Annotation,
//     AnnotationXml
// }

/// Types of presentation elements
// pub enum Element {
//     Mrow(Mrow),
//     Mi(Mi),
//     Mn(Mn),
//     Mo(Mo),
//     Msub(Msub),
//     Msup(Msup),
//     Mfrac(Mfrac),
//     // Mtext,
//     // Mspace,
//     // Ms,
//     // Msqrt,
//     // Mroot,
//     // Mstyle,
//     // Merror,
//     // Mpadded,
//     // Mphantom,
//     // Mfenced,
//     // Menclose,
//     // Msubsup,
//     // Munder,
//     // Mover,
//     // Munderover,
//     // Mmultiscripts,
//     // Mtable,
//     // Mtr,
//     // Mtd,
//     // Maligngroup,
//     // Malignmark,
//     // Mglyph,
//     // Mlongdiv,
// }

pub struct Mrow<'a> {
    pub children: Vec<&'a dyn Render>,
}
pub struct Mn {
    pub number: String,
}
pub fn mn(number: &str) -> Mn {
    let number = number.into();
    Mn { number }
}
pub struct Mi {
    pub identifier: String,
}
pub fn mi(identifier: &str) -> Mi {
    let identifier = identifier.into();
    Mi { identifier }
}
pub struct Mo {
    pub operator: String,
}
pub fn mo(operator: &str) -> Mo {
    let operator = operator.into();
    Mo { operator }
}
pub struct Msub<'a> {
    pub base: &'a dyn Render,
    pub subscript: &'a dyn Render,
}
pub fn msub<'a>(base: &'a dyn Render, subscript: &'a dyn Render) -> Msub<'a> {
    Msub { base, subscript }
}
pub struct Msup<'a> {
    pub base: &'a dyn Render,
    pub superscript: &'a dyn Render,
}
pub fn msup<'a>(base: &'a dyn Render, superscript: &'a dyn Render) -> Msup<'a> {
    Msup { base, superscript }
}
pub struct Mfrac<'a> {
    pub numer: &'a dyn Render,
    pub denom: &'a dyn Render,
}
pub fn mfrac<'a>(numer: &'a dyn Render, denom: &'a dyn Render) -> Mfrac<'a> {
    Mfrac { numer, denom }
}

// /// Represents content markup elements
// pub struct ContentNode {
//     pub node_type: ContentType,
//     pub attributes: HashMap<String, String>,
//     pub children: Vec<Element>,
// }

// /// Types of content elements
// pub enum ContentType {
//     Apply,
//     Bind,
//     Ci,
//     Cn,
//     Csymbol,
//     Share,
//     Declare,
//     Lambda,
//     Piecewise,
//     Piece,
//     Otherwise,
//     Sep,
// }
