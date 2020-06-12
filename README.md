# Building
To build the interpreter from source, open a command prompt in the root folder of the project, and execute the following command:

    cargo build --release
The built binary should be in ./target/release/.
# Executing code
To execute code with this interpreter, simply call the binary with 1 argument, which is the file name of the source.

    inxanelang <filename>.inx
The extension doesn't have to be .inx, it's just what I chose to save the code as.
# InxaneLang
InxaneLang is not supposed to be a langauge for mainstream use. It's a language made mainly for fun, and for practicing my programming skills.
The language works with instructions, separated by a new line.
Each instruction can have arguments, separated by spaces.

The Hello World in InxaneLang looks like this:

    PRINT Hello World!
Yes, it's that simple.
The program also has a stack, which is basically an array of integers. Values can be pushed (put on the stack) and popped (removed from the stack). The stack allows for basic arithmetic, for example adding numbers together.

    PUSH 5
    ADD 1
    POPPRINT
This program pushes 5 onto the stack, adds 1 to the top value on the stack, and popprints (pops and prints the popped value).
The output should be 6.

Here is the list of all instructions:
- STOP - stops the program
- PRINT \<text\> - prints the text specified as the argument
- GOTO \<line number\> - changes the instruction pointer to the specfiied line number. This allows for basic loops.
- IF \<number\> - pops a value, if it is equal to the argument, it will execute the next line of code, if not, it will skip over it.
- PUSH \<number\> - pushes a number onto the stack
- POP - pops a value off of the stack, discarding it.
- ADD/SUB/MULT \<number\> - adds, subtracts or multiplies the top value on the stack with the specified number
- POPPRINT - pops and prints the top number on the stack
- APOPPRINT - pops and prints the top number on the stack as an ascii character
- PRINTSTACK - prints the stack in the format of [x, y, z]
- REV - reverses the order of the stack
- INPUTSTR - takes input from the user, and pushed all the ascii values of each character onto the stack
- INPUTNUM - takes input from the user, parses it as a number and pushes the result onto the stack, if it fails to parse it as a number, it will push -1 instead
- LEN - pushes the length of the stack onto the stack
- DUP - duplicates the top value on the stack
- NEWLINE - prints a new line (other prints do not print a new line at the end)