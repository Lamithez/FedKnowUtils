pub mod tree {}

pub mod graph {
    // https://leetcode.cn/problems/network-delay-time/
    pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
        let mut matrix = vec![vec![i32::MAX; n as usize]; n as usize];
        for item in times {
            matrix[(item[0] - 1) as usize][(item[1] - 1) as usize] = item[2];
        }
        let distances = dijkstra(matrix, (k - 1) as usize);
        if let Some(&x) = distances.iter().max() {
            if x != i32::MAX { return x; }
        }
        return -1;
    }

    //参数为图的邻接矩阵表示，返回dist数组和path数组，分别表示距离原点的距离和上一个经过节点
    pub fn dijkstra(matrix: Vec<Vec<i32>>, k: usize) -> Vec<i32> {
        let len = matrix.len();
        let mut done: Vec<bool> = vec![false; len];
        let mut dist: Vec<i32> = matrix[k].clone();
        dist[k] = 0;
        done[k] = true;
        loop {
            //找到当前能添加的最小值
            // let mut min = len;
            // for (i, &ok) in done.iter().enumerate() {
            //     if !ok && (len == min || dist[i] < dist[min]) {
            //         min = i;
            //     }
            // }
            // if min == len || dist[min] == i32::MAX {
            //     return dist;
            // }

            let min = match dist.iter().enumerate().filter(|(index, _)| { !done[*index] }).min_by_key(|(index, &item)| item) {
                None => { return dist; }
                Some((_, &i32::MAX)) => { return dist }
                Some((index, _item)) => { index }
            };

            //将其加入到已经确定的队列
            done[min] = true;
            // 更新现有的节点
            for (index, &distance) in matrix[min].iter().enumerate() {
                if distance != i32::MAX {
                    dist[index] = dist[index].min(dist[min] + distance);
                }
            }
        }
    }

    pub mod topological_sorting {
        pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
            let mut adjacency_list = vec![vec![]; num_courses as usize];
            for edge in prerequisites {
                adjacency_list[edge[0] as usize].push(edge[1] as usize);
            }
            match zero_sort(adjacency_list) {
                Ok(_) => { true }
                Err(_) => { false }
            }
        }

        pub fn zero_sort(adjacency_list: Vec<Vec<usize>>) -> Result<Vec<usize>, ()> {
            let num = adjacency_list.len();
            let mut indegree = vec![0; num];
            let mut zeros = Vec::new();
            let mut sequence = Vec::with_capacity(num);
            for edges in &adjacency_list {
                for terminal in edges {
                    indegree[*terminal] += 1;
                }
            }
            for (i, &d) in indegree.iter().enumerate() {
                if d == 0 { zeros.push(i); }
            }
            loop {
                if zeros.is_empty() {
                    break;
                }
                let mut index = zeros.pop().unwrap();
                sequence.push(index);
                indegree[index] -= 1;
                for terminal in &adjacency_list[index] {
                    indegree[*terminal] -= 1;
                    if indegree[*terminal] == 0 {
                        zeros.push(*terminal);
                    }
                }
            }
            return if sequence.len() < num { Err(()) } else { Ok(sequence) };
        }
    }
}