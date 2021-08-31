### Todos

* [ ] run and test
* [ ] Refactor config parsing

* [ ] Basic utilies
  * [ ] Render
    * [x] Marp(pdf, pptx)
    * [ ] mediawiki(wikitext)
	  * [ ] Confirm send works
    * [ ] pandoc(docx)
    * [ ] Web UI
	  * [ ] Simply ported macro file, not functioning
	  * [ ] Make webui documentations
    * [ ] Flowchart
      * [ ] flowchar-js -> Not fully tested
      * [ ] Graphviz -> Not fully tested
  * [ ] gdlogue
    * [x] Change logics
	* [ ] Make documentations of gdlogue types

  * [x] Postprocess
     * [x] Copy file
	 * [x] Remove cache

## DONE

* [x] Make a logic structure much more maintainable
  * [x] Make path lazy static
  * [x] Make structure, method, argument naming cohesive and consistent

* [x] Json parsing

## Extra

* [ ] A shower thought, rendering should be a sole burden of the given renderer
not a burden of executor or game design engine
- What I mean is that gdlogue is only about validating and creating out.gv file
- But it might be a better refactor to enable gdlogue to actully create a file not just yield out.gv file. I mean it is much more natural, isn't it?
