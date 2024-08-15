# rustBookProjects
CLI project and multithreaded web server project

### simple_command_line_tool
minigrep, an imitation of grep tool

concepts covered

- practice of rust std library--error,fs,env,process
- modularity, separation of logic and error handling between main.rs and lib.rs
- usage of struct and implementation with error handling using a Result<T,E> output
- incorporation of references and lifetimes for arguments passed and outputs generated from the struct implementation

guided by <a href="https://doc.rust-lang.org/book/ch12-00-an-io-project.html"> I/O Project from the Rust Book </a>

### multithreaded_web_server
- incorporation of mpsc, arc, mutex
- implementation of drop in threadpool that contains a limit of pending tasks
- use of channels to receiving end that consumes
- used atomic reference counting combined with mutex to safely share the receiving end of the pool of threads; ensures only one worker owns a thread at a time
