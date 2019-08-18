pub const MAZE: &[&[char]] = &[
    &['#','S','#','#','#','#',],
    &['#','.','.','.','.','G',],
    &['#','.','#','#','#','#',],
    &['#','.','.','.','#','#',],
    &['#','#','#','#','#','#',],
    &['#','#','#','#','#','#',],  
];

const INF: usize = 10000000;
const INFARR: [usize;M] = [INF;M];
const N: usize = 6;
const M: usize = 6;

// 移動4方向
const DX: [isize;4] = [1,0,-1,0];
const DY: [isize;4] = [0,1,0,-1];

pub fn maze(path: &[&[char]]){
    let mut queue: Vec<Point> = Vec::new();
    // 各点までの最短距離の配列
    let mut d: [[usize;M];N] = [INFARR,INFARR,INFARR,INFARR,INFARR,INFARR];

    let (start, goal) = maze_point(path);
    let start_x = start.x;
    let start_y = start.y;

    queue.push(start);
    d[start_x][start_y] = 0;

    while queue.len() != 0 {
        // キューの先頭を取り出す
        let p = queue.pop().unwrap();
        // 取り出してきた状態がゴールなら探索終了
        if p.x == goal.x && p.y == goal.y {
            break;
        } 

        //移動四方向をループ
        for i in 0..3{
            let nx = p.x as isize + DX[i]; let ny = p.y as isize + DY[i];

            if 0 <= nx 
            && nx < N as isize 
            && 0 <= ny 
            && ny < M as isize 
            && path[nx as usize][ny as usize] != '#' 
            && d[nx as usize][ny as usize] == INF{
                // 移動できるならキューに入れ、その点の距離をpからの距離+1でマーク
                queue.push(Point{x: nx as usize, y: ny as usize});
                d[nx as usize][ny as usize] = d[p.x][p.y] + 1;
            }
        }
    }

    println!("{}", d[goal.x][goal.y]);
}

fn maze_point(path: &[&[char]]) -> (Point, Point){
    let mut sx = INF; let mut sy = INF;
    let mut gx = INF; let mut gy = INF;

    for i in 0..N{
        for j in 0..M{
            if path[i][j] == 'S'{
                sx = i; sy = j;
            }
            if path[i][j] == 'G'{
                gx = i; gy = j;
            }
        }
    }

    assert!(sx != INF);assert!(sy != INF);
    assert!(gx != INF);assert!(gy != INF);

    (Point{x: sx, y: sy}, Point{x: gx, y: gy})
}

struct Point{
    x: usize, 
    y: usize
}

