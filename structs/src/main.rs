fn main() {
    struct Bowler_stats {
        name: String,
        matches: u64,
        wickets: u64,
        retired:bool,
        best: (u32, u32),
    }

    let Stark = Bowler_stats {
        name: String::from("Mitchell Stark"),
        wickets: 382,
        retired: false,
        best:(6/48)

    }
    let Steyn = Bowler_stats{
        name:String::from("Dale Steyn"),
        ..Stark
    }

}

fn build_bowler(String: name, wickets:u64)->Bowler_stats{
Bowler_stats{
    name,
    wickets,
    retired:false,
    best:(6,48),
}
}
