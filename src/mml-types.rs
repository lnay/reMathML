use std::collections::HashMap;

/// The root element of a MathML document
pub struct Math {
    pub attributes: HashMap<String, String>,
    pub content: Mrow,
}

/// Types of semantic elements
// pub enum SemanticType {
//     Annotation,
//     AnnotationXml
// }

/// Types of presentation elements
pub enum Element {
    Mrow(Mrow),
    Mi(Mi),
    Mn(Mn),
    Mo(Mo),
    Msub(Msub),
    Msup(Msup),
    Mfrac(Mfrac),
    // Mtext,
    // Mspace,
    // Ms,
    // Msqrt,
    // Mroot,
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

pub struct Mrow {
    pub children: Vec<Element>,
}
pub struct Mn {
    pub number: String,
}
pub struct Mi {
    pub identifier: String,
}
pub struct Mo {
    pub operator: String,
}
pub struct Msub {
    pub base: Element,
    pub subscript: Element,
}
pub struct Msup {
    pub base: Element,
    pub superscript: Element,
}
pub struct Mfrac {
    pub numer: Element,
    pub denom: Element,
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
