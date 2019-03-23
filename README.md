# rusty_elma

//////////////////////////////////////// Note on Organization //////////////////////////
Please note the following organization. For grading purposes I have left in the goals and milestones previously written, but you will find them all the way at the bottom of this README.

//////////////////////////////////// An intro to Rusty Elma /////////////////////////////
Rusty Elma aims to be a port of the ELMA project written in C++ into the Rust programming language. ELMA is a task management system developed by klavins (see the following link).
https://github.com/klavins/ECEP520
Rusty Elma aims to be a management system wherein one can create different processes, data channels, and events. In the management system of Elma, one can then have all of these sub-systems talk to each other and coexist.

//////////////////////////////////// Rust directory explanation /////////////////////////////


//////////////////////////// Rust Learnings and Explanation /////////////////////////////


///////////////////////////////// Recommended Resources /////////////////////////////
-	The book, "Programming Rust: Fast, Safe Systems Development" was essential for me. I would not have been able to do this project without sitting down and reading this book (As of the project deadline I am about 200 pages in).
-	https://doc.rust-lang.org/std/
The rust crate documentation is amazing.
-	https://blog.jawg.io/docker-multi-stage-build/
This helped me get off the ground in using Rust with Docker
-	https://facility9.com/2016/05/writing-documentation-in-rust/
This was a nice simple intro to the Rustdocs comments system in rust
-	https://doc.rust-lang.org/book/index.html
This online documentation of Rust is superb
-	http://squidarth.com/rc/rust/2018/05/31/rust-borrowing-and-ownership.html
Found this one very nice post on ownership and the pointers in Rust


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


Finalized Final Project -
/////////////////////////////// Unfinished Milestones ////////////////////////////////////////
1. Implement events in the rust version of Elma.
2. Use the emit and watch functionality of Elma written in Rust to test the new event system

/////////////////////////////// Comments on Goals Achieved ////////////////////////////
Throughout this project I thought I would finish up through the events system with emit and watch in Rusty Elma. I was a little over ambitious in my goals, but I am still very pleased with what I got done.
The issue, was that for me personally there was a larger learning curve to Rust than anticipated. My initial thoughts were that I would jump in and it would be enough like C and C++ to just plow out the code. As it turns out, though there are many similarities to the languages in the syntax, the overarching rules and design stratagems are completely different. As a consequence, I wrote out the channel system, process system, and manager system in isolation, then when I went to integrate them they needed a full redesign. The issue is that Elma relies on sharing information between channels, processes, and manager through pointers to the classes. In Rust, while possible, this is not good Rust practice. You can believe me, I tried with all my might to break the compiler. I sincerely attempted to create unsafe C pointers, declare them as reference counters, assign them to other objects dynamically, pass them into other functions, and dereference them there. The compiler still caught my unsafe practices and prevented me from doing what I needed. Then I redesigned.
It took me two full redesigns and a lot more research into Rust to piece together an appropriate way to store the processes and channels within the manager using the box system. Then lending information between sub-systems became reasonable. This, along with plenty of other small learnings in the language surprised me.
In the end there are two large takeaways for me. The first is that one should come into learning a new language paying more attention to the paradigms set forth for that language. The second is that I love Rust. It has all the benefits of C, with the addition of some major improvements in C++, but alleviates all the huge issues with both languages. I can already say that through doing Rust it has made me think about how I write C on a daily basis.
Thought I did not manage to get as far as intended, I learned far more about the language than anticipated and I very much like it. In addition, while I will first allow the grading process to occur, I will likely continue iterating on this project just because I think its fun. I eventually want to learn enough Rust to use it in embedded systems, which requires I do far more work into learning how to use external system, unsafe systems, how to integrate interrupts, etc.
