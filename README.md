### Gdengine, a game design engine

This is a new cross platform implementation of previous
[gdmarp](https://github.com/Simhyeon/gdmarp/) program.

[한글](docs/korean.md)

### Install

Refer [release page](https://github.com/Simhyeon/gdengine/releases/new) for installation.

### What it does

Gdengine is a tool for game design documents creation. Gdengine does two jobs.
First process macros in index file. Secondly, render index file into a target
format.

There are various predefined macros built in gdengine for easier documents
creation. Also macros lets user to create multiple formats without modifying
index file.

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

```bash
# Init project 
gde init --git

# Render with marp backend 
gde render -m marp
```

### Usage

Gdengine specific macros, [Usage](docs/usage.md)

Gdengine uses r4d macro processor inside, [r4d built in
macros](https://github.com/Simhyeon/r4d/blob/master/docs/basic_macros.md). You
can utilize every r4d macros inside gdengine project.

Plus, Gdengine opens all permissions so you don't have to configure any authorization
yourself.

### Macros

[Macros](docs/macro.md)

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
