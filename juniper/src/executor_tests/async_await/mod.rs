use crate::{RootNode};

struct User {
    name: String,
}

#[crate::object_internal]
impl User {
    fn name(&self) -> &str {
        &self.name
    }

    async fn friends(&self) -> Vec<User> {
        vec![]
    }
}

struct Query; 

#[crate::object_internal]
impl Query {
    fn field_sync(&self) -> &'static str {
        "field_sync"
    }

    async fn field_async_plain() -> String {
        "field_async_plain".to_string()
    }

    fn user(id: String) -> User {
        User{ name: id }
    }

    async fn delayed() -> bool {
        let when = tokio::clock::now() + std::time::Duration::from_millis(100);
        tokio::timer::Delay::new(when).await;
        true
    }
}

struct Mutation;


#[crate::object_internal]
impl Mutation {
}

fn run<O>(f: impl std::future::Future<Output = O>) -> O {
    tokio::runtime::current_thread::Runtime::new().unwrap().block_on(f)
}

#[test]
fn async_simple() {
    let schema = RootNode::new(Query, Mutation);
    let doc = r"query { fieldSync, fieldAsyncPlain, delayed }";

    let vars = Default::default();
    let f = crate::execute_async(doc, None, &schema, &vars, &());

    let (res, errs) = run(f).unwrap();

    assert!(errs.is_empty());

    assert_eq!(
        res,
        crate::graphql_value!({
            "fieldSync": "field_sync",
            "fieldAsyncPlain": "field_async_plain",
            "delayed": true,
        }),
    );
}
