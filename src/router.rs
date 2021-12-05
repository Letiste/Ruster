use crate::{
    request::Request,
    response::Response,
    route_node::{RouteHandler, RouteNode},
};

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum HttpMethod {
    Get,
    Post,
    Patch,
    Put,
    Delete,
}

pub struct Router {
    routes: RouteNode,
}

impl Router {
    pub fn new() -> Router {
        let handler = |_req: Request, _res: Response| panic!("root of router has been called");
        Router {
            routes: RouteNode::new("/", handler),
        }
    }

    fn method_path_to_string(method: HttpMethod, path: &str) -> String {
        let method = match method {
            HttpMethod::Get => "GET",
            HttpMethod::Post => "POST",
            HttpMethod::Patch => "PATCH",
            HttpMethod::Put => "PUT",
            HttpMethod::Delete => "DELETE",
        };
        if path.starts_with('/') {
            format!("{}{}", method, path)
        } else {
            format!("{}/{}", method, path)
        }
    }

    pub fn route(&mut self, method: HttpMethod, path: &str, handler: RouteHandler) {
        let path = Self::method_path_to_string(method, path);
        self.routes.add(&path, handler)
    }

    pub fn get(&mut self, path: &str, handler: RouteHandler) {
        self.route(HttpMethod::Get, path, handler)
    }

    pub fn post(&mut self, path: &str, handler: RouteHandler) {
        self.route(HttpMethod::Post, path, handler)
    }

    pub fn patch(&mut self, path: &str, handler: RouteHandler) {
        self.route(HttpMethod::Patch, path, handler)
    }

    pub fn put(&mut self, path: &str, handler: RouteHandler) {
        self.route(HttpMethod::Put, path, handler)
    }

    pub fn delete(&mut self, path: &str, handler: RouteHandler) {
        self.route(HttpMethod::Delete, path, handler)
    }

    pub fn handle_request(&mut self, path: &str) -> Result<(), &str> {
        self.routes.call_handler(path, Request {}, Response {})
    }
}

#[cfg(test)]
mod tests {
    use crate::{request::Request, response::Response};

    use super::*;

    #[test]
    fn route_should_add_route_to_corresponding_method() {
        let handler = |_req: Request, _res: Response| {};
        let mut router = Router::new();

        router.route(HttpMethod::Get, "/hello/world", handler);

        assert!(router.routes.find("GET/hello/world").is_some());
    }

    #[test]
    fn get_should_add_route_to_get_method() {
        let handler = |_req: Request, _res: Response| {};
        let mut router = Router::new();

        router.get("/hello/world", handler);

        assert!(router.routes.find("GET/hello/world").is_some());
    }

    #[test]
    fn post_should_add_route_to_post_method() {
        let handler = |_req: Request, _res: Response| {};
        let mut router = Router::new();

        router.post("/hello/world", handler);

        assert!(router.routes.find("POST/hello/world").is_some());
    }

    #[test]
    fn patch_should_add_route_to_patch_method() {
        let handler = |_req: Request, _res: Response| {};
        let mut router = Router::new();

        router.patch("/hello/world", handler);

        assert!(router.routes.find("PATCH/hello/world").is_some());
    }

    #[test]
    fn put_should_add_route_to_put_method() {
        let handler = |_req: Request, _res: Response| {};
        let mut router = Router::new();

        router.put("/hello/world", handler);

        assert!(router.routes.find("PUT/hello/world").is_some());
    }

    #[test]
    fn delete_should_add_route_to_delete_method() {
        let handler = |_req: Request, _res: Response| {};
        let mut router = Router::new();

        router.delete("/hello/world", handler);

        assert!(router.routes.find("DELETE/hello/world").is_some());
    }
}
