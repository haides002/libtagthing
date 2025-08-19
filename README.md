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
