extern crate hello_cargo;
extern crate rspec;

use std::io;
use std::sync::Arc;


#[test]
pub fn main() {
    let logger = Arc::new(rspec::Logger::new(io::stdout()));
    let configuration = rspec::ConfigurationBuilder::default().build().unwrap();
    let runner = rspec::Runner::new(configuration, vec![logger]);

    runner.run(
        &rspec::given("an value of ten", 10, |ctx| {
            ctx.when("adding 5 to it", |ctx| {

                ctx.then("results in fifteen", |num| {
                    assert_eq!(*num, 15);
                });
            });
        }));
}