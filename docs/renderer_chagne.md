### The problem

Currently docx and slide generation is dependent on third party programs,
respectively marp and pandoc. Though gdengine has very distinct and
diverse functionality which is not practically possible to handle in
single language environment.

However using external programs without and sincere considerations makes
execution logic inconsistent and distribution of programs unnecessarily
burdensome. For example, inducing end users to install external programs can
cause undtested behaviours because of difference in versions. Including
external programs in release package is rather straightforward, but it also has
some caveats such as increased package size and procrastinated version updates
of external programs makes end users to predict different results.

### A sub-optimal solution 

As stated above, packaging everything in single release is relatively
consistent and eaiser solution. Gdengine's future direction is to use this
solution. External programs planned to be included are followings,

- Nodejs binary
- Marp-cli or marp imported index.js with custom configs
- Chromium for puppeteer
- Pandoc as a single binary (without haskell dependencies) : 73mb
- Graphviz dot program (image renderer) : 72mb

#### Maybe the "Deno"

Including nodejs and marp-cli with node modules doesn't not necessarily look
great. Not because it has relatively bloated binary size, but because about
project's philosophy. Although it hasn't been stated otherwise but gdengine
aims to be a concrete and easily mainatable program. Thus using a javascript
program or library diminishes the original intent of the program.

That's the reason why I'm looking forward to deno. Deno provides following
features compated to nodejs.

- Typescript's type safety by default
- Native support for "compilation" to single execution binary

Through I'm not sure eif deno can utilize exsting node modules without losing
the functionality. But it worth a try.
