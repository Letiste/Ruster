use crate::request::Request;
use crate::response::Response;

pub type RouteHandler = fn(Request, Response);

#[derive(Debug)]
pub struct RouteNode {
    path: String,
    handler: RouteHandler,
    children: Vec<RouteNode>,
}

impl RouteNode {
    pub fn new(path: &str, handler: RouteHandler) -> RouteNode {
        if path.is_empty() {
            panic!("path is empty");
        }
        let path = Self::with_starting_slash(path);
        let recursive_path: Vec<&str> = path.split_inclusive('/').collect();
        Self::new_recursive(recursive_path, handler)
    }

    fn new_recursive(recursive_path: Vec<&str>, handler: RouteHandler) -> RouteNode {
        let path = String::from(recursive_path[0]);
        let mut children: Vec<RouteNode> = Vec::new();
        if recursive_path.len() > 1 {
            children.push(Self::new_recursive(recursive_path[1..].to_vec(), handler))
        }
        RouteNode {
            path,
            children,
            handler,
        }
    }

    pub fn add(&mut self, path: &str, handler: RouteHandler) {
        let path = Self::with_starting_slash(path);
        let recursive_path: Vec<&str> = path.split_inclusive('/').collect();
        self.add_recursive(recursive_path[1..].to_vec(), handler);
    }

    fn add_recursive(&mut self, recursive_path: Vec<&str>, handler: RouteHandler) {
        let path = String::from(recursive_path[0]);
        for child in self.children.iter_mut() {
            if child.path == path {
                child.add_recursive(recursive_path[1..].to_vec(), handler);
                return;
            }
        }
        self.children
            .push(Self::new_recursive(recursive_path, handler))
    }

    pub fn find(&self, path: &str) -> Option<&RouteNode> {
        if path.is_empty() {
            return None;
        }
        let path = Self::with_starting_slash(path);
        if self.path == path {
            Some(self)
        } else {
            let recursive_path: Vec<&str> = path.split_inclusive('/').collect();
            self.find_recursive(recursive_path[1..].to_vec())
        }
    }

    fn find_recursive(&self, recursive_path: Vec<&str>) -> Option<&RouteNode> {
        let path = recursive_path[0];
        for child in self.children.iter() {
            if child.path == path {
                if recursive_path.len() == 1 {
                    return Some(child);
                } else {
                    return child.find_recursive(recursive_path[1..].to_vec());
                }
            }
        }
        None
    }

    pub fn call_handler(&self, path: &str, request: Request, response: Response) -> Result<(), &str> {
        match self.find(path) {
            Some(node) => {
                (node.handler)(request, response);
                Ok(())
            }
            None => Err("No route found"),
        }
    }

    fn with_starting_slash(path: &str) -> String {
        if path.starts_with('/') {
            String::from(path)
        } else {
            format!("/{}", path)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_route_node_should_have_path() {
        let handler = |_req: Request, _res: Response| {};
        let route_node = RouteNode::new("/", handler);

        assert_eq!(route_node.path, "/");
        assert_eq!(route_node.children.len(), 0);
    }

    #[test]
    fn new_route_node_should_have_recursive_path() {
        let handler = |_req: Request, _res: Response| {};
        let route_node = RouteNode::new("/hello/world", handler);

        assert_eq!(route_node.path, "/");
        assert_eq!(route_node.children.len(), 1);
        assert_eq!(route_node.children[0].path, "hello/");
        assert_eq!(route_node.children[0].children[0].path, "world");
    }

    #[test]
    #[should_panic]
    fn new_route_node_with_empty_path_should_panic() {
        let handler = |_req: Request, _res: Response| {};
        RouteNode::new("", handler);
    }

    #[test]
    fn add_route_should_create_new_node() {
        let handler = |_req: Request, _res: Response| {};
        let mut route_node = RouteNode::new("/hello/world", handler);

        route_node.add("goodbye", handler);

        assert_eq!(route_node.path, "/");
        assert_eq!(route_node.children.len(), 2);
        assert_eq!(route_node.children[0].path, "hello/");
        assert_eq!(route_node.children[1].path, "goodbye");
    }

    #[test]
    fn add_route_should_create_new_node_deeply() {
        let handler = |_req: Request, _res: Response| {};
        let mut route_node = RouteNode::new("/hello/world/deeply", handler);

        route_node.add("hello/world/so/deep", handler);

        let world_node = &route_node.children[0].children[0];
        assert_eq!(world_node.children.len(), 2);
        assert_eq!(world_node.children[1].path, "so/");
        assert_eq!(world_node.children[1].children[0].path, "deep");
    }

    #[test]
    #[should_panic]
    fn add_route_with_empty_path_should_panic() {
        let handler = |_req: Request, _res: Response| {};
        let mut route_node = RouteNode::new("hello/world", handler);

        route_node.add("", handler);
    }

    #[test]
    #[should_panic]
    fn add_existing_route_should_panic() {
        let handler = |_req: Request, _res: Response| {};
        let mut route_node = RouteNode::new("hello/world", handler);

        route_node.add("hello/", handler);
    }

    #[test]
    fn find_route_should_return_route() {
        let handler = |_req: Request, _res: Response| {};
        let route_node = RouteNode::new("hello/world", handler);

        let route = route_node.find("hello/world").unwrap();

        assert_eq!(route.path, "world");
    }

    #[test]
    fn can_call_corresponding_handler() {
        let handler = |_req: Request, _res: Response| {};
        let route_node = RouteNode::new("hello/world", handler);

        assert!(route_node
            .call_handler("hello/world", Request {}, Response {})
            .is_ok());
    }

    #[test]
    fn call_handler_returns_error_when_no_path_match() {
        let handler = |_req: Request, _res: Response| {};
        let route_node = RouteNode::new("hello/world", handler);

        assert!(route_node
            .call_handler("not/found", Request {}, Response {})
            .is_err());
    }
}
