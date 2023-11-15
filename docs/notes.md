# ISO/IEC 18004 Notes

This is a collection of notes of the fourth edition ISO/IEC 18004, which can be summarized as the QR Code specification.

These notes were created with the end purpose of aiding in teh creation of an encoder, decoder, and detector in the Rust programming language.

This is not a reproduction or copy, but a collection of thoughts made while reading the specification.

## Table of Contents

> Is rendered by a VSCode plugin

[[toc]]

## Intro

There are four kinds of QR Codes(not including other similar codes such as Aztec Codes)

- QR Code Model 1
- Qr Code Model 2
- QR Code
- Micro QR Code

QR Codes are made up of a two dimensional binary sequence where each binary unit and the entire sequence itself are arranged in a square. Binary values are typically indicated by being black and while, though there are other proposed codes that use color to increase encoding density.

Each QR Code also has some patterns used for finding(when doing image processing), typically in the corners. This finder pattern aids in detecting position, size, and inclination.

Note QR Code Model 1 is not recommended to be implemented, and is not needed to meet the spec

## Symbols

### Basics

QR Codes can be defined as such:

- Format
    - QR Code
    - Micro QR Code
- Character set
    - Numeric
    - Alphanumeric
    - Byte
    - Kanji
- Boolean representation
    - typically: dark is **true**, light is **false**
- Area
    - Micro QR Codes can be 11x11 to 17x17 from version M1 to M4, each version increasing by two
    - QR Codes can be 21x21 to 177x177 from version 1 to 40, each version increasing by four
- Maximum characters per code
    - Micro QR:
        - numeric: 35
        - alphanumeric: 21
        - byte: 15
        - kanji: 9
    - QR:
        - numeric: 7089
        - alphanumeric: 4296
        - byte: 2953
        - kanji: 1817
- Error correction levels using Reed-Solomon
    - L: 7%
    - M: 15%
    - Q: 25%
    - H: 30%

### Additional Features

Features that we can implement after finishing the core implementation

- Structured Append
    - Allows for files to be read from a sequence of QR Codes. Should not be available for Micro QR Codes
- Extended Channel Interpretations
    - Allow for other character sets and data interpretation to be used. Should not be available for Micro QR Codes
- Reflectance Reversal
    - Im guessing this is means Boolean representation is switched?
- Mirror Imaging
    - Read transposed images correctly when the alignment patterns are mirrored

## Symbol Structure

Should see 6.3(pg.7 -> pg.17) for a detailed view with diagrams. The below is just a simple description.

Function Patterns:

- Quiet Zone
    - The region free of markings, surrounds the Code on all sides. Should be 4 units for QR Codes, and 2 units for Micro QR Codes
- Finder Pattern
    - QR Codes have a pattern illustrated by figure 12 in the top left, top right, and bottom left
    - Micro QR Codes have the same pattern, only in the top left
- Separator
    - Placed outside finder patterns to differentiate the pattern and the encoding region.
- Timing Patterns
    - Horizontal & Vertical 1 width patterns that allow for symbol density, version, datum position, and module coordinate to be found. See figures 3 & 4 for diagrams for QR Code and Micro QR Code diagrams
- Alignment Patterns
    - Symbols versions 2+ have a number of patterns for alignment, each pattern is made up of a alternating boolean rings in a 5x5 area, typically the outermost and innermost areas are dark

Encoding Region:

- Format Information
- Version Information
- Data And Error Correction CodeWords

See chapter 7

## Encoding

### Step Preview

1. Data analysis
2. Data Encoding
3. Error correction encoding
4. Structure Final message
5. Module placement in matrix
6. Data masking
7. Format and version information

See 7.1 (pg. 18) for an overview explaining each step
