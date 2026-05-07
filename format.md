# Registry Format

## The Problem

The project manager needs a format for storing registered projects. It should be concise, but also not too difficult to implement.

Existing options:
 - SQLite
   - rusqlite
   - turso
   - sqlite crate
 - Markup Languages
   - JSON
   - YML (Not a favorite)
   - XML

Custom options:
 - Markup Languages
   - QANLang
   - A new one (yay)
 - Database
   - last resort if nothing works

## What Needs to be Stored

The project manager's first order of buisness is to switch to a directory. So, this format needs to store a directory path.
It will also eventually need to store the init file path. On register, it should either fuzzy search for this file and, if it does not exist, create one. This file's location should be stored so fuzzy search does not have to be run again.

Options:
 - SQLite may be a bit heavy for the current application
 - Markup languages will help with size in the short term, but scaling up might be difficult
