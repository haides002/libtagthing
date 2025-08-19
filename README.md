# Libtagthing

A collection of functions to share code between tagthing frontends.

## Goal

Make a lib that enables Tagging for audio/visual files which support XMP.

## Principles

- Statelessness
- Keeping it simple

### Statelessness

Meaning no database so you can move your files however you want because tagthing
won't keep track of where they are to associate data with the files. The
application using the lib will have to manage the runtime state itself.

And because sidecar files are a hassle for now we won't do any of those either
(also a lot of formats support XMP or other metadata formats which we'll use)

# Documentation
We are using the dc:subject array for storing tags.

We offer subtags and taggrouping. This is what they look like in plain text:

subtags: nature/tree

taggroup: person:linus

Internaly the media info is stored in Structs that implement the media trait.

## Filtering

Features:
- Binary operators (AND, OR, XOR, NAND, XNOR, NOT)
- Wildcards
- Grouping (Brackets)

Syntax:

| Longhand | Shorthand | Description |
|----------|-----------|-------------|
|  AND     |     &     |File has to have both tags|
|  OR      |    \|     |File has to have at least one of the two tags|
|  NOT     |     !     |File does not have the tag|
|  XOR     |           |File has to have only one of the two tags|
|  NAND    |           |File does not have both of the tags|
|  XNOR    |           |File has both or none of the tags|
|          |     *     |Wildcard, any character(s)|
|  \(      |           |Right side bracket|
|  \)      |           |Left side bracket|


Rules:

The binary operators `AND`, `OR`, `XOR`, `NAND` and `XNOR` (long and shorthand) have a Space in between them.

The binary operator `NOT` in shorthand `!` is allowed to contact everything. `NOT` is also applied to the right-hand argument.

Brackets can contact everything but need to have a filter in between them.

The wildcard `*` needs to be part of a tag.

Expressions get evaluated from left to right.

### Examples


`tree AND nature`

`tree OR landscape AND !car`

`tree XOR (landscape AND river NAND NOT(Car OR Bus))`

(just `tree` is also valid of course)
