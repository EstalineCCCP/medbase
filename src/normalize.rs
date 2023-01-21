pub fn noacentos (s: String) -> String {
    let chars: Vec<char> = s.chars().collect::<Vec<_>>();
    let mut out: Vec<char> = chars.clone();
//Loop through the vector
for (i, c) in chars.iter().enumerate() {
// Substitute the character if it is 'c'
    match c {
        'À' | 'Á' | 'Â' | 'Ã' | 'Ä' | 'Å' | 'Æ'| 'Þ' | 'Ç' | 'Č' | 'Ď' | 'Ð' |'Ě' | 'È' | 'É' | 'Ê' | 'Ë' | 'Ƒ' |'Ì' | 'Í' | 'Î' | 'Ï' | 'Ň' | 'Ñ' | 'Ò' | 'Ó' | 'Ô' | 'Õ' | 'Ö' | 'Ø' | 'Ř' | 'ß' | 'Š' | 'Ť' | 'Ů' | 'Ù' | 'Ú' | 'Û' | 'Ü' | 'Ý' | 'Ž' | 'à' | 'á' | 'â' | 'ã' | 'ä' | 'å' | 'æ' | 'þ' | 'ç' | 'č' | 'ď' | 'ð' | 'ě' | 'è' | 'é' | 'ê' | 'ë' | 'ƒ' | 'ì' | 'í' | 'î' | 'ï' | 'ñ' | 'ň' | 'ò' | 'ó' | 'ô' | 'õ' | 'ö' | 'ø' | 'ř' | 'š' | 'ť' | 'ů' | 'ù' | 'ú' | 'û' | 'ü' | 'ý' | 'ÿ' | 'ž' => out[i] = '*',
        _ => out[i] = *c,
         };
}
    let saida: String = out.iter().collect::<String>();
    return saida
}
