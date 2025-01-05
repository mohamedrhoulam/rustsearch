# rustsearch

A high-performance text indexing engine for searching large documents or corpora implemented in Rust and inspired by the [C++ Pisa text search engine.](https://github.com/pisa-engine/pisa)

## Overview

## Workflow

- The following workflow is inspired from the [PISA Index Building Pipeline (Mallia et al., 2019 ):](https://github.com/pisa-engine/pisa/wiki/Index-Building-Pipeline)

- Collection Processing
  - Load documents
  - Extract contents
  - Tokenize
  - Filter (Stemming + Stopword removal)

- Forward Index
  - Term Lexicon
  - Document Lexicon

- Inverted Index
  - Document reordering
  - Compression

- Index Compression

- Query Pocessing