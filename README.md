# ps05-thegrep-shannonandsarah
## Tar Heelegrep-thegrep
### PS05, COMP 590 Little Languages

### DESIGN DECISIONS
For **thegrep,** we decided to base our design and structure strictly off of the given grammar for the language. We kept the tokenizing functionality in one file, and organized it into several methods and sections. The parsing took place in another file, again, broken up into several methods and helper methods. Parsing and tokenizing were called in main.rs when the appropriate flags were signaled by the user during **cargo run**. 
Our recursive design in **parser.rs** started with the input in **regexpr(),** which called catenation methods, which then called closure methods, which then called atom methods. The recursion occurred when looking for catenations in an alternation, and other atoms in a catenation.

### NOTES FOR GRADERS
We discussed variable names and all the ways that we could name them or name functions, and we ultimately decided to name them as close to the grammar as we could. 

### COLLABORATION NOTES
We used the **driver/navigator** setup to code our project. Before we started, though, we sat down and drew out a layout of how the pieces of our program should connect, and how we should implement them in code. 
We swapped driver/navigator roles often (sometimes not swapping laptops, though, so our commit messages do not always reflect who was the driver at the time of the commit).
From pair programming, we learned the effectiveness of having another person to bounce ideas off of, and to check your logic as you are writing it. We found that we were much more efficient working together than either of us normally are working alone. 
We had conflicting ideas about some functionality, especially figuring out the grammar, we tried both ideas, gave them equal chances to succeed, and ultimately chose the more efficient one in the end.
We also took breaks together, and never coded without the other person, which was beneficial because then we were always on the same page.

### SARAH'S FOCUS
I started by leading the creation of our overall framework for the files and methods within the files, modeling a lot of the structure after previous problem sets, specifically thbc.
I was able to get better at reading command-line arguments, and actually creating the code that processed them.
Then, I helped brainstorm ideas for the parser, and how we wanted to structure our recursion based on the given grammar from the writeup. 
In parser, I was the navigator more than I was the driver.

### SHANNON'S FOCUS
I led the overall structure of main.rs, where I figured out the flags (and realized how to use structopt built-in functionality instead of hard-coding flags) and got the overall structure of main working so that our program compiled and ran when we typed **cargo run**.
I also helped figure out the grammar for the parser, where I was the driver a bit more than I was the navigator. 
Also, I had a little more experience with Github, so I was able to help resolve a merge conflict that we had. 
Finally, I helped think through the logic of recursion and get our overarching ideas of parser together.



:nail_care:
