fn main(){
    // bi color graph?
    // dfsで色塗って行けばよさそう？
    // 全て白からスタート
    // 0をもとにはじめ、1と2を黒に
    // 0: w 1:b 2: b
    // 1:b -> 0は白なので妥当、すでに黒で塗られた2は妥当でない。
    // 終了
    // 
    // 再度塗ることはできない、という直感
    //
    // 0:w 1:b 2:w 3:b
    // 1: b -> 0 ok 2 ok
    // 2: w -> 1:b ok 3:b ok
    // 3: b -> ok!
    // さて、これでは各頂点ごとにやる必要があってだるい
    // けどとりあえずやってみるか！

    let graph = [
        vec![1,3],vec![0,2],vec![1,3],vec![0,2],
    ];
    let mut color = [false;4];

    // dfsで各頂点に対して処理を行う！
    for (i,v) in graph.iter().enumerate(){
        let v_color = color[i];


        for &relate_v in v.iter(){

            let relate_v_color = color[relate_v];

            match color_check(v_color, relate_v_color){
                Check::Update => color[relate_v] = true,
                Check::Err => {
                    println!("NO");
                    panic!();
                },
                _ => ()    
            }
        }

    }

    println!("OK!");
}

fn color_check(v_color: bool, relate_v_color: bool) -> Check{
    if v_color == false && relate_v_color == false{
        return Check::Update;
    }
    if v_color == false && relate_v_color == true{
        return Check::Ok;
    }
    if v_color == true && relate_v_color == false{
        return Check::Ok;
    }

    return Check::Err;
}

enum Check{
    // 色を更新すべきか、妥当な色か、妥当でない結果となったか
    Update, Ok, Err
}