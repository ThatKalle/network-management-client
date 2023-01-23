use crate::aux_data_structures::neighbor_info::{Neighbor, NeighborInfo};
use crate::aux_functions::conversion_factors::{
    ALT_CONVERSION_FACTOR, HANOVER_LAT_PREFIX, HANOVER_LON_PREFIX, LAT_CONVERSION_FACTOR,
    LON_CONVERSION_FACTOR,
};
use crate::aux_functions::edge_factory::edge_factory;
use crate::aux_functions::take_snapshot::total_distance;
use crate::graph::graph_ds::Graph;
use crate::mesh::device::MeshNode;
use app::protobufs;
use petgraph::graph::NodeIndex;
use std::collections::HashMap;

pub fn init_graph(
    mut snr_hashmap: HashMap<(u32, u32), (f64, u64)>,
    mut loc_hashmap: HashMap<u32, MeshNode>,
) -> Graph {
    // Traverse the array of packets once, adding nodes and edges to our lists
    let mut graph = Graph::new();
    let mut edge_left_endpoints = Vec::<NodeIndex>::new();
    let mut edge_right_endpoints = Vec::<NodeIndex>::new();
    let mut edge_distances = Vec::<f64>::new();
    let mut edge_radio_quality = Vec::<f64>::new();

    for neighbor_pair in snr_hashmap {
        let node_id = neighbor_pair.0 .0;
        let neighbor_id = neighbor_pair.0 .1;
        add_node_to_graph_if_not_exists(&mut graph, node_id);
        add_node_to_graph_if_not_exists(&mut graph, neighbor_id);
        let node_idx = graph.get_node_idx(node_id.to_string());
        let neighbor_idx = graph.get_node_idx(neighbor_id.to_string());
        let snr = neighbor_pair.1 .0;
        let node_loc = loc_hashmap.get(&node_id).unwrap();
        let neighbor_loc = loc_hashmap.get(&neighbor_id).unwrap();
        let distance = get_distance(node_loc.clone(), neighbor_loc.clone());
        edge_left_endpoints.push(node_idx);
        edge_right_endpoints.push(neighbor_idx);
        edge_distances.push(distance);
        edge_radio_quality.push(snr);
    }

    // Create the edges
    let edges = edge_factory(
        edge_left_endpoints,
        edge_right_endpoints,
        edge_distances,
        edge_radio_quality,
        None,
        None,
    );
    // Add the edges to the graph
    for edge in edges {
        graph.add_edge_from_struct(edge);
    }
    graph
}

pub fn add_node_to_graph_if_not_exists(graph: &mut Graph, node_id: u32) {
    let name: String = node_id.to_string();
    if !graph.contains_node(name.clone()) {
        graph.add_node(name.clone());
    }
}

