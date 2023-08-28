# seed-encoder

A [BIP-39](https://github.com/bitcoin/bips/blob/master/bip-0039.mediawiki) seed phrase word encoder and decoder for alphabetic and numeric encoding schemes employed by steel plates. Can be built into wallets to support encoding and decoding words directly instead of using an intermediary transcription sheet.

Tested:

- Alphabetic - https://codl.co/products/punchplate-trade-v2-24-words
- Numeric - https://codl.co/products/new-punchplate-trade-12-24-words

The Punchplate alphabetic encoding uses the first four letters of the mnemonic. The numeric encoding assigns a number to each word which can be found in a lookup table.
