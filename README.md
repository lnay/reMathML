# MathML rendering

Exploratory implementation of MathML rendering using [linebender](https://github.com/linebender) ecosystem (in particular tiny_skia and parley).

No parsing of the XML format yet, and when it does, it will only deal with "presentation" MathML nodes (as opposed to "content" nodes).
From some testing, I don't think [Temml](https://github.com/ronkok/Temml)
outputs any of these nodes, which could make this fit for the purpose of rendering LaTeX.

## Render examples:

| raw MathML | rendered png |
| --- | --- |
| <math><msup><msub><mi>β</mi><mi>α</mi></msub><mn>2</mn></msup></math> | ![alt](examples/beta_sub_alpha_sup_2.png "{\\beta_{\\alpha}}^{2}") |

## Presentation nodes preliminarily implemented:
- [x] `<mi>`
- [x] `<mn>`
- [x] `<msub>`
- [x] `<msup>`
- [ ] `<mi>`
- [ ] `<mo>`
- [ ] `<mfrac>`
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
