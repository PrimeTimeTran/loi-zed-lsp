---
title: estate.md
path: ./crates/learn/src/mold/estate.md
description: This file is Cargo workspace crate named "learn". It will be promoted to a Delta Integral Paradigm(DIP) workspace generated crate once it's stabilized.
purpose:  
  - Previously .md files were exclusively used for planning.
  - We want "more" in the spirit of the DIP philosphy.
  - Headers/sections are titles used to generate visibily readable/appealing views in IDEs(larger font, diff colors, etc) and most importanly to divide bullet points.
  - Each section can have a ```loi ``` block which defines key value pairs, meta data, for it's own section.
  - Think of the ```.loi ``` block as "section level front matter".
  - A file named "estate.md" inside of a DIP workspace's lowest common denominator are it's bulletpoints.
    - The bulletpoints are plans, outlines, stories, goals, aspirations, todos, criteria, specs, so much more... but concretely now, a list of "files to generate". 
  - This file could have no front matter, no titles, no code blocks and still be a proper DIP estate codegen entrypoint if it has bullpoints.
  - Each bulletpoint item is either a dir or a file. Dirs do not have extensions whereas files do.
  - Lastly, if the estate crate executable processes this file with no code blocks for sections, then the default behavior is to generate the files inside of ./src. ./src being relative to where the user runs the command, not where the binary is installed or the file estate.md file is defined.
  - To start, the only key we have now is path. The path we want this bulletpoint list of items to be generated at. 
    - Currently there are only 4 abs paths which will have new files if everything goes according to plan.
      - ~/.loi
      - ./
      - ./src
      - ./public
---

# 1. Global
---
path: ~/.loi
description: The end goal of this crate is to install everything needed for the DIP toolchain. 
  - We want global dirs/files for historic and practical reasons
---

- "core"
- "daemon"
- "dashboard"
- "executable"
- "ide"
- "lsp"
- "toolchain"

# 2. Define DIP workspace deps/constructs.
---
path: ./
description: No names are finalizes yet but these in the direction.
  - Genesis is our estate "manager" file. Although that may be a slight misnomer as all files will play some part in management, like this file.
  - settings.json for IDE config
  - index.json for an index/registry of the FS which is maintained in a "omnipresence" way, regardless of view. UID, file name, alias, view tags which the index.json will one day hold.
  - view.json a top level file because it encapsulates a cornerstone objective of the DIP architecture, the believe that a new view/ruleset/zoom over the same tokens/symbols/materials/artifacts not only enables computers to do new things but for humans to more easily "shift gears"
---

- "genesis.loi"
- ".loi/settings.json"
- ".loi/index.json"
- ".loi/view.json"

# 2. Define downstream DIP pkg/fw/lang manager files.
---
path: ./
description: Our MVP supported languages/runtimes.
---

- "Cargo.toml"
- "go.mod"
- "go.sum"
- "package.json"
- "pyproject.toml"

# 3. Define the comprehensive estate knowledge base documents
---
path: ./docs/(estate)
description: Docs for reference. 
  - This will moved to the ~/.loi dir if/when we complete a few templates which showcase the design philosphy of DIP.
  - The path from root for this section of files is ./docs/(estate)/*
  - It follows the convention of "( )" representing a logical grouping that isn't exposd in the route. 
    - The nesting and then "()" wrapped tag not only drives exposure behavior but is human readable explanations as to "what type" of doc this is.
---

- "0.preface.md"
- "1.introduction.md" 
- "2.prologue.md" 
- "3.welcome.md"
- "4.philosphy.md" 
- "4.philosphy.[zen].md" 
- "4.philosphy.[problem-space].md" 
- "4.philosphy.[pain-point].md" 
- "4.philosphy.[reality].md" 
- "4.philosphy.[friction].md" 
- "5.analogy.[deck-of-cards].md" 
- "5.analogy.[analog-television].md" 
- "5.analogy.[carrier-pigeon].md" 
- "5.analogy.[open-closed-principle].md" 
- "4.philosphy.[vision/solution].md" 
- "6.philosphy.[goal].md" 
- "7.philosphy.[rationale].md" 
- "8.philosphy.[principle].md" 
- "9.philosphy.[arch].md" 
- "10.philosphy.[specs].md" 
- "11.philosphy.[roadmap].md" 
- "12.guide.md" 
- "13.best-practices.[open-closed-principle].md" 
- "14.convention.md" 
- "15.cheatsheet.md"
- "16.example.md" 
- "17.community.md" 
- "18.contributing.md" 
- "19.references.md" 
- "20.glossary.md" 

# 4. Define web platform showcase
---
path: ./public
description: A POC that we "get you" the user and that our "bottom up" doc generation does not end at a panel in the ID but crosses the finish line at a web platform compliant bundle which will one day encaptulate any JS framework current or to come in the future.
  - 
---

- "static"
- "assets/"
- "css/style.css"
- "js/main.js"
- "js/delta.js"
- "js/md.js"
- "js/mdx.js"
- "js/react.js"
- "js/vue.js"
- "index.html"

---
$ cargo build --bin bootstrap
$ cargo install --bin bootstrap