# Anew

Copy and reuse common default templates for files and directories.

    anew 0.1.0
    Matt Torrence (github: Torrencem)
    Copy and reuse common default templates for files and directories

    USAGE:
        anew [ARGS] [SUBCOMMAND]

    FLAGS:
        -h, --help       Prints help information
        -V, --version    Prints version information

    ARGS:
        <NAME>         The name of the template to copy
        <DIRECTORY>    The directory to clone into [default: .]

    SUBCOMMANDS:
        create    Create a new template
        dir       Alias for ls
        help      Prints this message or the help of the given subcommand(s)
        ls        List available templates
        remove    Remove a template

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