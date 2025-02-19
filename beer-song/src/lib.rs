pub fn verse(n: u32) -> String {
    if n == 0 {
        return "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.".to_string();
    }
    let o = n-1;
    format!("{} bottle{} of beer on the wall, {} bottle{} of beer.\nTake {} down and pass it around, {} bottle{} of beer on the wall.\n\n", 
            n, if n > 1 { "s" } else { "" }, 
            n, if n > 1 { "s" } else { "" },
            if n > 1 { "one"} else { "it"},
            if o == 0 { "no more".to_string() } else { o.to_string() },
            if o == 0 || o > 1 { "s" } else { "" })
}

pub fn sing(start: u32, end: u32) -> String {
    let mut output= String::from("");
    for i in (end..=start).rev() {
        println!("i = {}",i);
        output = format!("{}{}",output,verse(i));
    }
    output
}
