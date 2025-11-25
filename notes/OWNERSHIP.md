# Ownership

[What Is Ownership?](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)

* Both the stack and the heap are parts of memory available to your code to use at runtime.
* The main purpose of ownership is to manage heap data. Understanding this can help explain why ownership works the way it does.

## The Stack

* The stack stores values in the order it gets them and removes the values in the opposite order. This is referred to as last in, first out.
* Think of a stack of plates: when you add more plates, you put them on top of the pile, and when you need a plate, you take one off the top. Adding or removing plates from the middle or bottom wouldn’t work as well! 
* Adding data is called pushing onto the stack, and removing data is called popping off the stack. 
* All data stored on the stack must have a known, fixed size.

## The Heap

* When you put data on the heap, you request a certain amount of space. The memory allocator finds an empty spot in the heap that is big enough, marks it as being in use, and returns a pointer, which is the address of that location. This process is called allocating on the heap and is sometimes abbreviated as just allocating (pushing values onto the stack is not considered allocating). 
* Because the pointer to the heap is a known, fixed size, you can store the pointer on the stack, but when you want the actual data, you must follow the pointer.
* Data with an unknown size at compile time or a size that might change must be stored on the heap.

## Ownership Rules

Keep these rules in mind:

* Each value in Rust has an owner.
* There can only be one owner at a time.
* When the owner goes out of scope, the value will be dropped.

Four general strategies can help with ownership issues:

* Use references where full ownership is not required.
    * The most common change you will make to your code is to reduce the level of access you require. Instead of requesting ownership, you can use a "borrow" (aka reference) in your function definitions. For read-only access, use `&T`. For read-write access, use `&mut T`.
        * NOTE: Adding the `&mut` prefix to a moveable parameter type (i.e. a non-copyable type) allows the calling scope to retain ownership of the data that was passed in the function call while at the same time allowing the function that was called to mutate the data that was passed to it.
    * Ownership might be needed in advanced cases, such as when functions want to adjust the lifetime of their arguments. For example, you can move the ownership of a variable into a function (by passing the variable into that function) and when the function returns, do not return the variable. This will cause the variable to be deleted and its lifetime will end there.
* Duplicate the value.
* Refactor code to reduce the number of long-lived objects.
* Wrap your data in a type designed to assist with movement issues.
