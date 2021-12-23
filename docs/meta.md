### Changed

* [x] Bug : Copy doesn't work on preserve flag hmm...
- I don't know what the heck happened. It is somehow fixed... Welp.
* [-] Bug : Component render may fail if some macros were not provided 
Now it yield proper and help error message

* [x] Make gdtable's header width to be more aesthetically pleasing
* [x] Image upload should come first

### Todos

* [ ] Make sure marp indentation works

* [ ] Documentation
	* [ ] Make full usage
	* [ ] At least include schmeas for each renderer module
		- Everything except webuibts and flowchart
	* [ ] Korean version

* [ ] Add demos in master branch
* [ ] Make cargo branch for cargo push
* [ ] Make build branch for building
* [ ] Make dev branch for developing

* [x] Enable user to override programs (marp, chrome + chromium,)
- Also set flag to only use contained binary
- At least it doesn't break existing setting
- !!But needs test of overriding!!

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

* [ ] Lengthy readme for presentation and other things.

* [ ] A graph renderer (matplotlib)
- This would be awesomely cool...

* [ ] Better flowchart pls...
- Maybe with flou-cli ?

* [ ] Complete docx renderer
- I dont' if it is possible or not.

I suffered a lot for "ul" and "ol" macro, maybe $ul(level, content) is better I
guess? I'm just too afraid to ask at this time.

Always prefer builder pattern because it is recoverable by other formats

* [ ] Make split macro set as builder pattern

* [ ] Split class is strange
- Split class changes total slide behaviour
- Make this as some split container

* [ ] Make proper horizontal layout ( other than split )
- This should be recoverable
- Make it as an builder pattern

* [ ] Color support
    * [x] With basic syntactic sugar for red,green,blue -> Etcetra (linux term colors) 
    * [ ] Configure color easily

* [ ] Marp uses strange path... because file exists in root not in build
- Maybe this is fine but can it be improved?

* [ ] mediawiki test option is really necessary
    * [ ] Custom style file support
    * [ ] Should copy images to build directory
	* [ ] Create table of contents (JS function)
	* [ ] Apply proper css
	  * [ ] TOC style
	* [ ] Hook is necessary for simulated list elements
	- This is because wikitext parser respects newline after list element while
	html preview doesn't
	- To solve this, line element macro should invoke hook trigger of surplus
	newline attachment when encountered newline.

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

---

* [-] Path ergonomics is hard to resolve
- For example, image file is based on current workingg diretory and in normal
scenarios, it is in root directory however some cases, index file may not be in
root directory. And image's source directory is hard to catch, especially in marp page
- However there are no gold magic for this, keep it well documented.

## DONE

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
