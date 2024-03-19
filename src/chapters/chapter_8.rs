pub mod closures {
    // Characteristics of closures
    // using || instead of `()` around input variables.
    // Optionaly body delimination `{}` for a single ecpression (mandatory otherwise).
    // the ability to capture the outer environment variables.

    pub fn testing() -> (i32, i32) {
        let outer_var = 42;

        let closure_annotated = |i: i32| -> i32 { i + outer_var };
        let closure_inferred = |i| i + outer_var;
        println!("closure_annoted: {}", closure_annotated(1));
        println!("closure_inferred: {}", closure_inferred(1));
        (closure_annotated(1), closure_inferred(2))
    }

    pub fn capturing() {
        use std::mem;

        let color = String::from("green");
        // A closure to print `color` which immediately borrows (`&`) `color` and
        // stores the borrow and closure in the `print` variable. It will remain
        // borrowed until `print` is used the last  time.
        //
        //  `println!` only requires arguments by immutable refence so it doesn't
        // impose anything more restrictive.

        let print = || println!("{}", color);

        // call the closure using the borrow
        print();

        // `color` can be borrowed immutably again because the closure only holds
        //  an immutable reference to `color`.
        let _reborrow = &color;
        print();

        // A move or reborrow is allowed after the final use of print
        let _color_moved = color;

        let mut count = 0;
        // A clisure to increament `count` coluld take either `&mut count` or `count`
        // but `&mut count` is less restictive so it takes that. Immediately
        // borrows `count`.
        //
        // A `mut` is required on `inc` bcause a `&mut` is stored inside. Thus,
        // calling the closure mutate the closure which requires a `mut`.
        let mut inc = || {
            count += 1;
            println!("`count`: {}", count);
        };

        // Call the closure using a mutable borrow.
        inc();

        // The closure still mutably borrows `count` because it is called latter.
        // An attempt to reborrow will lead to an error.
        // let _reborrow = &count;
        // ^ uncomment to see
        inc();

        // The closure no longer needs to borrow `&mut count`. Therefore, it is
        // posible to reborrow without an error
        let _count_reborrowed = &mut count;

        // A non-copy type.
        let movable = Box::new(3);

        // `mem::drop` requires  `T` so this must take by value. A copy type
        // would copy into the closure leaving the original untouched.
        // A non-copy must move anf so `movable` immediately moves into
        // the closure.

        let consume = || {
            println!("`movable`: {:?}", movable);
            mem::drop(movable)
        };

        // `consume` consumes the variables so this can only be called once.
        consume();

        // Using `move` before vertical pipes forces closure to take ownership of captured variables:

        let haystack = vec![1, 2, 3];

        let contains = move |needle| haystack.contains(needle);

        println!("{}", contains(&1));
        println!("{}", contains(&4));

        // println!("There're {} elements in vec", haystack.len());
        // ^ Uncommenting above line will result in compile-time error
        // because borrow checker doesn't allow re-using variable after it
        // has been moved.

        // Removing `move` from closure's signature will cause closure
        // to borrow _haystack_ variable immutably, hence _haystack_ is still
        // available and uncommenting above line will not cause an error.
    }

    pub mod closure_as_parameter {
        // Fn: the closure uses the captured value by reference (&T)
        // FnMut: the closure uses the captured value by mutable reference (&mut T)
        // FnOnce: the closure uses the captured value by value (T)

        fn apply<F>(f: F)
        where
        // The closure takes no input and return  nothing
        F: FnOnce() {
            f();
        }

        // A function which takes a closure and returns an `i32`.
        fn apply_to_3<F>(f: F) -> i32 where 
        // the closure that taked an i32 and returns an `i32`.
        F: Fn(i32) -> i32 {
            f(3)
        }

        pub fn closure_as_parameter() {
            use std::mem;

            let greeting = "Hello";
            // A non type.
            // `to_owned` creates owned data from from borrowed one
            let mut farewell = "goodbye".to_owned();

            // capture 2 variabled: `greeting` by reference and 'farewell'
            // by value

            let diary = || {
                // `greeting` is by refence: requires `Fn`.
                println!("I said {}.", greeting);

                // Mutation forces `farewell` to be captured by 
                // Mutable reference. Now requires `FnMut`.
                farewell.push_str("!!!");
                println!("Then I screamed {}.", farewell);
                println!("Now I can sleep. zzzzz");

                // manually calling drop forces `farewell` to 
                // be captured by value. Now requires `FnOnce`.
                mem::drop(farewell);
            };

            // Call the function which applys the closure.
            apply(diary);

            // `double` satisfies `apply_to_3` 's trait bound
            let double = |x| 2 * x;

            println!("3 Doubled: {}", apply_to_3(double))

        }
    }
}

#[cfg(test)]
mod test {
    use super::closures::testing;

    #[test]
    fn test_one() {
        let (one, two) = testing();
        assert_eq!(one, 43);
        assert_eq!(two, 44);
    }
}
