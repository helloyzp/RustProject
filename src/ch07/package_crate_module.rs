mod front_of_house {

    pub mod hosting {
        pub fn add_to_waitlist() {}
    }

}

use crate::ch07::package_crate_module::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
