use num_integer::sqrt;
use pathfinding::prelude::astar;
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, hash::Hash};
use thiserror::Error;

#[derive(
    Serialize, Deserialize, Default, Debug, Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd,
)]
struct NodePosition {
    x: isize,
    y: isize,
    z: isize,
}

impl NodePosition {
    #[allow(dead_code)]
    pub fn new(x: isize, y: isize, z: isize) -> Self {
        Self { x, y, z }
    }

    pub fn with_x(mut self, x: isize) -> Self {
        self.x += x;

        self
    }

    pub fn with_y(mut self, y: isize) -> Self {
        self.y += y;

        self
    }

    #[allow(dead_code)]
    pub fn with_z(mut self, z: isize) -> Self {
        self.z += z;

        self
    }
}

type NodeId = NodePosition;

static mut NODES: BTreeMap<NodeId, Node> = BTreeMap::new();

unsafe fn nodes_mut() -> &'static mut BTreeMap<NodeId, Node> {
    unsafe { &mut *std::ptr::addr_of_mut!(NODES) }
}

fn get_node(id: NodeId) -> Option<Node> {
    unsafe { nodes_mut().get(&id).cloned() }
}

fn update_node(id: NodeId, node: Node) {
    unsafe { nodes_mut().entry(id).or_insert(node) };
}

fn remove_node(id: &NodeId) {
    unsafe { nodes_mut().remove(id) };
}

#[derive(Serialize, Deserialize, Default, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Node {
    position: NodePosition,
    mask: usize,
    links: Vec<NodePosition>,
}

impl Node {
    fn successors(&self, pass_bit: usize, deny_bit: usize) -> Vec<(Node, usize)> {
        let pos = &self.position;
        let mut neighbours = Vec::with_capacity(4 + self.links.len());

        if let Some(up) = get_node(pos.with_y(-1)) {
            neighbours.push(up);
        };

        if let Some(right) = get_node(pos.with_x(1)) {
            neighbours.push(right);
        };

        if let Some(down) = get_node(pos.with_y(1)) {
            neighbours.push(down);
        };

        if let Some(left) = get_node(pos.with_x(-1)) {
            neighbours.push(left);
        }

        for link in &self.links {
            if let Some(node) = get_node(*link) {
                neighbours.push(node);
            }
        }

        neighbours
            .into_iter()
            .filter_map(|node| {
                if (node.mask & pass_bit) == 0 {
                    return None;
                }

                if (node.mask & deny_bit) != 0 {
                    return None;
                }

                let dst = self.distance(&node);

                Some((node.clone(), dst))
            })
            .collect()
    }

    fn distance(&self, other: &Self) -> usize {
        sqrt(
            ((self.position.x as isize - other.position.x as isize).pow(2)
                + (self.position.y as isize - other.position.y as isize).pow(2))
                as usize,
        )
    }
}

