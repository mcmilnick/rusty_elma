# basic process
/// # Arguments
/// 
/// * `_period` - The period at which the manager should call the update function of the process.
/// * `_previous_update` - This contains the the time that the previous update occured as is used to measure when the next update should occur in the manager.
/// * `_last_update` - Contains the time when the latest update happened. 
/// * `_start_time` - Contains the time when the start function was called for the process.
/// * `_name` - This contains the name of the process.
/// * `_num_updates` - A counter for the number of times the process has been updated since instantiation. 
/// * `_status` - Provides an unsigned byte showing the current state of the process. This enum is defined in the process file next to the trait. 
/// 
/// # Remarks
/// 
/// A basic process is one which doesn't have any special fields or do anything very specific. These are useful for quick testing.


/// # Remarks
/// 
/// The implementation of a basic channel contains getters and setters for most of the fields in the BasicProcess struct. The only
/// field which is read only is the name. The update function given is very basic and takes care of the timing updates necessary to
/// keep the manager updating the process on a regular basis.

# channel
/// # Arguments
/// 
/// * `_name` - A string holding the name of the channel instantiation.
/// * `_capacity` - This contains the total capacity of the channel.
/// * `_queue` - A queue where all data is placed in and read out from the channel. 
/// 
/// # Remarks
/// 
/// All ELMA channels are meant to have the same fields. For this reason, all Channel fields are described in
/// the Channel struct.



/// # Remarks
/// 
/// The implementation of Channel shares fields from the pub struct Channel. This means that when a channel object
/// is created the object contains fields from the struct and one may operate on the object using the impl functions.
/// The given functions allow for full read/write access to the _capacity and _queue, but the _name is read only.

# lib
/// # Remarks
/// The lib collects the following modules as they are used for elma. They are compiled into a library file.
/// This elma library can then be included elsewhere such as it is in the test /test directory.


# manager
/// # Arguments
/// 
/// * `_processes` - When a process is created and then scheduled, the ownership is transferred to the manager and the process is stored in this vector.
/// * `_channels` - When a channel is created and then added to the manager it is stored in this vector.
/// * `_start_time` - This contains the time at which the manager was started. 
/// * `_elapsed` - This field contains the amount of time which has passed since the manager was started.

/// # Remarks
/// 
/// The manager struct contains data which will interact with the processes and channels given to it. This struct will gain ownership of the data given to it
/// and this will be stored in the _processes and _channels fields. These fields can then lend ownership to subfunctions within the manager, but can not
/// give up ownership.



/// # Remarks
/// 
/// The implementation of the manager contains knowledge of how and when to operate on the given processes and channels owned by the manager.
/// The schedule fn is how to pass ownership of processes to the manager and the add_channel fn gives ownership of channels to the manager.
/// The user passes a run time to the manager, which then sits in a tight loop until this amount of time has passed. After general setup, using the
/// run fn will start and continue updates until finally calling the stop fn.
/// The main update process occuring in the run loop looks for the scheduled timing of individual processes and calls their corresponding update fn when
/// appropriate. These sub processes also interact with different channels so it is important to lend ownership of the channels to the process update channels
/// when calling them.

# process
/// # Remarks
/// 
/// Note that the base implementation a process in Rust is through a trait. This is done to enforce certain rules
/// for all instantiations of a process. Ensure that all processes created utilize the process trait.
/// 
/// Functions given here are a mix of just a signature or may include the functionality. All functions in this trait are
/// available to use in the implementation of the trait, but one also has the opportunity to shadow any given function given
/// below in the implementation as long as it doesn't conflict with the fn signature. This means all given functions in this trait
/// are the equivalent of a  virtual function in C++
/// 
/// Of important note, if there is only a signature, that function needs expansion in the implementer. If the functionality is
/// called out below, then it doesn't need reimplementing in the implementation unless you want to overshadow it. We have thus provided
/// definete rules for all processes.

# receiver process
/// # Arguments
/// 
/// * `_sum` - Grabs the sum of the data in a channel added to the manager.
/// * `_period` - The period at which the manager should call the update function of the process.
/// * `_previous_update` - This contains the the time that the previous update occured as is used to measure when the next update should occur in the manager.
/// * `_last_update` - Contains the time when the latest update happened. 
/// * `_start_time` - Contains the time when the start function was called for the process.
/// * `_name` - This contains the name of the process.
/// * `_num_updates` - A counter for the number of times the process has been updated since instantiation. 
/// * `_status` - Provides an unsigned byte showing the current state of the process. This enum is defined in the process file next to the trait. 
/// 
/// # Remarks
/// 
/// The reciever looks for data in a data channel given to the manager. If there is data in the channel, it looks at all of it and
/// finds the sum of all of it.



/// # Remarks
/// 
/// The reciever looks for data in a data channel given to the manager. If there is data in the channel, it looks at all of it and 
/// finds the sum of all of it.

# sender process
/// # Arguments
/// 
/// * `_idx` - Contains the current position in the data channel to know how many data points have been put in up to that point.
/// * `_data` - This is a vector onto which the sender can push data for consumption by other processes.
/// * `_period` - The period at which the manager should call the update function of the process.
/// * `_previous_update` - This contains the the time that the previous update occured as is used to measure when the next update should occur in the manager.
/// * `_last_update` - Contains the time when the latest update happened. 
/// * `_start_time` - Contains the time when the start function was called for the process.
/// * `_name` - This contains the name of the process.
/// * `_num_updates` - A counter for the number of times the process has been updated since instantiation. 
/// * `_status` - Provides an unsigned byte showing the current state of the process. This enum is defined in the process file next to the trait. 
/// 
/// # Remarks
/// 
/// Every round in the update, the sender sends new data to the data channel for consumption by other processes.



/// # Remarks
/// 
/// Every round in the update, the sender sends new data to the data channel for consumption by other processes.

# stopwatch
/// # Arguments
/// 
/// * `sw_start_time` - Contains the time at which the start 'button' is hit on the stopwatch.
/// * `sw_stop_time` - Contains the time time at which the stop 'button' is hit on the stopwatch.
/// * `sw_total_time` - This is the total time between all start and stop sequences. This is reset when the reset 'button' is hit.
/// If one continues to hit start after stop continuously then the time will add into total time. 

/// # Remarks
/// 
/// The stopwatch structure is for use with the stopwatch implementation only. This was created mainly for learning rust and as a
/// nice way to debug the other portions of the manager as I was creating them.



/// # Remarks
/// 
/// The fn should resemble buttons on a stopwatch for the most part. The start, stop, and reset all do what they imply to the total time counter.
/// The other fn are used for getting the current time in different units. 

