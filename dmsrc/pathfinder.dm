/// Register a list of nodes into a rust library. This list of nodes must have been serialized in a json.
/// {
///		"position": { x: 0, y: 0, z: 0 },
///		"mask": 0,
///		"links": [{x: 0, y: 0, z: 1}]
/// }
/// A node cannot link twice to the same node and shouldn't link itself either.
#define rustg_update_nodes_astar(json) RUSTG_CALL(RUST_G, "update_nodes_astar")(json)

/// Remove the node with node position.
#define rustg_remove_node_astar(node_pos) RUSTG_CALL(RUST_G, "remove_node_astar")(node_pos)

/// Compute the shortest path between start_node and goal_node using A*.
#define rustg_generate_path_astar(strat_node_pos, goal_node_pos, pass_bit, deny_bit, costs) RUSTG_CALL(RUST_G, "generate_path_astar")(strat_node_pos, goal_node_pos, istext(pass_bit) ? pass_bit : num2text(pass_bit), istext(deny_bit) ? deny_bit : num2text(deny_bit), costs)
