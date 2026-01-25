use async_graphql::{
    EmptyMutation, EmptySubscription, Object, Result, Schema, extensions::OpenTelemetry,
};
use async_graphql_poem::GraphQL;
use opentelemetry::trace::TracerProvider as _;
use opentelemetry_sdk::trace::TracerProvider;
use poem::{EndpointExt, Route, Server, listener::TcpListener, post};

struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn hello(&self) -> Result<String> {
        Ok("World".to_string())
    }
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // build OTEL provider
    let provider = TracerProvider::builder()
        .with_simple_exporter(opentelemetry_stdout::SpanExporter::default())
        .build();
    let tracer = provider.tracer("poem-opentelemetry-basic");
    let opentelemetry_extension = OpenTelemetry::new(tracer);

    // build GraphQL schema
    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .extension(opentelemetry_extension)
        .finish();

    let app = Route::new()
        .at("/graphql", post(GraphQL::new(schema.clone())))
        .data(schema);

    println!("Poem server started at http://0.0.0.0:3000");
    println!("GraphQL endpoint at http://0.0.0.0:3000/graphql");

    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await
}
