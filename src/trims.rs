pub fn trim_ref(entrada: &str, corte: usize) -> String{
    let linha: &str = entrada;
/* corte: tamanho em chars da âncora */
    let lin: Vec<char> = linha[corte..].chars().collect::<Vec<_>>();
    if lin.len() < 1 {return linha.to_string()}else{
        let mut i: usize = 0;
        while lin[i] == ' ' || lin[i] == '\t' {i = i + 1};
        return lin[i..].iter().collect::<String>()
    }
}

pub fn remove_spaces(entrada: &str) -> String {
    let s = entrada.to_string();
    s.trim_start().trim_end().to_string();
    return s
}
