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
  -k, --key <KEY>  			    Game key for decryption/encryption [default: cowabunga]
  -c, --custom <CUSTOM_KEY> Use a custom key (in hexadecimal, e.g., '0xC90CA066')
  -h, --help       			    Print help information
  -V, --version    		    	Print version information
```
Possible Keys:
```
cowabunga
atari
atari-dlc1
atari-dlc2
making-karateka
garbage-pail-kids
jeff-minter
blizzard-arcade
mighty-morphin
```
The key for Tetris Forever is the same as `jeff-minter`.

Example usage:
```
cowabunga64.exe -k cowabunga assets.pie assets.zip
cowabunga64.exe -k cowabunga assets.zip assets.pie
cowabunga64.exe -c 0xFA5E893B assets.pie assets.zip
```
