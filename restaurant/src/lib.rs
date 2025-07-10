mod front_of_house { // module ke andar module pattern
    mod hosting { // module 1
        fn add_to_waitlist() {} // function 

        fn seat_at_table() {}
    }

    mod serving {  // module 2
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}