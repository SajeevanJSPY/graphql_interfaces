#[derive(Debug, Clone, Copy)]
pub struct GraphqlModel {}

impl GraphqlModel {
    pub fn new() -> Self {
        Self {}
    }
}

/// graphql client can send multiple requests to graphql server via graphql model
#[derive(Debug, Clone, Copy)]
pub struct GraphqlRequest {
    model: GraphqlModel,
}

impl GraphqlRequest {
    pub fn new() -> Self {
        Self {
            model: GraphqlModel::new(),
        }
    }
}

pub fn send_request_to_server() {
    let mut request = GraphqlRequest::new();

    let raw_client_request = r#"
        {
            user(id: 4) {
                name
            }
        }
    "#;

    println!("{}", raw_client_request);
}
