use crate::adapter::http::axum_middle_wares;
use axum::Router;

pub struct InAssembly(Router);
pub struct Finalized(Router);

pub trait RouterExt: Sized {
    fn into_assembly(self) -> InAssembly;
}

impl RouterExt for Router {
    fn into_assembly(self) -> InAssembly {
        InAssembly(self)
    }
}

impl InAssembly {
    pub fn apply<F>(self, f: F) -> Self
    where
        F: FnOnce(Router) -> Router,
    {
        InAssembly(f(self.0))
    }

    pub fn register_middlewares(self) -> Finalized {
        let router = self
            .0
            .layer(axum::middleware::from_fn(axum_middle_wares::simple_logging));
        Finalized(router)
    }
}

impl Finalized {
    pub fn finalize(self) -> Router {
        self.0
    }
}
