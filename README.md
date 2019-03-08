# rusty_elma

///////////////////////////////////////// Goals /////////////////////////////////////////

This repo aims to port the ELMA project into the programming language Rust. ELMA is a task management system developed by klavins (see the following link).
https://github.com/klavins/ECEP520
The ELMA manager aims to manage different processes for an embedded system. The goal of this project is to recreate this system using Rust.
For this project, I will be porting over the manager, process system, events and state machine. The functionality should be the same with the hope that the code is cleaner since Rust is supossed to remove some of the less friendly functionality of C and C++.
  To achieve this goal, I will need to learn Rust from scratch. The book, "Programming Rust: Fast, Safe Systems Development" should be of great help in this task. This will purely be a task in understanding the differences from C, C++, and Rust. There are many differences in memory management and task managing. To get some examples of how to properly write Rust and the style guidelines to follow I will also use the Rust github shown below...
https://github.com/rust-lang
While porting over the different pieces of ELMA, I will be writing test cases and examples to prove the functionality and give an idea how to use this new version of ELMA.  
  
///////////////////////////////////////// Milestones /////////////////////////////////////////
1. Study Rust equivalents from C and C++. I aim to find the apprioriate ways to handle memory, do basic print debug, and what the structure types are in Rust.
Deadline - 3/9/2019
2. Implement and test Rust implementation of ELMA processes.
Deadline - 3/10/2019
3. Implement and test Rust implementation of ELMA Manager.
Deadline - 3/11/2019
4. Recreate ELMA example of a car wuth cruise control. This will control a cars speed as the desired speed is changed.
Deadline - 3/11/2019
5. Implement events in the rust version of Elma.
Deadline - 3/16/2019
6. Use the emit and watch functionality of Elma written in Rust to test the new event system
Deadline - 3/16/2019
7. Create the Rust equivalent of a state machine class
Deadline - 3/18/2019
8. Create a new state machine in Rust to model a microwave.
Deadline - 3/18/2019
9. Clean doxygen documentation and publish it as the github pages.
Deadline - 3/20/2019
10. Finalize and publish project
Deadline - 3/21/2019
