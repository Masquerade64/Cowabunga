# Cowabunga
Decryption tool for assets.pie for Digital Eclipse games:

- Teenage Mutant Ninja Turtles: The Cowabunga Collection
- Atari 50 (+ The Wider World of Atari DLC and The First Console War DLC)
- The Making of Karateka.
- Garbage Pail Kids: Mad Mike and the Quest for Stale Gum
- Llamasoft: The Jeff Minter Story
- Teris Forever (same key as Jeff Minter)
- Blizzard Arcade Collection (music.pie)

This tool was made in its entirety by SowwyItsAnAlt.

```
Usage: cowabunga64.exe [OPTIONS] <INPUT> <OUTPUT>

Arguments:
  <INPUT>
  <OUTPUT>

Options:
  -k, --key <KEY>  [default: cowabunga] [possible values: cowabunga, atari, atari-dlc1, atari-dlc2, making-karateka, garbage-pail-kids, jeff-minter, blizzard-arcade]
  -h, --help       Print help information
  -V, --version    Print version information
```
Example usage:
```
cowabunga64.exe -k cowabunga assets.pie assets.zip
cowabunga64.exe -k cowabunga assets.zip assets.pie
```
