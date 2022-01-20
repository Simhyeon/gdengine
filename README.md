# Gdengine, a game design engine

Gdengine is a game design documentation engine. Gdengine automates render
processes of various kinds of document formats, enable designers to utilize
multiple variables that is used across design documents, and can be easily
integrated to git based version control systems.

## Table of contents

1. [Why made a game design engine?](#why-made-a-game-design-engine?)
2. [Gdengine workflow](#gdengine-workflow)
3. [Intsall](#install)
4. [Dependencies](#dependencies)
5. [Currently supported renderers](#currently-supported-renderers)
6. [Basic usage](#basic-usage)
7. [Supported macros](#supported-macros)

## Why made a game design engine?

Because it is hardly sane to write documents in XML files when documents
changes so often and have to be versatile at the same time.

**XML is not suitable for VCS**. There are two major formats for game design
documents: PPTX and DOCX. Those are XML files and includes so many
representational tags which makes it harder to utilize VCS. Though subversion
somewhat enables better version control for the file formats, you lose many
features as trade-off. 

**Game design document changes often**, which is unideal but realistic. Designing
process itself is not so painful becuase there are no limits in tooling or
environments. Jira, google docs, notion, or any other collaborative tools you
name it; what fits a team is the best tool. But time comes when you have to
make a contained and statically built document. Those documents are prone to
errors such as typos and outdated informations.

As much as I believe programmers should not be accountable for memory errors, I
believe game designers and other developers **deserve better game design
processes**. This is the reason why I made a Gdengine.

## Gdengine workflow

Gdengine is a tool for game design documents creation. Gdengine does two jobs.

1. Process macros in index file. 
2. Render the index file into a target format.

There are various predefined macros built in gdengine for easier documents
creation. These macros let user to create multiple formats without modifying
index file. You can also define your macros or add style files for
deeper representational customizations too, though you need to make macros
fallable if you target multiple formats at the same time.

## Install

Refer [release page](https://github.com/Simhyeon/gdengine/releases/new) for installation.

You can build the project by yourself but you need to install following
binaries.
```
# Build binary youself.
cargo install gdengine

# You also need to manually install following binaries
# - Marp
# - Chrome or chromium
# - Pandoc
```

## Dependencies

- git flag : [git](https://git-scm.com/)
- flowchartgvz(static flowchart) : [graphviz](https://graphviz.org/)
- gdlogue(Dialogue) : [graphviz](https://graphviz.org/)

**Extra dependencies if you build gdengine yourself**

- [Marp-cli](https://github.com/marp-team/marp-cli)
- Chrome or [chromium](https://www.chromium.org/)
- [Pandoc](https://pandoc.org/)

Caveat: Chrome has higher priority over chromium

## Currently supported renderers

**Document renderer**
- marp (slide as html,pdf,pptx)
- mediawiki (web served wiki page)
- pandoc (docx, or say ooxml compatible file)

**Component renderer**
- flowchartjs (flowchart, html)
- flowchartgvz (flowchart, png, pdf)
- gdlogue (Dialgoue, html, png, pdf)
- webuibts (web based ui with bootstrap)

**To be updated**
- plotter ( Plot chart )

## Basic usage

```bash
# Init project with extra git flag
gde init --git

# Render with given renderer module
gde render -M <MODULE_NAME>

# e.g. render with marp backend 
gde render -m marp

# Render with pre-defined test scripts named wiki
gde test --target wiki
```

[full usage](./docs/usage.md)

## Macros

[Gdengine specific macros](./docs/macro.md)

Gdengine uses r4d macro processor intenrally and all [r4d built in
macros](https://github.com/Simhyeon/r4d/blob/master/docs/basic_macros.md) are
supported. You can utilize every r4d macros inside gdengine project.

Basic rad configuations are,

- All auths are open by default
- All macros are greedy be default
- Macro expansion is lenient by default
- Use comment option of start. Comment character is ```%```.
- Macro character is ```$```.

## Yet to come

* [ ] Docx renderer with docx-rs with markdown-pulldown-parser
- With possibly domain specific language designed for docx generation
* [ ] Create a separate crate for converting markdown into pptx file
* [ ] Compile static binary of dot renderer rather than using system path or use flou-cli

## Goal

- Easy documentation
- Cross platform without hassel and (possibly) without nodejs
- Intuitive macro definitions
- Useful document forms (pdf,pptx,html,docx)
- Various component renderings (dialogue, flowchart, webui)

## About Predecessor

This is a new cross platform implementation of previous
[gdmarp](https://github.com/Simhyeon/gdmarp/) program.

Gdengine was recreated in Rust because gdmarp lacked several core features that follows.

Firstly, gdmarp uses M4 macro and it prohibits usage of ```'``` and ```\````.You can
change quote syntax but there is always a tradeoff that hardly tolerable for
text based rendering program.

Secondly, gdmarp is a shell script which collaborates multiple command line programs.
Which means it is never meant for non-unix operating systems. And to make
matters worse, some command line utilities were GNU specific version and used
non-POSIX standard functions. Converting existing code into POSIX-compliant
codes were painful and sometimes just possible for some features. Other
platform was recommend to use docker image which was not the most ergonomic way
of using command line utility.

Finally, gdmarp became just too big to maintain as a form of dynamic scripting
language. Shellcheck is a great program, however I just missed so many features
from complete programming languages and especially Rust.
