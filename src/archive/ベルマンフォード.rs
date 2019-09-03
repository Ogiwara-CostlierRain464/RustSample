struct Edge{
    from: usize, to: usize, cost: usize
}

const INF: usize = 10_0000;

fn main(){

    let edges = [
        Edge { from: 0, to: 2, cost: 2 },
        Edge { from: 0, to: 1, cost: 5 },
        Edge { from: 1, to: 2, cost: 4 },
        Edge { from: 2, to: 3, cost: 6 },
        Edge { from: 1, to: 3, cost: 2 },
        Edge { from: 2, to: 4, cost: 10 },
        Edge { from: 3, to: 5, cost: 1 },
        Edge { from: 4, to: 5, cost: 3 },
        Edge { from: 4, to: 6, cost: 5 },
        Edge { from: 5, to: 6, cost: 9 },
    ];

    let mut min_d = [INF;7];
    // s番目の頂点から各頂点への最短距離
    let s = 0;
    min_d[s] = 0;
    loop {
        let mut update = false;
        for i in 0..edges.len(){
            let edge = &edges[i];
            if min_d[edge.from] != INF && min_d[edge.to] > min_d[edge.from] + edge.cost{
                // より小さい経路が見つかったので更新
                min_d[edge.to] = min_d[edge.from] + edge.cost;
                update = true;
            }

            println!("{:?}", min_d);
        }
        if !update { break; }
        println!("LOOP");
    }

    println!("{:?}", min_d);
}