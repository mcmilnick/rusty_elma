# rusty_elma

///////////////////////////////////////// Goals /////////////////////////////////////////

This repo aims to port the ELMA project into the programming language Rust. ELMA is a task management system developed by klavins (see the following link).
https://github.com/klavins/ECEP520
The ELMA manager aims to manage different processes for an embedded system. The goal of this project is to recreate this system using Rust.
For this project, I will be porting over the manager, process system, events and state machine. The functionality should be the same with the hope that the code is cleaner since Rust is supossed to remove some of the less friendly functionality of C and C++.
  To achieve this goal, I will need to learn Rust from scratch. The book, "Programming Rust: Fast, Safe Systems Development" should be of great help in this task. This will purely be a task in understanding the differences from C, C++, and Rust. There are many differences in memory management and task managing. To get some examples of how to properly write Rust and the style guidelines to follow I will also use the Rust github shown below...
https://github.com/rust-lang
While porting over the different pieces of ELMA, I will be writing test cases and examples to prove the functionality and give an idea how to use this new version of ELMA.  
  
 Week 1 of final project -
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


Week 2 of final project -
////////////////////////////// Finished goals //////////////////////////////////
This week I accomplished a couple things.
- I did a lot of studying of Rust. This took quite a bit of time given it is a new language. I made it through quite a bit of the Programming rust book I mentioned before, which allowed me to get a good running start at the code.
- I got the channels working for ELMA in Rust. I forgot to take this into account in the milestones, so I should have made time for that first. I am not fully done with the testing of this functionality and it works as expected.
- I have begun implementing the Processes and Manager of ELMA, but still need to test them.
- I figured out the proper way to test code in Rust

////////////////////////////Working Code ////////////////////////////////////
To see the current working code you can navigate into src/main.rs
The main file contains a general stopwatch module and tests for all the other pieces of ELMA. I have properly implemented the test for channels which can be seen as...
#[test]
fn test_channel()
In Rust, all functions marked with the test macro are run as tests when you run "cargo test". This testing tests all the basic functions.

/////////////////////////////// Adjustments to goals /////////////////////////////
With these things in mind, I still think the milestones given are achievable though the dates are skewed. I did not have as much time last week as I hoped I would, but that was mainly because it took me extra time to learn the language. I now feel more confident in the language, however, and think most of the rest can be done in short order. I am thinking if anything of dropping the state machine in order to spend a good amount of time properly documenting using Rust and Github.

/////////////////////////////// Remaining Milestones ////////////////////////////////////////
1. Implement and test Rust implementation of ELMA processes.
Deadline - 3/17/2019
2. Implement and test Rust implementation of ELMA Manager.
Deadline - 3/17/2019
3. Recreate ELMA example of a car wuth cruise control. This will control a cars speed as the desired speed is changed.
Deadline - 3/18/2019
4. Implement events in the rust version of Elma.
Deadline - 3/19/2019
5. Use the emit and watch functionality of Elma written in Rust to test the new event system
Deadline - 3/19/2019
6. Clean doxygen documentation and publish it as the github pages.
Deadline - 3/20/2019
7. Finalize and publish project
Deadline - 3/21/2019
