### Todos
$todo_start()
* [ ] Reduce external dependencies
* [ ] Error: TOC\_LIST doesn't exist but it somehow works anyway? Wut?
* [ ] Don't create build or cache if index.gddt is not present.
* [ ] Only expand macros option : Um... what was the intent? For testing? I don't remember
* [ ] Github renderer
* [ ] Split macro should work differently in other platforms
* [ ] Maybe todo macro?
	- THe problem is that todo will always invoke, but is it necessary? or intended?

- Others

* [ ] Enable autocomplete
* [-] Copy worked strange in my experience
	- But I cannot reproduce... hell..

* [ ] Gcalc integration
* [ ] Make sure marp indentation works

- Renderers

* [ ] Test renderers
	- Graph renderer
* [ ] 1. Github readme renderer
* [ ] 2. Mediawiki preview
	* [ ] Hook is necessary for simulated list elements
	- This is because wikitext parser respects newline after list element while
	html preview doesn't
	- To solve this, line element macro should invoke hook trigger of surplus
	newline attachment when encountered newline.
* [ ] 3. Docx renderer

- Project visibility

* [ ] Documentation
	* [x] Make full usage
	* [ ] At least include schmeas for each renderer module
		- Everything except webuibts and flowchart
	* [ ] Korean version

* [ ] Project organization
	* [ ] Add demos in master branch
	* [ ] Make cargo branch for cargo push
	* [ ] Make build branch for building
	* [ ] Make dev branch for developing
$todo_end()

### Others

**Better Flowchart**
* [ ] Possibly use flou-cli for flowchart generation because graphviz is hard to distribute
- I don't hate graphviz but it has so much dependencies and features that I
don't need. Thus there are rarely contained binary for general usage which
sucks

* [ ] Make flowchartgvz much more ergonomic
- The problem is, there are quite unallowed patterns which are vey confusing at first sight
- First no condition should not directly go to other node, it will totally break everything
- Second You cannot directly go to condition node, it will also break everything

- What could be an alternative way?
- At least give a chance to do it in a way
    - For example -> $fconda for direct no connection
	- $fdot for empty branch
- Create a new domain specific language ;-) and fuck my life

* [ ] Font supports
    * [x] Flowchartgvz 
    * [ ] Flowchartjs 
    - It is not so intuitive to make it, because only data(dms) is given to html file
    * [-] Marp
    - Not yet tested

* [ ] Easily distributable renderer
    * [ ] DOCX + domain specific language
    * [ ] PPTX + dms
	* [ ] Graphviz alternative

* [ ] Flowchart
  * [ ] flowchart-js -> Not fully tested
    * [ ] Interactive flowchart with given json chunk functionality

### Changed

* [x] Marp font macro is broken, fix it
* [x] Add independent font macro variant
- I made table macro as relay 
- THus I don't need to make new one

* [x] Add new global macros
	* [x] Add table and query in single macro
	* [x] $npc macro ( new page with class support  )

* [x] Create generic renderer for future
* [x] Support split images macro
* [x] TOC macro

Marp : 
* [x] Refactored split class
* [x] Make proper horizontal layout

Mediawiki-Preview
* [x] Should copy images to build directory
* [x] Create table of contents (Use toc macro)
* [x] Custom style file support


## DONE

* [-] Custom css style integration: I mean... include it with $include macro

* [x] Utilize r4d storage feature
	* [x] Remove ImageList file logics
	* [x] Remove graphviz source file logics
	- With test function of writing

* [x] Read any file (but mostly for .env format) as static macro list

* [x] Mkdir cache or build if not existent
* [x] Read env file with option because setting all env files all the way is
very inefficient

* [x] Mediawiki functionality
  * [x] Cannot add duplicate image, just ignore in this case
  * [-] This can be improved but temporarily it's fine 
* [x] Multiple target for test and run
* [x] Util function to squueze newliens into a single one

* [x] Remove env from config and utilize .env file

* [x] Flowchart
    * [x] Make yes distinguishable
    * [x] Make horizontal variation

* [x] Renderpath is not set right
* [x] Purge was not default -> Fixed

* [-] Make sandboxed release package.
- Compiled marp
- Chromium
- pandoc
- Graphviz dot renderer needs more consideration
* [x] Make vim snippets
* [x] Make gdlogue rust implementation
  * [x] Png
  * [x] Pdf
  * [x] Html
* [x] Make mediawiki renderer in rust

* [x] Make cross platform command call
* [x] Refactor config parsing
* [x] Make a logic structure much more maintainable
  * [x] Make path lazy static
  * [x] Make structure, method, argument naming cohesive and consistent

* [x] Json parsing

* [x] Postprocess
    * [x] Copy file
    * [x] Remove cache

### How to make docx renderer

**Options**

- Pandoc
Pandoc is useful in a way that it can insert raw openxml directly.

- Pulldown parser + docx-rs
Highly customizable but hard to make
