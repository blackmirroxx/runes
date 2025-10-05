# rhunes
translate text into runes on the cli. 
More specifically it uses "Elder Futhark" also known as the Old norse Alphabet. 
It was used from 500 B.C.E until 500 A.D. and got replaced by Younger Futhark, Short-Twig Futhark, 
Staveless Hälsinge Futhark, Medieval Runerow and eventually emerged to the Anglo-Saxon Futhark. 

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
ᚾᛇᚱ ᚺᚱᚲᛖᛈᛈᛖᚾ ᚢᚨᚱ ᚷᛃᛟᚱᛞᚨ ᚨᚢ ᛏᚱᛇ, ᚢᚨᚱ ᛗᛇᚾᚾᛖᚾ ᚷᛃᛟᚱᛞᚨ ᚨᚢ ᛊᛏᚨᚨᛚ.
```

3. Reverse the translation
```bash
./target/release/rhunes-translator -r "ᚾᛖᚱ ᚺᚱᚲᛖᛈᛈᛖᚾ ᚢᚨᚱ ᚷᛃᛟᚱᛞᚨ ᚨᚢ ᛏᚱᛖ, ᚢᚨᚱ ᛗᛖᚾᚾᛖᚾ ᚷᛃᛟᚱᛞᚨ ᚨᚢ ᛊᛏᚨᚨᚢᛚ."

```

This will output:
```
ner skeppen var gjorda av tre, var mennen gjorda av staavl.
```
The reverse is far from perfect but good enough XD. x is sometimes translated as ᛊ but also as ᚲᛊ.
I chose the x =  ᚲᛊ, as it is more consistent. Unfortunatelly, old scriptures, do not always follow this rule -.- 

**Features:**
- Handles all your special rules for ä, ö, ü, å, ø, æ, th, sk, sj
- Case-insensitive translation (converts everything to lowercase in Rhunes)
- Reverse translation with `-r` flag
- Proper command-line interface
- Unit tests included

The code handles the special multi-character rules by looking ahead in the character stream, and processes the text according to your specific translation rules.
