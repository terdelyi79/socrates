use crate::{
    error::Error,
    event::Event,
    sink::{Command, Sink},
    storage::Storage,
};
use http_body_util::{BodyExt, Full};
use hyper::{body::Bytes, server::conn::http1, service::service_fn, Method, Request, Response};
use hyper_util::rt::TokioIo;
use std::{collections::HashMap, future::Future, net::SocketAddr, sync::Arc};
use tokio::{net::TcpListener, sync::RwLock};

type Query<A> = Box<fn(aggregate: &A) -> Result<String, Error>>;

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
    pub fn new(sink: Sink<S, A>) -> Self {
        Self {
            sink: Arc::new(RwLock::new(sink)),
            queries: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn add_command(&mut self, type_id: &str, command: Command<A>) {        
        let mut sink = self.sink.write().await;
        sink.add_handler(type_id.into(), command);
    }

    pub async fn add_query(&mut self, type_id: &str, query: Query<A>) {
        let mut queries = self.queries.write().await;
        queries.insert(type_id.into(), query);
    }

    pub async fn run(self, address: SocketAddr) -> Result<(), Error> {

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

    fn handle_request(
        &self,
        sink: Arc<RwLock<Sink<S, A>>>,
        request: hyper::Request<hyper::body::Incoming>,
    ) -> impl Future<Output = Result<Response<Full<Bytes>>, Error>> {
        let sink = sink.clone();

        let queries = self.queries.clone();

        async move {
            let url = request.uri().to_string();
            let method = request.method().clone();
            let body = Self::body_to_string(request).await;

            match method {
                Method::GET => {
                    let queries = queries.read().await;
                    let query = queries.get(&url[1..]);

                    match query {
                        Some(query) => {
                            let sink = sink.read().await;
                            let aggregate = sink.aggregate();
                            let aggregate = aggregate.read().await;
                            let result = query(&aggregate);
                            Ok(Response::new(Full::new(Bytes::from(result?))))
                        }
                        None => Err(Error {
                            message: "Unknown query".into(),
                        }),
                    }
                }
                Method::POST => {
                    let event = Arc::new(Event::Text(url[1..].into(), body));
                    let mut sink = sink.write().await;
                    sink.add(event).await?;
                    Ok(Response::new(Full::new(Bytes::from(""))))
                }
                _ => Err(Error {
                    message: "Unsupported method".into(),
                }),
            }
        }
    }

    async fn body_to_string(req: Request<hyper::body::Incoming>) -> String {
        let bytes = req.collect().await.unwrap().to_bytes();
        String::from_utf8(bytes.to_vec()).unwrap()
    }
   
}
