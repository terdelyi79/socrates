use crate::{
    error::Error,
    event::Event,
    sink::{Command, Sink},
    storage::Storage,
};
use http_body_util::{BodyExt, Full};
use hyper::{body::Bytes, server::conn::http1, service::service_fn, Method, Response, StatusCode};
use hyper_util::rt::TokioIo;
use std::{collections::HashMap, future::Future, net::SocketAddr, sync::Arc};
use tokio::{net::TcpListener, sync::RwLock};

type Query<A> = Box<fn(aggregate: &A) -> Result<String, Error>>;

/// A REST based API service receiving commands and queries.
/// I uses ths underlying sink to with registered commands nd queries together
pub struct Api<S, A>
where
    S: Storage + Send + Sync,
    A: Send + Sync,
{
    sink: Arc<RwLock<Sink<S, A>>>,
    queries: Arc<RwLock<HashMap<String, Query<A>>>>,
}

impl<S, A> Api<S, A>
where
    S: Storage + Send + Sync + 'static,
    A: Send + Sync + 'static,
{
    /// Create a new API
    pub fn new(sink: Sink<S, A>) -> Self {
        Self {
            sink: Arc::new(RwLock::new(sink)),
            queries: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Register a function as a command
    pub async fn add_command(&mut self, type_id: &str, command: Command<A>) {
        let mut sink = self.sink.write().await;
        sink.add_handler(type_id.into(), command);
    }

    /// Register a function as a query
    pub async fn add_query(&mut self, type_id: &str, query: Query<A>) {
        let mut queries = self.queries.write().await;
        queries.insert(type_id.into(), query);
    }

    /// Start to listen on a newtork address to received commands and queries
    pub async fn listen(self, address: SocketAddr) -> Result<(), Error> {
        {
            let mut sink = self.sink.write().await;
            sink.init().await?;
        }

        let this = Arc::new(self);

        // Create a listener listening on port 7777 of localhost
        let listener = TcpListener::bind(address).await?;

        // Process all incoming HTTP requests in a loop
        loop {
            let (stream, _) = listener.accept().await?;
            let io = TokioIo::new(stream);
            let this = this.clone();
            let sink = this.sink.clone();

            // Start a new task for the current request runnin the handle_request function on it
            tokio::task::spawn(async move {
                http1::Builder::new()
                    .serve_connection(
                        io,
                        service_fn(|request| this.handle_request(sink.clone(), request)),
                    )
                    .await
            });
        }
    }

    /// Handle a command or query request received from te network
    fn handle_request(
        &self,
        sink: Arc<RwLock<Sink<S, A>>>,
        request: hyper::Request<hyper::body::Incoming>,
    ) -> impl Future<Output = Result<Response<Full<Bytes>>, Error>> {
        let sink = sink.clone();

        let queries = self.queries.clone();

        async move {
            
            // Extract all needed data from te request
            let url = request.uri().to_string();
            let method = request.method().clone();
            let bytes = request.collect().await?.to_bytes();
            let body = String::from_utf8(bytes.to_vec())?;

            match method {
                // GET method is used for queries
                Method::GET => {
                    let queries = queries.read().await;
                    let query = queries.get(&url[1..]);

                    match query {
                        Some(query) => {
                            
                            // Get agregate for the sink
                            let sink = sink.read().await;
                            let aggregate = sink.aggregate();
                            let aggregate = aggregate.read().await;
                            
                            // Call the query
                            let result = query(&aggregate);
                            
                            // Return result
                            Ok(Response::new(Full::new(Bytes::from(result?))))
                        },
                        None => {
                            
                            // If query does not exist, then return an "Unknown Query" error message with status code 404
                            let mut response = Response::new(Full::new(Bytes::from("Unknown query")));
                            *response.status_mut() = StatusCode::NOT_FOUND;
                            
                            Ok(response)
                        }
                    }
                }

                // POST method is used for commands
                Method::POST => {

                    // Add the even to the sink
                    let event = Arc::new(Event::Text(url[1..].into(), body));
                    let mut sink = sink.write().await;
                    sink.add(event).await?;

                    // Return empty budy using OK as status code
                    Ok(Response::new(Full::new(Bytes::from(""))))
                }
                _ => {
                            
                    // Only GET and POST methods are supported
                    let mut response = Response::new(Full::new(Bytes::from("Method is not supported")));
                    *response.status_mut() = StatusCode::BAD_REQUEST;
                    
                    Ok(response)
                }
            }
        }
    }
    
}
