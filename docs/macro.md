# Renderers

* [marp](#marp)
* [mediawiki](#mediawiki)
* [pandoc](#pandoc)

* [flowchartjs](#flowchartjs)
* [flowchartgvz](#flowchartgvz)
* [gdlogue](#gdlogue)
* [webuibts](#webuibts)

## Document renderer macros

Document renderers render documents that are mostly the final
target of design processes.

### Default(global) macros
```gddt
% Transpose. horizontal array into vertical array
$tp(1,2,3)
% This is same with following
1
2
3

% <ifmod>
% If only given mod is used, execute expression
$ifmod(marp,Expression to execute)

% <ifmodel>
% If only given mod is used, execute expression
% else execute other expression
$ifmodel(marp,Expression to execute,Other expression to execute)

% Color settings
$COLOR_BLACK()
$COLOR_RED()
$COLOR_GREEN()
$COLOR_BLUE()
$COLOR_YELLOW()
$COLOR_MAGENTA()
$COLOR_CYAN()
$COLOR_WHITE()

% Override this as one's demand
% Such as followings,
$static(COLOR_BLACK,#000000)
$static(COLOR_RED,#ff0000)
$static(COLOR_GREEN,#00ff00)
$static(COLOR_BLUE,#0000ff)
$static(COLOR_YELLOW,#ffff00)
$static(COLOR_MAGENTA,#ff00ff)
$static(COLOR_CYAN,#00ffff)
$static(COLOR_WHITE,#ffffff)
```

### Compatibility macros

These macros are all defined through document renderers.
```
% Headers
$h1(First)
$h2(Second)
$h3(Third)
$h4(Fourth)
$h5(Fifth)

% Emphasis
$b(Bold text)
$i(Italic text)
$bi(Bold & italic text)
$st(Strike through)
$ud(Underline)

% Indentation
$idt(1) Indent 4 spaces

% Paragraph
$p(Paragraph content)

% Lists
$ul(1) First level unordered list
$ul(2) Second level unordered list

$ol(1) First level ordered list
$ol(2) Second level ordered list

% Url(link)
$url(https://website.com)

% Image
$image(image_path.png)

% Cut page and go to newpage
$newpage()
	
% Text color
$text_color(css_compliant_color_code,Contnet to be color highlighed)

% Shorthand text color macros
% This respects Color coded defined by default color settings
$black(Text)
$red(Text)
$green(Text)
$blue(Text)
$yellow(Text)
$magenta(Text)
$cyan(Text)
$white(Text)
```

### marp (slide as pdf, pptx, html)

**Environment variables**
```gddt
% Height of slide page
$v_basis_height()

% Default font size as px
$v_font_default()

% Default table header font size
$v_thead_default()

% Default table body font size
$v_tbody_default()

% Default font family
$v_font_family()

% Override env values as
$static(v_basis_height,500)
$static(v_font_default,24)
$static(v_thead_default,18)
$static(v_tbody_default,14)
$static(v_font_family,sans-serif)
```
**Class**
```
% Class changes layout of the slide that is used.
% Class related macros ought to come first before other macros in the slide

% Title
% This is a shorthand macro of,
% $class(title)
% $h1(Main title)
% $h2(Sub title)
$title(Main title,Sub title)

% tnc(Title and contents)
% This pre-allocates large chunk of space for contents
$class(tnc)

% Gdtable, applies specific style to tables
$class(gdtable)

% Split screen
% split related macros require split class to be
% defined before
$class(split)
```
**Layout**
```gddt
% Apply pre-defined style into a slide
$styles(<STYLE_NAME>,...)

% Default style files' names are
% - layout
% - table

% Sized image
$simage(image_path.png,max_image_size)

% Text macro with font size
$text(20px, Content that has a font size of 20px)

% Flex box or flex container
% Use this macro inside split macro 
% Fbox inside split slide takes the same amount
% among fboxes group
$fbox(
	-- Content goes here --
)
% Also sets font size
$ffbox(font_size,
	-- Content goes here --
)

% Table of contents macro
$toc(page_number)

% split related macros
% You can put nested other contents inside panes
% Most of the time, it will works...
$left()
	Left pane contents
$right()
	Right pane contents
$end()

% Center
% Center the given contents
$center(Content to be centered)
```
**Complex table macros**
```
% Use r4d's table macro if
% you want to use simple syntax of table
% construction

% Start building a table
$tstart()

% Set custom font size, this is optional
$tfont(20px,14px)

% Set headers
$theader(number,Content)

% Set rows
$trow(1,I'm second column content)

% End table build
$tend()
```
**MISC**
```
% Cut page
% but this is not compatible with other renderers
% and only work in marp rendering
$splitpage()
```

### mediawiki (wikitext)

**Wiki page link**
```gddt
$wiki_page(Wiki_Page_url)
$wiki_page_alt(Wiki_Page_url, alternative text)
```

**MISC**
```gddt
% Horizontal line
$hr()
```

**Advanced table**
```
% This is currently quite expreimental
% But you can use marp's advanced table macro syntax in mediawiki
$tstart()
...
$tend()
```

### pandoc (docx)

Currently pandoc only supports compatibility macros.

## Component renderer macros

Components are mostly contained item that is included in
document.

### flowchartjs & flowchartgvz (flowchart)

Both renderer have same interface

```gddt

$flowchart_begin()

$fstart(Start node,node1)

$fnode(node1,Label1,node2)

$finput(id,label,nextId)

$fcond(node2,Label2,node3)

$fnode(node1,Label1,node2)

$fnoda(node1,Label1,node2)

$end(End node)

$flowchart_end()

```

### gdlogue (dialogue)

```
$dial_begin()

$start_node()

$text_node()

$omit_node()

$end_node()

$dial_end()
```

### webuibts (webui)

To be dated
