use actix::{Actor, Context, Message, System};
use actix_handler_macro::actix_handler;

struct Example;

type ExampleContext = Context<Example>;

impl Actor for Example {
    type Context = Context<Example>;
}

#[derive(Clone, Message)]
#[rtype(result = "String")]
struct Greeting {
    name: String,
}

#[actix_handler]
impl Example {
    fn greet(&self, message: Greeting, _ctx: &ExampleContext) -> String {
        format!("Hello {}", message.name).to_string()
    }
}

#[test]
fn example_usage() {
    let mut sys = System::new("actix-test-runtime");
    let addr = Example {}.start();

    sys.block_on(async move {
        let greeting = Greeting {
            name: "you".to_string(),
        };
        // `Example` has its handler impl block
        let result = addr.send(greeting.clone()).await.ok().unwrap();
        assert_eq!(result, "Hello you");

        // An Addr trait is also expanded:
        let result = addr.greet(greeting.clone()).await.ok().unwrap();
        assert_eq!(result, "Hello you")
    });
}
