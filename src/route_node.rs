use crate::request::Request;
use crate::response::Response;

pub type RouteHandler = fn(Request, Response);

#[derive(Debug, Clone)]
pub struct RouteNode {
    path: String,
    pub handler: Option<RouteHandler>,
    children: Vec<RouteNode>,
}

impl RouteNode {
    pub fn new(path: &str, handler: Option<RouteHandler>) -> RouteNode {
        if path.is_empty() {
            panic!("path is empty");
        }
        RouteNode {
            path: path.to_string(),
            handler,
            children: Vec::new(),
        }
    }

    pub fn add(&mut self, path: &str, handler: RouteHandler) {
        if path.is_empty() {
            panic!("path is empty");
        }
        let path = Self::with_starting_slash(path);
        self.add_recursive(&path, handler);
    }

    fn add_recursive(&mut self, path: &str, handler: RouteHandler) {
        let mut path_iter = path.chars().peekable();
        let mut current_path_iter = self.path.chars().peekable();
        if current_path_iter.peek() == path_iter.peek() {
            let mut common_path = String::new();
            while current_path_iter.peek().is_some()
                && path_iter.peek().is_some()
                && current_path_iter.peek() == path_iter.peek()
            {
                common_path.push(path_iter.peek().unwrap().to_owned());
                path_iter.next();
                current_path_iter.next();
            }
            // if path shorter than node path
            if path_iter.peek().is_none() && current_path_iter.peek().is_some() {
                let mut new_node =
                    RouteNode::new(current_path_iter.collect::<String>().as_ref(), self.handler);
                new_node.children = self.children.as_slice().to_vec();
                self.path = common_path;
                self.handler = Some(handler);
                self.children = vec![new_node];
            }
            // if path longer than node path
            else if path_iter.peek().is_some() && current_path_iter.peek().is_none() {
                let mut next_node = None;
                for child in self.children.iter_mut() {
                    if child.path.starts_with(path_iter.peek().unwrap().to_owned()) {
                        next_node = Some(child);
                    }
                }
                if let Some(next_node) = next_node {
                    next_node.add_recursive(path_iter.collect::<String>().as_ref(), handler);
                } else {
                    self.children.push(RouteNode::new(
                        path_iter.collect::<String>().as_ref(),
                        Some(handler),
                    ))
                }
            }
            // if both path differ at some point
            else if path_iter.peek().is_some() && current_path_iter.peek().is_some() {
                let node = RouteNode::new(path_iter.collect::<String>().as_ref(), Some(handler));
                let mut new_node =
                    RouteNode::new(current_path_iter.collect::<String>().as_ref(), self.handler);
                new_node.children = self.children.as_slice().to_vec();
                self.path = common_path;
                self.handler = None;
                self.children = vec![new_node, node];
            } else {
                match &self.handler {
                    Some(_) => panic!("Route already exists!"),
                    None => self.handler = Some(handler),
                }
            }
        }
    }