/*
* Calculates the distance between two points on a sphere using helpers in graph snapshot
*
* Conversion function:
* Lat/Long: 1e-7 conversion from int to floating point degrees; see mesh.proto
* Altitude: in meters above sea level, no conversion needed
*/
pub fn get_distance(node_1: MeshNode, node_2: MeshNode) -> f64 {
    let node_1_data = node_1.data;
    let node_2_data = node_2.data;
    let node_1_pos = node_1_data.position.unwrap();
    let node_2_pos = node_2_data.position.unwrap();
    total_distance(
        node_1_pos.latitude_i as f64 * LAT_CONVERSION_FACTOR,
        node_1_pos.longitude_i as f64 * LON_CONVERSION_FACTOR,
        node_1_pos.altitude as f64 * ALT_CONVERSION_FACTOR,
        node_2_pos.latitude_i as f64 * LAT_CONVERSION_FACTOR,
        node_2_pos.longitude_i as f64 * LON_CONVERSION_FACTOR,
        node_2_pos.altitude as f64 * ALT_CONVERSION_FACTOR,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn generate_zeroed_position() -> protobufs::Position {
        let position = protobufs::Position {
            latitude_i: 0,
            longitude_i: 0,
            altitude: 0,
            time: 0,
            location_source: 0,
            altitude_source: 0,
            timestamp: 0,
            timestamp_millis_adjust: 0,
            altitude_hae: 0,
            altitude_geoidal_separation: 0,
            pdop: 0,
            hdop: 0,
            vdop: 0,
            gps_accuracy: 0,
            ground_speed: 0,
            ground_track: 0,
            fix_quality: 0,
            fix_type: 0,
            sats_in_view: 0,
            sensor_id: 0,
            next_update: 0,
            seq_number: 0,
        };
        position
    }

    fn generate_test_user() -> protobufs::User {
        let user = protobufs::User {
            id: "test".to_string(),
            long_name: "test".to_string(),
            short_name: "test".to_string(),
            macaddr: Vec::new(),
            hw_model: 0,
            is_licensed: false,
        };
        user
    }

    fn generate_zeroed_device_metrics() -> protobufs::DeviceMetrics {
        let devicemetrics = protobufs::DeviceMetrics {
            battery_level: 0,
            voltage: 0.0,
            channel_utilization: 0.0,
            air_util_tx: 0.0,
        };
        devicemetrics
    }

    #[test]
    fn test_init_graph() {
        let meshnode_1: MeshNode = MeshNode {
            device_metrics: vec![],
            environment_metrics: vec![],
            data: protobufs::NodeInfo {
                num: 1,
                user: Some(generate_test_user()),
                position: Some(generate_zeroed_position()),
                snr: 0.0,
                last_heard: 0,
                device_metrics: Some(generate_zeroed_device_metrics()),
            },
        };
        let meshnode_2: MeshNode = MeshNode {
            device_metrics: vec![],
            environment_metrics: vec![],
            data: protobufs::NodeInfo {
                num: 2,
                user: Some(generate_test_user()),
                position: Some(generate_zeroed_position()),
                snr: 0.0,
                last_heard: 0,
                device_metrics: Some(generate_zeroed_device_metrics()),
            },
        };
        let meshnode_3 = MeshNode {
            device_metrics: vec![],
            environment_metrics: vec![],
            data: protobufs::NodeInfo {
                num: 3,
                user: Some(generate_test_user()),
                position: Some(generate_zeroed_position()),
                snr: 0.0,
                last_heard: 0,
                device_metrics: Some(generate_zeroed_device_metrics()),
            },
        };
        let meshnode_4 = MeshNode {
            device_metrics: vec![],
            environment_metrics: vec![],
            data: protobufs::NodeInfo {
                num: 4,
                user: Some(generate_test_user()),
                position: Some(generate_zeroed_position()),
                snr: 0.0,
                last_heard: 0,
                device_metrics: Some(generate_zeroed_device_metrics()),
            },
        };
        let mut loc_hashmap: HashMap<u32, MeshNode> = HashMap::new();
        let mut snr_hashmap: HashMap<(u32, u32), (f64, u64)> = HashMap::new();
        loc_hashmap.insert(1, meshnode_1);
        loc_hashmap.insert(2, meshnode_2);
        loc_hashmap.insert(3, meshnode_3);
        loc_hashmap.insert(4, meshnode_4);
        snr_hashmap.insert((1, 2), (0.9, 0));
        snr_hashmap.insert((1, 3), (0.9, 0));
        snr_hashmap.insert((1, 4), (0.9, 0));
        snr_hashmap.insert((2, 3), (0.9, 0));
        snr_hashmap.insert((2, 4), (0.9, 0));
        snr_hashmap.insert((3, 4), (0.9, 0));
        let graph = init_graph(snr_hashmap, loc_hashmap);
        // Check that the graph has the correct number of nodes
        assert_eq!(graph.get_order(), 4);
        // Check that the graph has the correct number of edges
        assert_eq!(graph.get_size(), 6);
    }

    #[test]
    fn test_prioritize_earlier_snr() {
        let neighbor_1 = Neighbor {
            id: 1,
            timestamp: 0,
            snr: 0.9,
        };
        let neighbor_2 = Neighbor {
            id: 2,
            timestamp: 100,
            snr: 0.1,
        };
        let neighbor_info_1 = NeighborInfo {
            id: 1,
            timestamp: 0,
            neighbors: vec![neighbor_2.clone()],
        };
        let neighbor_info_2: NeighborInfo = NeighborInfo {
            id: 2,
            timestamp: 0,
            neighbors: vec![neighbor_1.clone()],
        };
        let distance_1_info = protobufs::Position {
            latitude_i: (43.7022 / LAT_CONVERSION_FACTOR) as i32,
            longitude_i: (72.2882 / LON_CONVERSION_FACTOR) as i32,
            altitude: 0,
            time: 0,
            location_source: 0,
            altitude_source: 0,
            timestamp: 0,
            timestamp_millis_adjust: 0,
            altitude_hae: 0,
            altitude_geoidal_separation: 0,
            pdop: 0,
            hdop: 0,
            vdop: 0,
            gps_accuracy: 0,
            ground_speed: 0,
            ground_track: 0,
            fix_quality: 0,
            fix_type: 0,
            sats_in_view: 0,
            sensor_id: 0,
            next_update: 0,
            seq_number: 0,
        };
        let meshnode_1: MeshNode = MeshNode {
            device_metrics: vec![],
            environment_metrics: vec![],
            data: protobufs::NodeInfo {
                num: 1,
                user: Some(generate_test_user()),
                position: Some(distance_1_info),
                snr: 0.0,
                last_heard: 0,
                device_metrics: Some(generate_zeroed_device_metrics()),
            },
        };
        let distance_2_info = protobufs::Position {
            latitude_i: (43.7030 / LAT_CONVERSION_FACTOR) as i32,
            longitude_i: (72.2890 / LON_CONVERSION_FACTOR) as i32,
            altitude: 0,
            time: 0,
            location_source: 0,
            altitude_source: 0,
            timestamp: 0,
            timestamp_millis_adjust: 0,
            altitude_hae: 0,
            altitude_geoidal_separation: 0,
            pdop: 0,
            hdop: 0,
            vdop: 0,
            gps_accuracy: 0,
            ground_speed: 0,
            ground_track: 0,
            fix_quality: 0,
            fix_type: 0,
            sats_in_view: 0,
            sensor_id: 0,
            next_update: 0,
            seq_number: 0,
        };
        let meshnode_2: MeshNode = MeshNode {
            device_metrics: vec![],
            environment_metrics: vec![],
            data: protobufs::NodeInfo {
                num: 2,
                user: Some(generate_test_user()),
                position: Some(distance_2_info),
                snr: 0.0,
                last_heard: 0,
                device_metrics: Some(generate_zeroed_device_metrics()),
            },
        };
        let mut loc_hashmap: HashMap<u32, MeshNode> = HashMap::new();
        let mut snr_hashmap: HashMap<(u32, u32), (f64, u64)> = HashMap::new();
        loc_hashmap.insert(1, meshnode_1);
        loc_hashmap.insert(2, meshnode_2);
        snr_hashmap.insert((1, 2), (0.1, 100));
        snr_hashmap.insert((2, 1), (0.9, 0));
        let mut graph = init_graph(snr_hashmap, loc_hashmap);
        // Check that the graph has the correct number of edges
        assert_eq!(graph.get_size(), 1);
        // Check the edge weights to check that they are both the weight of the 1-2 edge, which has neighbor 2's SNR
        // Assert that the 1-2 edge is the correct (smaller) SNR
        let first_edge_weight = graph.get_edge_weight(
            neighbor_1.id.to_string(),
            neighbor_2.id.to_string(),
            None,
            Some(false),
        );
        // The correct weight should a sum of the two distances normalized w 0.1 radio quality, which is this float
        assert_eq!(first_edge_weight, 1.0);
    }
}
