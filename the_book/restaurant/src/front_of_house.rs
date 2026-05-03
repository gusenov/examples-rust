    // In Rust, all items (functions, methods, structs, enums, modules, and constants) are private to parent modules by default. 
    // If you want to make an item like a function or struct private, you put it in a module.

    // making the module public doesn’t make its contents public
    // The pub keyword on a module only lets code in its ancestor modules refer to it, not access its inner code. 
    // Because modules are containers, there’s not much we can do by only making the module public; we need to go further and choose to make one or more of the items within the module public as well.
   
    pub mod hosting;

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }