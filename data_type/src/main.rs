fn main() {
    let s = format!("{1}是个有着{0:>0width$}KG重，{height:?}cm高的大胖子",
                    81,
                    "wayslog",
                    width = 4,
                    height = 178);
    // 我被逼的牺牲了自己了……
    print!("{}", s);
}
