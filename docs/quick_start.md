### Abstract

We will use marp renderer for this tutorial.

### Start a project

Start a porject with git initiated.

```bash
# In target directory
gde init --git

# If git is not installed then simply init wihtout git
gde init
```

### File structure

```bash
# <Root directory>
├── .env
├── varfile.env
├── build
├── cache
├── gde_config.json
├── inc
├── index.gddt
├── index.r4d
└── res
```

Short explanations for each file + directory

- .env             : where renderer specific environment varaible is saved.
- varfile.env      : built in file for saving variables 
- build            : where built file is located
- cache            : where cached file is located you can preserve cache with --preserve flag
- gde\_config.json : where gde related config is located
- inc              : built in folder that you can place fragmented text files
- res              : built in folder that you can place fragmented resource files
- index.gddt       : entry file for processing. you can change entry file with --input option
- index.r4d        : user defined macro rule file. This cannot be customized.

### Simple index file 

```gddt
$styles(layout)

$title(Title, Subtitle)

$splitpage()

$h1(First slide)

Content goes here. Marp has markdown syntax and any markdown compatible syntax
is allowed. Though there are several caveats to marp which is sometimes
non-trivial unfortunately.

$splitpage()

$h1(Second slide with table)

$table(github,header1,header2,header3
value1,value2,value3)

$splitpage()

$h1(Simple macros)

Current date is $date()

Current time is $time()
```

Render file with command which yield pdf format

```bash
gde render -m marp -f pdf
```

[result\_pdf](simple_output.pdf)
