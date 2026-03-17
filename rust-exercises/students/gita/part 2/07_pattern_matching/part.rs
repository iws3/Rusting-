// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter,
// }

// fn value_in_cents(coin:Coin)->u8 {
//     match coin {
//         Coin::Penny=>1,
//         Coin::Nickel=>5,
//         Coin::Dime=>10,
//         Coin::Quarter=>25
//     }
// }

// fn main() {
//     let coin=Coin::Dime;
//     println!("Value: {}", value_in_cents(coin));
// }



// enum Players {
//     Messi,
//     Ronaldo,
//     Neymar,
//     Mbappe
// }


// fn player_number(player:Player)->u8 {
//     match playder {
//         Player::Messi=>10,
//         Player::Ronaldo=>7,
//         Neymar=>11,
//         Mbappe=>7
//     }
// }


// fn main() {
//     let player=Player::Messi;
//     println!("Player Number: {}", player_number(player));
// }




// Pattern can bind to values inside enum variaints

#[derive(Debug)]
enum CamRegion {
    North,
    South,
    East,
    West,
}



enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(CamRegion),
}


fn value_in_cents(coin:Coin)->u8 {
match coin {
    Coin::Penny=>1,
    Coin::Nickel=>5,
    Coin::Dime=>10,
    Coin::Quarter(region)=>{
        println!("Quarter from {:?}!", region);
        23
    }
}

}

fn main() {
    let coin=Coin::Quarter(CamRegion::North);
    println!("Value: {}", value_in_cents(coin));

}