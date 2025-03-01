mod front_of_house {
  // pub keyword makes the module public
    pub mod hosting {
      // pub keyword makes the function public
        pub fn add_to_waitlist() {
            println!("add_to_waitlist");
        }

        fn seat_at_table() {
            println!("seat_at_table");
        }
    }
    mod serving {
        fn take_order() {
            println!("take_order");
        }

        fn serve_order() {
            println!("serve_order");
        }

        fn take_payment() {
            println!("take_payment");
        }
    }
}

pub fn eat_at_restaurant() {
    // Absolute path - prefer this over relative paths,
    // assume you are more likely going to move code around
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}
