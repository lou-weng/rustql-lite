# Dev-notes for rust project

## String Operations
* input strings often have trailing new lines (remember to trim)

* String vs str
  * str types are references to strings stored in binary
    * immutable since it's a fixed sized sequence of UTF-8 bytes
    * you can access a view of the string by looking at its pointer (&str)
  * String types can be dynamically sized and are stored in the heap
    * String variables store a pointer to the heap
    * storing/accessing str in stack is faster than storing String to heap

## Ownership Rules
* each value in rust has a variable that is its owner
* there can only be one owner at a time
* when an owner is out of scope, value is dropped

* move: change of ownership
    ```
        let s1: String = String::from("Hello")
        let s2: String = s1

        // ERR: Rust would invalidate s1 if you try to use it
        //  call s1.clone() if you want to copy the entire string into another heap location
        //  primitive types like integer have an attribute called copy instead of move
    ```

* ownership example
    ```
        let s1: String = String::from("Hello")
        take_ownership(s1);
        println!("{}", s1);

        // ERR: s1's ownership is moved to the s variable in take_ownership. s1 cannot be referenced afterward

        ...
        fn take_ownership(s: String) {
            println!("{}", s);
        }        
    ```

    ```
        let s1: String = String::from("Hello")
        let s2: String = take_and_give_back_ownership(s1);
        println!("{}", s2);

        // VALID: since s2 now has ownership

        fn take_and_give_back_ownership(s: String) -> String {
            return s;
        }
    ```

* borrowing: references are used because you don't take ownership of the passed in value!
    ```
        let s1: String = String::from("Hello");
        println("{}", s1);

        // VALID: ownership is not taken by the function variable

        fn no_take_ownership(s: &String) {
            println("{}", s);
        }
    ```
    * references are immutable!
    * how do you modify a value without taking ownership?

* mutable reference
    ```
       let mut s1: String = String::from("Hello");
       change(&mut s1)

       fn change(s: &mut String) {
        s.push_str(", world");
       } 
    ```
    * you can only have one mutable reference for a value within a scope. Cannot exist if an immutable reference currently owns a value
