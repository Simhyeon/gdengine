### Gde, game design engine

This is a new cross platform implementation of previous
[gdmarp](https://github.com/Simhyeon/gdmarp/) program.

### Dependencies

- **Git** for --git flag

- marp(slide): [marp-cli](https://github.com/marp-team/marp-cli) + [chromium](https://www.chromium.org/)
- flowchartgvz(static flowchart) : [graphviz](https://graphviz.org/)
- gdlogue(Dialogue) : [graphviz](https://graphviz.org/)

### Renderers

**Document renderer**
- marp (slide as html,pdf,pptx)
- mediawiki (web served wiki page)

**Component renderer**
- flowchartjs (flowchart, html)
- flowchartgvz (flowchart, png, pdf)
- gdlogue (Dialgoue, html, png, pdf)
- webuibts (web based ui with bootstrap)

### Usage

```
gde render -m marp -p -c ../build
```

### Yet to come

* [ ] Docx renderer with docx-rs with markdown-pulldown-parser
* [ ] Create a separate crate for converting markdown into pptx file
* [ ] Global installer for dependencies

### Goal

- Easy documentation
- Cross platform without hassel (Glibc, Musl, Windows) and without nodejs
- Intuitive macro definitions
- Useful document forms (pdf,pptx,html,docx)
- Various component rederings (dialogue, flowchart, webui)
