## Extracting the Key Manually

If no decryption key exists for your game, you can extract it manually using a hex editor. This tutorial uses **HxD** on Windows.

### Step 1.1: Search for the Key

1. Open the game executable file (e.g., `MMPR_RR.exe`) in HxD (or any hex editor of your choice).
2. Search for the following hex pattern:

   `50 C7 87 E8`

3. Locate the sequence in the file. After this sequence, you will see three null bytes (`00 00 00`), making the full sequence:

   `50 C7 87 E8 00 00 00`

4. The decryption key is located in the four bytes immediately following this sequence.

   Example: If the bytes after are `3B 89 5E FA`, these represent the key.

5. After the key, you will find the hex sequence `48 85 DB 74`. The key is sandwiched between these two patterns.

---

### Step 1.2: Change the Endianness

To use the key, you need to reverse the order of its bytes. For example:

- Original bytes: `3B 89 5E FA`
- Reversed bytes: `FA 5E 89 3B`

Convert the reversed bytes into the proper format by prefixing it with `0x`. For example:

- Final key: `0xFA5E893B`

## Notes

This method was discovered while attempting to decrypt **Mighty Morphin Power Rangers: Rita's Rewind** when no pre-existing decryption tool was available. Use these instructions for any other Digital Eclipse game with a similar file structure.

Happy decrypting!