#[derive(Error, Debug)]
enum RegisteringNodesError {
    #[error(transparent)]
    Serialization(#[from] serde_json::Error),
}

byond_fn!(fn update_nodes_astar(json) {
    match update_nodes(json) { Ok(s) => Some(s),
        Err(e) => Some(format!("{e}"))
    }
});

// Builds a list of nodes from a json file.
fn update_nodes(json: &str) -> Result<String, RegisteringNodesError> {
    let deserialized_nodes: Vec<Node> = serde_json::from_str(json)?;

    for node in deserialized_nodes {
        update_node(node.position.clone(), node);
    }

    Ok("1".to_string())
}

byond_fn!(fn remove_node_astar(node_id) {
    let node_id: NodeId = match serde_json::from_str(node_id) {
        Err(err) => return Some(format!("{err}")),
        Ok(v) => v
    };

    remove_node(&node_id);

    Some("1".to_string())
});

#[derive(Error, Debug, PartialEq, Eq)]
enum AstarError {
    #[error("Starting node not found")]
    StartNodeNotFound,
    #[error("Goal node not found")]
    GoalNodeNotFound,
    #[error("No path found")]
    NoPath,
}

byond_fn!(fn generate_path_astar(start_node_pos, goal_node_pos, pass_bit, deny_bit) {
    let start_node_pos = match serde_json::from_str::<NodePosition>(start_node_pos) {
        Err(err) => return Some(format!("{err}")),
        Ok(v) => v
    };

    let goal_node_pos = match serde_json::from_str::<NodePosition>(goal_node_pos) {
        Err(err) => return Some(format!("{err}")),
        Ok(v) => v
    };

    let pass_bit: usize = match pass_bit.parse() {
        Err(err) => return Some(format!("{err}")),
        Ok(v) => v
    };

    let deny_bit: usize = match deny_bit.parse() {
        Err(err) => return Some(format!("{err}")),
        Ok(v) => v
    };

    match generate_path(start_node_pos, goal_node_pos, pass_bit, deny_bit) {
        Ok(vector) => Some(match serde_json::to_string(&vector) {
            Ok(s) => s,
            Err(_) => "Cannot serialize path".to_string(),
        }),
        Err(e) => Some(format!("{e}"))
    }
});

// Compute the shortest path between start node and goal node using A*
fn generate_path(
    start_node_pos: NodePosition,
    goal_node_pos: NodePosition,
    pass_bit: usize,
    deny_bit: usize,
) -> Result<Vec<NodePosition>, AstarError> {
    let start_node = match get_node(start_node_pos) {
        Some(node) => node,
        _ => return Err(AstarError::StartNodeNotFound),
    };

    let goal_node = match get_node(goal_node_pos) {
        Some(node) => node,
        _ => return Err(AstarError::GoalNodeNotFound),
    };

    if goal_node.position.z != start_node.position.z {
        return Err(AstarError::NoPath);
    }

    // Compute the shortest path between start node and goal node using A*
    let path = astar(
        &start_node,
        |node| node.successors(pass_bit, deny_bit),
        |node| node.distance(&goal_node),
        |node| node.position == goal_node.position,
    );

    // Extract a vector of node container from the path variable. Errors if no path was found
    let path = match path {
        None => return Err(AstarError::NoPath),
        Some(path) => path.0,
    };

    // Map every nodecontainer to the unique id of its node, so it can be sent to byond
    Ok(path
        .into_iter()
        .map(|node| node.position)
        .rev() // Reverse iterator so it is easy to pop the list in byond
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    const NODE_TURF_BIT: usize = 1 << 0;
    const NODE_SPACE_BIT: usize = 1 << 1;

    #[test]
    fn test_pathfinding() {
        let nodes = vec![
            Node {
                position: NodePosition::new(0, 0, 0),
                mask: NODE_TURF_BIT,
                ..Default::default()
            },
            Node {
                position: NodePosition::new(1, 0, 0),
                mask: NODE_SPACE_BIT,
                ..Default::default()
            },
            Node {
                position: NodePosition::new(2, 0, 0),
                mask: NODE_SPACE_BIT,
                ..Default::default()
            },
            Node {
                position: NodePosition::new(3, 0, 0),
                mask: NODE_SPACE_BIT,
                ..Default::default()
            },
            Node {
                position: NodePosition::new(4, 0, 0),
                mask: NODE_TURF_BIT,
                ..Default::default()
            },
        ];

        for node in nodes {
            update_node(node.position, node);
        }

        let path = generate_path(
            NodePosition::new(0, 0, 0),
            NodePosition::new(4, 0, 0),
            NODE_TURF_BIT,
            NODE_SPACE_BIT,
        );

        assert_eq!(path, Err(AstarError::NoPath));

        let path = generate_path(
            NodePosition::new(0, 0, 0),
            NodePosition::new(4, 0, 0),
            NODE_TURF_BIT | NODE_SPACE_BIT,
            0,
        );

        assert_eq!(
            path,
            Ok(vec![
                NodePosition::new(4, 0, 0),
                NodePosition::new(3, 0, 0),
                NodePosition::new(2, 0, 0),
                NodePosition::new(1, 0, 0),
                NodePosition::new(0, 0, 0),
            ])
        );

        let nodes = vec![
            Node {
                position: NodePosition::new(0, 1, 0),
                mask: NODE_TURF_BIT,
                ..Default::default()
            },
            Node {
                position: NodePosition::new(1, 1, 0),
                mask: NODE_TURF_BIT,
                ..Default::default()
            },
            Node {
                position: NodePosition::new(2, 1, 0),
                mask: NODE_TURF_BIT,
                ..Default::default()
            },
            Node {
                position: NodePosition::new(3, 1, 0),
                mask: NODE_TURF_BIT,
                ..Default::default()
            },
            Node {
                position: NodePosition::new(4, 1, 0),
                mask: NODE_TURF_BIT,
                ..Default::default()
            },
        ];

        for node in nodes {
            update_node(node.position, node);
        }

        let path = generate_path(
            NodePosition::new(0, 0, 0),
            NodePosition::new(4, 0, 0),
            NODE_TURF_BIT,
            NODE_SPACE_BIT,
        );

        assert_eq!(
            path,
            Ok(vec![
                NodePosition::new(4, 0, 0),
                NodePosition::new(4, 1, 0),
                NodePosition::new(3, 1, 0),
                NodePosition::new(2, 1, 0),
                NodePosition::new(1, 1, 0),
                NodePosition::new(0, 1, 0),
                NodePosition::new(0, 0, 0),
            ])
        );
    }
}
