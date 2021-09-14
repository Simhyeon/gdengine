### Gde, game design engine

This is a new cross platform implementation of previous
[gdmarp](https://github.com/Simhyeon/gdmarp/) program.

### Install

Refer [release page](https://github.com/Simhyeon/gdengine/releases/new) for installation.

### Dependencies

- **Git** for --git flag

- flowchartgvz(static flowchart) : [graphviz](https://graphviz.org/)
- gdlogue(Dialogue) : [graphviz](https://graphviz.org/)

### Renderers

**Document renderer**
- marp (slide as html,pdf,pptx)
- mediawiki (web served wiki page)
- pandoc (docx, or say ooxml compatible file)

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

* [ ] Proper documentation to use gdengine
* [ ] Detailed documentation for macro usages

* [ ] Docx renderer with docx-rs with markdown-pulldown-parser
- With possibly domain specific language designed for docx generation
* [ ] Create a separate crate for converting markdown into pptx file
* [ ] Compile static binary of dot renderer rather than using system path

### Goal

- Easy documentation
- Cross platform without hassel and (possibly) without nodejs
- Intuitive macro definitions
- Useful document forms (pdf,pptx,html,docx)
- Various component renderings (dialogue, flowchart, webui)
