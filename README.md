# Anew

Copy and reuse common default templates for files and directories.

    (anew --help goes here)

Example usage:

    mkdir latex_template
    cd latex_template

    echo "\documentclass{article}" >> main.tex
    echo "\begin{document}" >> main.tex
    echo "Hi, there!" >> main.tex
    echo "\end{document}" >> main.tex

    anew create latex ./* # or: anew create latex

This copies `main.tex` into `$HOME/.anew/templates/latex/`.

After setting up the "latex" template, you can initialize a directory like so:

    mkdir my_new_project
    cd my_new_project
    anew latex . # or: anew latex