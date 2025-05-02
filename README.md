# MathML rendering

Exploratory implementation of MathML rendering using [linebender](https://github.com/linebender) ecosystem (in particular tiny_skia and parley).

No parsing of the XML format yet, and when it does, it will only deal with "presentation" MathML nodes (as opposed to "content" nodes).
From some testing, I think [Temml](https://github.com/ronkok/Temml)
only outputs presentation nodes, which could make this fit for the purpose of rendering LaTeX.

## Render examples:

White background on all texts helps check alignments.

| description | rendered png |
| --- | --- |
| $${\beta_\alpha}^2$$ | ![alt](examples/beta_sub_alpha_sup_2.png "{\\beta_{\\alpha}}^{2}") |
| alpha over beta | ![alt](examples/alpha_over_beta.png "{\\frac{\\alpha}{\\beta}}") |
| alpha_n over two | ![alt](examples/half_alpha_n.png "{\\frac{\\alpha}{\\beta}}") |

## Presentation nodes preliminarily implemented:
- [x] `<mi>`
- [x] `<mn>`
- [x] `<msub>`
- [x] `<msup>`
- [ ] `<mi>`
- [ ] `<mo>`
- [x] `<mfrac>`
- [ ] `<mtext>`
- [ ] `<mspace>`
- [ ] `<ms>`
- [ ] `<msqrt>`
- [ ] `<mroot>`
- [ ] `<mstyle>`
- [ ] `<merror>`
- [ ] `<mpadded>`
- [ ] `<mphantom>`
- [ ] `<mfenced>`
- [ ] `<menclose>`
- [ ] `<msubsup>`
- [ ] `<munder>`
- [ ] `<mover>`
- [ ] `<munderover>`
- [ ] `<mmultiscripts>`
- [ ] `<mtable>`
- [ ] `<mtr>`
- [ ] `<mtd>`
- [ ] `<maligngroup>`
- [ ] `<malignmark>`
- [ ] `<mglyph>`
- [ ] `<mlongdiv>`

## Future subtleties to work in:
- stretching operators
- make text italic when its meant to (identifier with one character)
- keep track of where the `+` horizontal line comes to (to keep vertical alignment with fraction line etc)
