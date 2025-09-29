# rhunes
translate text into runes on the cli

Here's a Rust implementation of your Rhunes translator:


**How to use:**

1. Create a new cargo project and replace the files with the content above
2. Build and run:
```bash
cargo build --release
./target/release/rhunes-translator "När skeppen var gjorda av trä, var männen gjorda av stål."
```

This will output:
```
ᚾᛖᚱ ᚺᚱᚲᛖᛈᛈᛖᚾ ᚢᚨᚱ ᚷᛃᛟᚱᛞᚨ ᚨᚢ ᛏᚱᛖ, ᚢᚨᚱ ᛗᛖᚾᚾᛖᚾ ᚷᛃᛟᚱᛞᚨ ᚨᚢ ᛊᛏᚨᚨᚢᛚ.
```

3. Reverse the translation
```bash
./target/release/rhunes-translator -r "ᚾᛖᚱ ᚺᚱᚲᛖᛈᛈᛖᚾ ᚢᚨᚱ ᚷᛃᛟᚱᛞᚨ ᚨᚢ ᛏᚱᛖ, ᚢᚨᚱ ᛗᛖᚾᚾᛖᚾ ᚷᛃᛟᚱᛞᚨ ᚨᚢ ᛊᛏᚨᚨᚢᛚ."

```

This will output:
```
ner skeppen var gjorda av tre, var mennen gjorda av staavl.
```
The reverse is for from perfect but good enough XD

**Features:**
- Handles all your special rules for ä, ö, ü, å, ø, æ, th, sk, sj
- Case-insensitive translation (converts everything to lowercase in Rhunes)
- Reverse translation with `-r` flag
- Proper command-line interface
- Unit tests included

The code handles the special multi-character rules by looking ahead in the character stream, and processes the text according to your specific translation rules.