    pub fn find(&self, path: &str) -> Option<&RouteNode> {
        if path.is_empty() {
            return None;
        }
        let path = Self::with_starting_slash(path);
        let mut path_iter = path.chars().peekable();
        let mut node = self;
        loop {
            let mut current_path_iter = node.path.chars().peekable();
            while path_iter.peek().is_some()
                && current_path_iter.peek().is_some()
                && path_iter.peek() == current_path_iter.peek()
            {
                path_iter.next();
                current_path_iter.next();
            }
            if current_path_iter.peek().is_none() {
                if path_iter.peek().is_none() {
                    return Some(node);
                } else {
                    for child in node.children.iter() {
                        if child.path.starts_with(path_iter.peek().unwrap().to_owned()) {
                            node = child;
                        }
                    }
                }
            } else {
                return None;
            }
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
        let route_node = RouteNode::new("/hello/world", None);

        assert_eq!(route_node.path, "/hello/world");
        assert_eq!(route_node.children.len(), 0);
    }

    #[test]
    #[should_panic]
    fn new_route_node_with_empty_path_should_panic() {
        RouteNode::new("", None);
    }

    #[test]
    fn add_route_should_create_new_node() {
        let handler = |_req: Request, _res: Response| {};
        let mut route_node = RouteNode::new("/hello/world", None);

        route_node.add("goodbye", handler);

        assert_eq!(route_node.path, "/");
        assert!(route_node.handler.is_none());
        assert_eq!(route_node.children.len(), 2);
        assert_eq!(route_node.children[0].path, "hello/world");
        assert_eq!(route_node.children[1].path, "goodbye");
    }

    #[test]
    fn add_route_should_swap_nodes_if_new_path_shorter() {
        let handler = |_req: Request, _res: Response| {};
        let mut route_node = RouteNode::new("/hello", Some(handler));

        route_node.add("/hell", handler);

        assert_eq!(route_node.path, "/hell");
        assert_eq!(route_node.children.len(), 1);
        assert_eq!(route_node.children[0].path, "o");
    }

    #[test]
    fn add_route_should_add_new_node_to_children_if_new_path_shorter() {
        let handler = |_req: Request, _res: Response| {};
        let mut route_node = RouteNode::new("/hell", Some(handler));

        route_node.add("hello", handler);

        assert_eq!(route_node.path, "/hell");
        assert_eq!(route_node.children.len(), 1);
        assert_eq!(route_node.children[0].path, "o");
    }

    #[test]
    fn add_route_should_add_new_node_to_parent_children_if_path_dont_match() {
        let handler = |_req: Request, _res: Response| {};
        let mut route_node = RouteNode::new("/hello", Some(handler));

        route_node.add("/hello/world", handler);
        route_node.add("/hello/you", handler);

        assert_eq!(route_node.path, "/hello");
        assert_eq!(route_node.children.len(), 1);
        assert_eq!(route_node.children[0].path, "/");
        let child = &route_node.children[0];
        assert_eq!(child.children.len(), 2);
        assert_eq!(child.children[0].path, "world");
        assert_eq!(child.children[1].path, "you");
    }

    #[test]
    fn add_router_should_work_through_multiple_levels() {
        let handler = |_req: Request, _res: Response| {};
        let mut route_node = RouteNode::new("/hello", Some(handler));

        route_node.add("hola", handler);

        assert_eq!(route_node.path, "/h");
        assert_eq!(route_node.children.len(), 2);
        assert_eq!(route_node.children[0].path, "ello");
        assert_eq!(route_node.children[1].path, "ola");

        route_node.add("helli", handler);
        let node = &route_node.children[0];

        assert_eq!(node.children.len(), 2);
        assert_eq!(node.path, "ell");
        assert_eq!(node.children[0].path, "o");
        assert_eq!(node.children[1].path, "i");
    }

    #[test]
    #[should_panic]
    fn add_route_with_empty_path_should_panic() {
        let handler = |_req: Request, _res: Response| {};
        let mut route_node = RouteNode::new("hello/world", Some(handler));

        route_node.add("", handler);
    }

    #[test]
    #[should_panic]
    fn add_existing_route_should_panic() {
        let handler = |_req: Request, _res: Response| {};
        let mut route_node = RouteNode::new("/hello/world", Some(handler));

        route_node.add("/hello/world", handler);
    }

    #[test]
    fn find_route_should_return_route() {
        let route_node = RouteNode::new("/hello/world", None);

        let route = route_node.find("/hello/world").unwrap();

        assert_eq!(route.path, "/hello/world");
    }

    #[test]
    fn find_route_should_return_route_in_deep_node() {
        let handler = |_req: Request, _res: Response| {};
        let mut route_node = RouteNode::new("/hello", None);
        route_node.add("/hello/world", handler);
        let route = route_node.find("/hello/world").unwrap();

        assert_eq!(route.path, "/world");
    }

    #[test]
    fn find_route_should_return_none_if_not_found() {
        let route_node = RouteNode::new("/hello", None);

        assert!(route_node.find("/goodbye").is_none());
    }
}
