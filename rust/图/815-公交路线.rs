use std::collections::{HashMap, VecDeque};

impl Solution {
    pub fn num_buses_to_destination(routes: Vec<Vec<i32>>, source: i32, target: i32) -> i32 {
        if source == target {
            return 0;
        }
        let n = routes.len();
        let mut m = 0;
        let mut site_route_map = HashMap::new();
        for (i, route) in routes.iter().enumerate() {
            for &site in route.iter() {
                site_route_map.entry(site).or_insert(vec![]).push(i);
                m = m.max(site as usize + 1);
            }
        }
        let mut queue = VecDeque::new();
        queue.push_back(source);
        let mut visited_route = vec![false; n];
        let mut visited_site = vec![false; m];
        visited_site[source as usize] = true;
        let mut step = 1;
        while !queue.is_empty() {
            for _ in 0..queue.len() {
                let cur = queue.pop_front().unwrap();
                for &route_id in site_route_map[&cur].iter() {
                    if visited_route[route_id] {
                        continue;
                    }
                    visited_route[route_id] = true;
                    for &nxt in routes[route_id].iter() {
                        if visited_site[nxt as usize] {
                            continue;
                        }
                        if nxt == target {
                            return step;
                        }
                        visited_site[nxt as usize] = true;
                        queue.push_back(nxt);
                    }
                }
            }
            step += 1;
        }
        -1
    }
}
