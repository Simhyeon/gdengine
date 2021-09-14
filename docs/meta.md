### Imminent

* [x] Make sandboxed release package.
- Compiled marp
- Chromium
- pandoc
- Graphviz dot renderer needs more consideration

* [ ] Update vs code extensions

### Todos

* [ ] Make docx renderer in rust
* [ ] Make a separate crate to create a pptx file
- Completely ditch nodejs dependencies
* [ ] Find a way to decrease external dependencies
- Reqwest is super heavy... though it is not so easy to create all necessary
logics with only hyper crate.
- Ditch clap derive and use clap builder. Who would've though clap derive is that heavy
* [ ] Find if graphviz/dot binding exists
* [ ] Find a way to create global installer program
- graphviz

* [ ] run and test
  * [x] Run -> Simply run not so hard
  * [x] Test
    * [x] No panicking
	* [x] Final result

* [ ] Basic utilies
  * [ ] Render
    * [x] Marp(pdf, pptx)
    * [x] mediawiki(wikitext)
	  * [x] Confirm send works
    * [ ] pandoc(docx)
	  * [x] At least make it work
	  * [ ] Make pure rust implementation
    * [ ] Web UI
	  * [x] Port macro file
	  * [ ] Make webui documentations
    * [ ] Flowchart
      * [ ] flowchart-js -> Not fully tested
	    * [ ] Interactive flowchart with given json chunk functionality
      * [ ] Graphviz -> Not fully tested
  * [ ] gdlogue
	* [ ] Make documentations of gdlogue types
    * [x] Change logics
	* [x] Make renderer's rust implementation

  * [x] Postprocess
     * [x] Copy file
	 * [x] Remove cache

## DONE

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

### How to make docx renderer

**Options**

- Pandoc
Pandoc is useful in a way that it can insert raw openxml directly.

- Pulldown parser + docx-rs
Highly customizable but hard to make
