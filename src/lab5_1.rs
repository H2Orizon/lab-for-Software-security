pub fn caesars_cipher() {
    let vector_c: Vec<char> = "vppanlwxlyopyncjae".chars().collect();
    let vector_abs: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
    let abc_powr: usize = vector_abs.len();

    for key in 1..abc_powr{
        print!("{}. ",key);
        for ch in &vector_c {
            let inx = match ch.is_ascii_lowercase() {
                true => (*ch as u8 - b'a') as usize,
                false => {
                    print!("{}", ch);
                    continue;
                }
            };
            let temp = (inx + abc_powr - key) % abc_powr;
            print!("{}", vector_abs[temp]);
        }
        println!();
    }
}