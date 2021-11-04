# Command options

```
# Show help message
gde help

# Inititiate project
gde init
# Also initiate with git 
gde init --git

# Render with marp renderer
gde render -m marp

# Available renderers are,
# marp
# mediawiki
# pandoc
# flowchartjs
# flowchartgvz
# gdlogue
# webuibts

# Available flags for renderer
-m, --module <STR>     # Renderer to render
-i, --input <PATH>     # File to render, default is index.gddt
-o, --output <PATH>    # Output file name
-f, --format <STR>     # Renderer specific format

-c, --copy <PATH>      # Copy built file into given path
-e, --env <VAR>        # Set environment variable as empty value
-E, --envfile <PATH>   # Read environment variables from given path file

-s, --strict           # Strict macro mode, panicks when macro fails
-p, --preserve         # Preserve cache file
    --test             # Yield multiple informations for test
	--log              # Log macro invocations

# You can run multiple render process with run command
gde run              // run first process in gde_config.json's run field
gde test             // run first process in gde_config.json's test field
gde run -t marp      // run marp process
gde test --target mw // run mw process
```

# Config example

```json
{
    "run": {
		"marp": ["render -m marp"],
		"second": [
			"render -m flowchartjs -e UPLOAD_JS", 
			"render -m marp"
		]
	},
    "test": {"mw": ["render -m mediawiki --test --preserve"]}
}
```
