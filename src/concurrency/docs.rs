/*
Concurrency and parallelism are considered of these context in the rust book:

    - How to create threads to run multiple pieces of code at the same time
    - Message-passing concurrency, where channels send messages between threads
    - Shared-state concurrency, where multiple threads have access to some piece of data
    - The Sync and Send traits, which extend Rust’s concurrency guarantees to user-defined types as
    well as types provided by the standard library

*/

/*

The green-threading M:N model requires a larger language runtime to manage threads.
As such, the Rust standard library only provides an implementation of 1:1 threading.
Because Rust is such a low-level language, there are crates that implement M:N threading if you would
rather trade overhead for aspects such as more control over which threads run when and lower costs
of context switching, for example.

*/

/*
The receiving end of a channel has two useful methods: recv and try_recv. We’re using recv,
short for receive, which will block the main thread’s execution and wait until a value is sent
down the channel.

The try_recv method doesn’t block, but will instead return a Result<T, E> immediately: an Ok value
holding a message if one is available and an Err value if there aren’t any messages this time.

Using try_recv is useful if this thread has other work to do while waiting for messages: we could
write a loop that calls try_recv every so often, handles a message if one is available, and otherwise
does other work for a little while until checking again.
*/

/*
MUTEX

Mutex is an abbreviation for mutual exclusion, as in, a mutex allows only one thread to access some
data at any given time. To access the data in a mutex, a thread must first signal that it wants
access by asking to acquire the mutex’s lock. The lock is a data structure that is part of the mutex
that keeps track of who currently has exclusive access to the data. Therefore, the mutex is described
as guarding the data it holds via the locking system.

Mutexes have a reputation for being difficult to use because you have to remember two rules:

    You must attempt to acquire the lock before using the data.
    When you’re done with the data that the mutex guards, you must unlock the data so other threads can acquire the lock.
*/

/*
Arc<T>
Fortunately, Arc<T> is a type like Rc<T> that is safe to use in concurrent situations. The a stands
for atomic, meaning it’s an atomically reference counted type.

You might then wonder why all primitive types aren’t atomic and why standard library types aren’t
implemented to use Arc<T> by default. The reason is that thread safety comes with a performance penalty
that you only want to pay when you really need to. If you’re just performing operations on values
within a single thread, your code can run faster if it doesn’t have to enforce the guarantees
atomics provide.
*/

/*
RefCell / Rc / Mutex / Arc
Another detail to note is that Rust can’t protect you from all kinds of logic errors when you use Mutex<T>.
Recall in Chapter 15 that using Rc<T> came with the risk of creating reference cycles, where two Rc<T>
values refer to each other, causing memory leaks.

Similarly, Mutex<T> comes with the risk of creating deadlocks.
These occur when an operation needs to lock two resources and two threads have each acquired one of the locks,
causing them to wait for each other forever. If you’re interested in deadlocks, try creating a Rust
program that has a deadlock; then research deadlock mitigation strategies for mutexes in any language
and have a go at implementing them in Rust. The standard library API documentation for Mutex<T> and
MutexGuard offers useful information.

*/

/*
Note for Send trait for Rc Types:
Almost every Rust type is Send, but there are some exceptions, including Rc<T>: this cannot be Send
because if you cloned an Rc<T> value and tried to transfer ownership of the clone to another thread,
both threads might update the reference count at the same time.

For this reason, Rc<T> is implemented for use in single-threaded situations where you don’t want to
pay the thread-safe performance penalty.
Therefore, Rust’s type system and trait bounds ensure that you can never accidentally send an Rc<T> value across threads unsafely.

Almost all primitive types are Send, aside from raw pointers, is discussed in Chapter 19.
The RefCell<T> type (which we talked about in Chapter 15) and the family of related Cell<T> types are not Sync

# Implementing Send and Sync Manually Is Unsafe
Because types that are made up of Send and Sync traits are automatically also Send and Sync,
we don’t have to implement those traits manually.
*/