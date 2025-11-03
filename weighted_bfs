use std::{collections::VecDeque, usize};
type OmomiGraph = Vec<Vec<(usize, usize)>>;

fn bfs(G: &OmomiGraph, bgnv: usize, dist: &mut Vec<usize>) {
    let mut q: VecDeque<usize> = VecDeque::new();
    q.push_back(bgnv);

    while !q.is_empty() {
        let v = match q.pop_front() {
            Some(value) => value,
            None => {
                break;
            }
        };
        for nextv in &G[v] {
            if dist[nextv.0] < dist[v] + nextv.1 {
                //より距離が大きくなる経路は探索しない
                continue;
            }

            dist[nextv.0] = dist[v] + nextv.1;
            q.push_back(nextv.0);
        }
    }
}
