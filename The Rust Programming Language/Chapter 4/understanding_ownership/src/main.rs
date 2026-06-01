fn main() {
    // Ownership is a set of rules that govern how a Rust program manages memory.
    // This creates safe and efficient code
    //
    // THE STACK AND THE HEAP
    // The stack stores values in the order received, then removes the values in the opposite order
    // Last in, first out (LIFO)
    // Adding data is called pushing to the stack, removing is popping off the stack
    // The data must have a known fixed size, else it will be stored in the heap
    // The heap is less organized, when requesting space, the memory allocator finds an empty spot,
    // marks it as being used and returns a pointer(address of the location): proccess is called
    // allocating on the heap. The pointer is of fixed size and knows so it gets added to the stack
    // 
    // Pushing to the stack is faster than allocating on the heap because it never has to search
    // for a place.
    //
    // OWNERSHIP RULES
    // Each value in Rust has an owner
    // There can only be one owner at a time
    // When the owner goes out of scope, the value will be dropped.
    //
    // VARIABLE SCOPE
}
