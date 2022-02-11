### Changed

* [x] Marp font macro is broken, fix it
* [x] Add independent font macro variant
- I made table macro as relay 
- THus I don't need to make new one

* [x] Add new global macros
	* [x] Add table and query in single macro
	* [x] $npc macro ( new page with class support  )

### Todos

**Others**

* [ ] Create generic renderer for future

* [-] Copy worked strange in my experience
- But I cannot reproduce... hell..
* [ ] Support split images macro

* [ ] Gcalc integration
* [ ] Make sure marp indentation works

* [ ] New global macro
	* [ ] TOC macro
	- This reads information into a chunk and creates
	- Necessary information as another macro
	for example
	```
	When given --toc option gdengine registers macro hook for specific macro that
	creates toc, such as h1 for marp and h2 for wikipage
	
	This yields text into some macro such as GDE_TOC_LIST
	
	And re-evalutes the same file.
	
	This should also come with END macro of r4d which is not implemented yet to
	evade extra operational cost.
	```

**Renderers**

1. Github readme renderer
- This would be really cool
1. Graph renderer
	* [ ] Battle test it
	* [x] Make new error type rather than box new error hassle
	* [x] Plot rendering
		* [x] Bar chart
		* [x] Line chart
		* [x] Area chart
	* [x] Make plotters builder macros 
2. Flowchart 
	* [ ] Better flowchart implementation
	- Maybe with flou-cli ?
3. Docx renderer
	* [ ] Complete docx renderer
4. MISC
	* [-] Port ul and ol to contained macro e.g. -> $ul(2,Content goes here) 
	* [ ] Rather than porting, add enumerate and itemize for concrete listing macros
5. Improve marp renderer
	* [x] Make split macro set as builder pattern
	* [ ] Fix split class
		- Split class changes total slide behaviour
		- Make this as some split container
	* [ ] Make proper horizontal layout ( other than split )
		- This should be recoverable
		- Make it as an builder pattern
6. mediawiki test option
    * [ ] Custom style file support
    * [ ] Should copy images to build directory
	* [ ] Create table of contents (Use toc macro)
	* [ ] Apply proper css
	  * [ ] TOC style
	* [ ] Hook is necessary for simulated list elements
	- This is because wikitext parser respects newline after list element while
	html preview doesn't
	- To solve this, line element macro should invoke hook trigger of surplus
	newline attachment when encountered newline.

**Project visibility**

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

---

* [ ] Use flou-cli for flowchart generation because graphviz is hard to distribute
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

* [ ] Basic utilies
    * [ ] Flowchart
      * [ ] flowchart-js -> Not fully tested
        * [ ] Interactive flowchart with given json chunk functionality

## DONE
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
