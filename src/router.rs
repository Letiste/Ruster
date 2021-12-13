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
        Router {
            routes: RouteNode::new("/", None),
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

    pub fn handle_request(&self, path: &str) -> Result<(), &str> {
        self.call_handler(path, Request {}, Response {})
    }

    fn call_handler(&self, path: &str, request: Request, response: Response) -> Result<(), &str> {
        match self.routes.find(path) {
            Some(node) => {
                if let Some(handler) = node.handler {
                    (handler)(request, response);
                    Ok(())
                } else {
                    Err("No route found")
                }
            }
            None => Err("No route found"),
        }
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
        println!("{:?}", router.routes);
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

    #[test]
    fn can_call_corresponding_handler() {
        let handler = |_req: Request, _res: Response| {};
        let mut router = Router::new();
        router.get("/hello/world", handler);

        assert!(router
            .call_handler("GET/hello/world", Request {}, Response {})
            .is_ok());
    }

    #[test]
    fn call_handler_returns_error_when_no_path_match() {
        let handler = |_req: Request, _res: Response| {};
        let mut router = Router::new();
        router.get("/hello/world", handler);

        assert!(router
            .call_handler("not/found", Request {}, Response {})
            .is_err());
    }

    #[test]
    fn call_handler_returns_error_when_no_handler() {
        let handler = |_req: Request, _res: Response| {};
        let mut router = Router::new();
        router.get("/hello/world", handler);
        router.get("/hello/you", handler);

        assert!(router
            .call_handler("/hello", Request {}, Response {})
            .is_err());
    }
}
