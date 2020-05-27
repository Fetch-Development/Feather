# Feather
Creates images, demonstrating dominant colors out of e-books, analyzes text of an e-book.

Usage:

    feather <INPUT>
    
    FLAGS:
    
    -h, --help       Prints help information
    
    -V, --version    Prints version information
    
    -a, --analyze    Analyzing the text of an e-book

    -i, --image      Create a picture, which represent dominant colors mentioned in an e-book

ARGS:
    
    <INPUT>    Sets path to the e-book file


Mind that only .txt files with Russian text and total word count over 10000 are supported.

It's recommended to remove prefaces and epilogues for better visualization.
