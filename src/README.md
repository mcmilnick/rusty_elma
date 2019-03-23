# Source files

## basic process
A basic process is one which doesn't have any special fields or do anything very specific. These are useful for quick testing.

The implementation of a basic process contains getters and setters for most of the fields in the BasicProcess struct. The only
field which is read only is the name. The update function given is very basic and takes care of the timing updates necessary to
keep the manager updating the process on a regular basis.

## channel
All ELMA channels are meant to have the same fields. For this reason, all Channel fields are described in
the Channel struct.
 
The implementation of Channel shares fields from the pub struct Channel. This means that when a channel object
is created the object contains fields from the struct and one may operate on the object using the impl functions.
The given functions allow for full read/write access to the _capacity and _queue, but the _name is read only.

## lib
The lib collects the following modules as they are used for elma. They are compiled into a library file.
This elma library can then be included elsewhere such as it is in the test /test directory.


## manager
The manager struct contains data which will interact with the processes and channels given to it. This struct will gain ownership of the data given to it and this will be stored in the _processes and _channels fields. These fields can then lend ownership to subfunctions within the manager, but can not give up ownership.

The implementation of the manager contains knowledge of how and when to operate on the given processes and channels owned by the manager. The schedule fn is how to pass ownership of processes to the manager and the add_channel fn gives ownership of channels to the manager. The user passes a run time to the manager, which then sits in a tight loop until this amount of time has passed. After general setup, using the run fn will start and continue updates until finally calling the stop fn. The main update process occuring in the run loop looks for the scheduled timing of individual processes and calls their corresponding update fn when appropriate. These sub processes also interact with different channels so it is important to lend ownership of the channels to the process update channels when calling them.

## process
Note that the base implementation a process in Rust is through a trait. This is done to enforce certain rules for all instantiations of a process. Ensure that all processes created utilize the process trait.
Functions given here are a mix of just a signature or may include the functionality. All functions in this trait are available to use in the implementation of the trait, but one also has the opportunity to shadow any given function given below in the implementation as long as it doesn't conflict with the fn signature. This means all given functions in this trait are the equivalent of a  virtual function in C++

Of important note, if there is only a signature, that function needs expansion in the implementation. If the functionality is called out below, then it doesn't need reimplementing in the implementation unless you want to overshadow it. We have thus provided definete rules for all processes.

## receiver process
The reciever looks for data in a data channel given to the manager. If there is data in the channel, it looks at all of it and finds the sum of all of it.

## sender process
Every round in the update, the sender sends new data to the data channel for consumption by other processes.

## stopwatch
The stopwatch structure is for use with the stopwatch implementation only. This was created mainly for learning rust and as a nice way to debug the other portions of the manager as I was creating them.

The functions should resemble buttons on a stopwatch for the most part. The start, stop, and reset all do what they imply to the total time counter. The other fn are used for getting the current time in different units. 
