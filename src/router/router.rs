use router::Router;

pub struct RouterInstance {
    router: Router,
}

pub fn router() -> Router {
    let mut router = Router::new();
    return router;
}
