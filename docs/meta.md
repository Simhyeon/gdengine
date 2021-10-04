### Todos

* [ ] Mediaikwi image macro needs change
- Image upload needs absolute path but file link needs only filename which is a bummer

* [ ] Vim snippets
    * [ ] FLowchart name changes
	* [ ] Upload\_image

* [x] Saving bot password in config.json is not necessarily a good idea, utilslize .env file
* [ ] Path ergonomics is hard to resolve
- For example, image file is based on current workingg diretory and in normal
scenarios, it is in root directory however some cases, index file may not be in
root directory. And image's source directory is hard to catch, especially in marp page

* [ ] Flowchart
    * [x] Make yes distinguishable
	* [ ] Make horizontal variation

* [ ] Font supports
    * [x] Flowchartgvz 
	* [ ] Flowchartjs 
	- It is not so intuitive to make it, because only data(dms) is given to html file
	* [-] Marp
	- NOt yet tested

* [ ] Wikitext previewer
    * [ ] Somehow find a way to mimick wikipedia for testing

* [ ] Consider adding debug flags

* [ ] Easily distributable renderer
	* [ ] DOCX + domain specific language
	* [ ] PPTX + dms
* [ ] Find a way to decrease external dependencies
	* [ ] Possibly reqwest crate

* [ ] run and test
  * [x] Test
	* [ ] Maybe send test flag to each renderer 
	* [ ] File diff output

* [ ] Basic utilies
  * [ ] Render
    * [ ] pandoc(docx)
	  * [x] At least make it work
	  * [ ] Make pure rust implementation
	    * [ ] This needs new language 
    * [ ] Web UI
	  * [x] Port macro file
	  * [ ] Make webui documentations
    * [ ] Flowchart
      * [ ] flowchart-js -> Not fully tested
	    * [ ] Interactive flowchart with given json chunk functionality
      * [ ] Graphviz -> Not fully tested
  * [ ] gdlogue
	* [ ] Make documentations of gdlogue types

## DONE

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
