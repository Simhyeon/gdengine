### Todos

* [ ] Vim snippets
    * [ ] Flowchart name changes
	* [-] Upload\_image
	- To think again, is this really necessary? Well sadly yes

* [ ] Font supports
    * [x] Flowchartgvz 
	* [ ] Flowchartjs 
	- It is not so intuitive to make it, because only data(dms) is given to html file
	* [-] Marp
	- NOt yet tested

* [ ] Wikitext previewer
    * [ ] Somehow find a way to mimick wikipedia for testing
	- First use convert wikitext to markdown or simply make markdown in the first place.
	- Generate html file
	- Apply some custom css to the file

* [ ] Debugging
    * [ ] Diff
	* [ ] Miscellaenous rad flags

* [ ] Easily distributable renderer
	* [ ] DOCX + domain specific language
	* [ ] PPTX + dms

* [ ] Find a way to decrease external dependencies
	* [ ] Possibly reqwest crate

* [ ] Good documentations
    * [ ] Gdlogue
	* [ ] Flowchart
	* [ ] Webui
	* [ ] Of course others

* [ ] Basic utilies
    * [ ] Web UI
	  * [ ] Make webui documentations
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
