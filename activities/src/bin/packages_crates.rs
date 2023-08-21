// Packages and Crates
// A crate is the smallest amount of code that Rust compiler considers at time.
// A crate can come in one of two forms: a binary crate or a library crate. Binary crates are programs you can compile to an executable that you can run, such as a CLI or a server. Each must have a function called main that defines what happens when the executable runs.

// Library crates don't have a main function, and they don't compile to an executable. Instead, they define functionality intended to be shared with multiple projects.(ex: the rand crate provides functionality that generate random numbers).

// The crate root is a source file that Rust compiler starts from and makes up the root module of your crate.

// A package is a bundle of one or more crates that provides a set of functionality. It contains a Cargo.toml file that describes how to build those crates. A package can contain as many binary crates as you like, but at most only one library crate. A package must contain at least one crate, whether that's a library or binary crate.

// Cargo follows a convention that src/main.rs is the crate root of a binary crate with the same name as the package.
// Likewise, Cargo knows that if the package directory contains src/lib.rs, the package contains a library crate with the same name as the package, and src/lib.rs is its crate root.

// MODULES CHEAT SHEET

// Start from the crate root When compiling a crate, the compiler first looks in the crate root file (usually src/lib.rs for a library crate or src/main.rs for a binary crate) for code to compile.

// Declaring modules
// In the crate root file, you can declare new modules; say you declare a “garden” module with mod garden;. The compiler will look for the module’s code in these places:
// Inline, within curly brackets that replace the semicolon following mod garden
// In the file src/garden.rs
// In the file src/garden/mod.rs

// Declaring submodules
// In any file other than the crate root, you can declare submodules. For example, you might declare mod vegetables; in src/garden.rs. The compiler will look for the submodule’s code within the directory named for the parent module in these places:
// Inline, directly following mod vegetables, within curly brackets instead of the semicolon
// In the file src/garden/vegetables.rs
// In the file src/garden/vegetables/mod.rs

// Paths to code in modules
// Once a module is part of your crate, you can refer to code in that module from anywhere else in that same crate, as long as the privacy rules allow, using the path to the code. For example, an Asparagus type in the garden vegetables module would be found at crate::garden::vegetables::Asparagus.

// Private vs. public
// Code within a module is private from its parent modules by default. To make a module public, declare it with pub mod instead of mod. To make items within a public module public as well, use pub before their declarations.

// The use keyword
// Within a scope, the use keyword creates shortcuts to items to reduce repetition of long paths. In any scope that can refer to crate::garden::vegetables::Asparagus, you can create a shortcut with use crate::garden::vegetables::Asparagus; and from then on you only need to write Asparagus to make use of that type in the scope.

//  Modules let us to control the privacy of items because code within a module is private by default. We can choose to make modules and the items within them public, which exposes them to allow external code to use and depend on them.

// src/lib.rs and src/main.rs are called crate roots, because the contents of either these two files form a module named crate at the root of crate's module structure, known as the module tree.

// Paths and referring to an item in the module tree
// A path can take 2 forms:
// An absolute path is the full path starting from a crate root; for code from an external crate, the absolute path begins with the crate name, and for code from the current crate, it starts with the literal crate.
// A relative path starts from the current module and uses: self, super, or an indentifier in the current module.

// In Rust, all items(functions, methods, structs, enums, modules and constants) are private to the parent modules by default. If you want to make an item like a function or struct private, you put it in a module.

// Items in a parent module can't use the private items inside child modules, but items in child modules can use the items in their ancestor modules. However, Rust does give you the option to expose inner parts of child modules' code to outer ancestor modules by uising the pub keyword to make an item public.

// the body of the module goes inside {}
// Inside modules, we can place other modules. Modules can also hold definitions for other items such as structs, enums, constants, traits and functions.

fn main() {}
mod front_of_house {
    // The pub keyword on a module only lets code in its ancestor modules refer to it, not access its inner code, so we need to go further and choose to make one or more of the items within the module public as well.
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }
    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}

fn deliver_order() {}

mod back_of_house {
    // if we use pub before a struct definition, we make the struct public, but the struct's fields will still be private.
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }
    impl Breakfast {
        // because back_of_house::Breakfast has a private field, the struct needs to provide a public associated function that constructs an instance of Breakfast (we’ve named it summer here). If Breakfast didn’t have such a function, we couldn’t create an instance of Breakfast in eat_at_restaurant because we couldn’t set the value of the private seasonal_fruit field in eat_at_restaurant.

        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
    // in contrast, if we make an enum public, all of its variants are public
    pub enum Appetizer {
        Soup,
        Salad,
    }
    fn fix_incorrect_order() {
        cook_order();
        // using super allows us to reference an item that we know is in the parent module
        super::deliver_order();
    }
    fn cook_order() {}
}

// bringing paths into scope with the use keyword
// adding use and a path in a scope is similar to create symbolic klink in the filesystem. Paths brought into scope with use also check privacy, like any other paths
// note that use only creates the shortcut for the particular scope in which use occurs.
// use crate::front_of_house::hosting;

// Re-exporting names with: pub use
// When we bring a name into scope with the use keyword, the name available in the new scope is private. private. To enable the code that calls our code to refer to that name as if it had been defined in that code’s scope, we can combine pub and use. This technique is called re-exporting because we’re bringing an item into scope but also making that item available for others to bring into their scope.
// Before this change, external code would have to call the add_to_waitlist function by using the path restaurant::front_of_house::hosting::add_to_waitlist(). Now that this pub use has re-exported the hosting module from the root module, external code can use the path restaurant::hosting::add_to_waitlist() instead.
pub use crate::front_of_house::hosting;

// The eat_at_restaurant function is part of our library crate’s public API, so we mark it with the pub keyword.
pub fn eat_at_restaurant() {
    // order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // change your mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    // meal.seasonal_fruit = String::from("blueberries"); - doesn't work

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    // absolute path
    // While front_of_house isn’t public, because the eat_at_restaurant function is defined in the same module as front_of_house (that is, eat_at_restaurant and front_of_house are siblings), we can refer to front_of_house from eat_at_restaurant.

    // crate::front_of_house::hosting::add_to_waitlist(); - before use
    hosting::add_to_waitlist();

    // relative path, starting with a module name means that the path is relative
    front_of_house::hosting::add_to_waitlist();
}
// Alternate File Paths
// So far we’ve covered the most idiomatic file paths the Rust compiler uses, but Rust also supports an older style of file path. For a module named front_of_house declared in the crate root, the compiler will look for the module’s code in:
// src/front_of_house.rs (what we covered)
// src/front_of_house/mod.rs (older style, still supported path)

// For a module named hosting that is a submodule of front_of_house, the compiler will look for the module’s code in:
// src/front_of_house/hosting.rs (what we covered)
// src/front_of_house/hosting/mod.rs (older style, still supported path)

// If you use both styles for the same module, you’ll get a compiler error. Using a mix of both styles for different modules in the same project is allowed, but might be confusing for people navigating your project. The main downside to the style that uses files named mod.rs is that your project can end up with many files named mod.rs, which can get confusing when you have them open in your editor at the same time.
