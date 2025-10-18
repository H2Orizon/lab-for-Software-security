pub fn vigenere_cipher() {
    let c  = "криптографічніметодизахистуінформації";
    let key_word: Vec<char> = "ференц".chars().collect();
    let vector_abs: Vec<char> = "абвгґдеєжзиіїйклмнопрстуфхцчшщьюя".chars().collect();
    let abs_powr: usize = vector_abs.len();
    println!("C: {}", c);

    let crypt = crypt_lab5_2(&key_word, &vector_abs, abs_powr, c);
    println!("Encrypted: {}", crypt);

    let decrypted = decrypt_lab5_2(&key_word, &vector_abs, abs_powr, &crypt);
    println!("Decrypted: {}", decrypted);
}

fn crypt_lab5_2 (key_word:&Vec<char>, vector_abs:&Vec<char>, abs_powr: usize, c:&str) -> String{
    let mut result = String::new();
        for (i,ch) in c.chars().enumerate(){
        let key_char = key_word[i % key_word.len()];
        if let Some(p_inx) = vector_abs.iter().position(|&c| c == ch){ 
            let key_ind = vector_abs.iter().position(|&c| c == key_char).unwrap();
            let enc_inx = (p_inx + key_ind) % abs_powr;
            result.push(vector_abs[enc_inx])
        }else{
            result.push(ch)
        }
    }
    return result;
}

fn decrypt_lab5_2 (key_word: &Vec<char>, vector_abs:&Vec<char>, abs_powr: usize, crypt:&str) -> String{
    let mut result = String::new();
    for (i,ch) in crypt.chars().enumerate(){
        let key_char = key_word[i % key_word.len()];
        if let Some(p_inx) = vector_abs.iter().position(|&result| result == ch){ 
            let key_ind = vector_abs.iter().position(|&c| c == key_char).unwrap();
            let enc_inx = (p_inx + abs_powr - key_ind) % abs_powr;
            result.push(vector_abs[enc_inx])
        }else{
            result.push(ch)
        }
    }
    return result;
}