# Feather
Creates images, demonstrating dominant colors out of e-books, analyzes text of an e-book.

Usage:

    feather [FLAGS] <INPUT> <NAME>
    
    FLAGS:
    
    -b, --bot        Format output for usage with bot
    -h, --help       Prints help information
    -V, --version    Prints version information
    -a, --analyze    Analyzing the text of an e-book
    -i, --image      Create a picture, which represent dominant colors mentioned in an e-book

ARGS:
    
    <INPUT>    Sets path to the e-book file
    <NAME>     Sets output file name
    
Example command:

    ./feather -i /Users/lesterrry/Downloads/Roberts_G._Shantaram2._Ten_Goryi.txt Шантарам

Mind that only .txt CP1251-encoded files with Russian text and total word count over 10000 are supported.

It's recommended to remove prefaces and epilogues for better visualization.

# Output examples:
![alt text](https://github.com/Fetch-Development/Feather/blob/master/Examples/IMAGE%202020-05-30%2013:56:01.jpg)
![alt text](https://github.com/Fetch-Development/Feather/blob/master/Examples/feather_out_172610.png)
![alt text](https://github.com/Fetch-Development/Feather/blob/master/Examples/feather_out_361342.png)
