### Imminent

* [ ] Make docx renderer in rust
* [ ] Make a separate crate to create a pptx file
* [x] Make gdlogue rust implementation
  * [x] Png
  * [x] Pdf
  * [x] Html
* [x] Make mediawiki renderer in rust

### Todos

* [ ] Find a way to decrease external dependencies
* [ ] Find if graphviz/dot binding exists
* [ ] Find a way to create global installer program
- graphviz
- nodejs
- pandoc

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
	  * [ ] Make pure rust implementation
    * [ ] Web UI
	  * [x] Port macro file
	  * [ ] Make webui documentations
    * [ ] Flowchart
      * [ ] flowchart-js -> Not fully tested
      * [ ] Graphviz -> Not fully tested
  * [ ] gdlogue
	* [ ] Make documentations of gdlogue types
    * [x] Change logics
	* [x] Make renderer's rust implementation

  * [x] Postprocess
     * [x] Copy file
	 * [x] Remove cache

## DONE

* [x] Make cross platform command call
* [x] Refactor config parsing
* [x] Make a logic structure much more maintainable
  * [x] Make path lazy static
  * [x] Make structure, method, argument naming cohesive and consistent

* [x] Json parsing

## Extra

* [ ] A shower thought, rendering should be a sole burden of the given renderer
not a burden of executor or game design engine
- What I mean is that gdlogue is only about validating and creating out.gv file
- But it might be a better refactor to enable gdlogue to actully create a file not just yield out.gv file. I mean it is much more natural, isn't it?
