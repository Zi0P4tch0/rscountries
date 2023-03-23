
use actix_web::Error;
use actix_web::dev::{Service, Transform, ServiceRequest, ServiceResponse, forward_ready};
use futures::future::{ready, Ready, LocalBoxFuture};
use slog::{info, warn};

use crate::AppState;

pub struct SlogLogger;

impl<S, B> Transform<S, ServiceRequest> for SlogLogger
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = SlogLoggerMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(SlogLoggerMiddleware { service }))
    }
}

pub struct SlogLoggerMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for SlogLoggerMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        
        let logger = req.app_data::<actix_web::web::Data<AppState>>().unwrap().logger.clone();
        
        let req_method = req.method().to_string();
        let req_path = req.path().to_string();
        let start = std::time::Instant::now();

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;
            let elapsed = start.elapsed();
            let status = res.status();
            if status.is_client_error() || status.is_server_error() {
                warn!(logger, "{} {} {} {:?}", req_method, req_path, status, elapsed);
            } else {
                info!(logger, "{} {} {} {:?}", req_method, req_path, status, elapsed);
            }
            Ok(res)
        })
    }
}