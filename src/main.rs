/*
* Copyright (c) 2022 - Jefferson T.
* Telegrão @StalinCCCP
* This program is free software: you can redistribute it and/or modify it under 
* the terms of the GNU General Public License as published by the Free Software 
* Foundation, either version 3 of the License, or (at your option) any later 
* version. This program is distributed in the hope that it will be useful, but 
* WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or
* FITNESS FOR A PARTICULAR PURPOSE.  See the GNU General Public License for 
* more details. You should have received a copy of the GNU General Public 
* License along with this program.  If not, see <https://www.gnu.org/licenses/> 
*/
extern crate diacritics;
mod entrada;
mod trims;
use {   regex::Regex,
        entrada::*,
        trims::*,
        diacritics::*,
};

fn main() {
/* Variáveis */    
let mut linhas: Vec<String> = entrada();
let termos = prompt();
let mut index_notas: Vec<usize> = Vec::new();

/* Regex de âncoras*/
let refer = Regex::new(&format!(r"::ref")).unwrap();
let link = Regex::new(&format!(r"::lin")).unwrap();
let local = Regex::new(&format!(r"::loc")).unwrap();

/* Busca */
for term in termos {
    let termo = remove_spaces(&term).to_string();
    let busca = Regex::new(&format!(r"(?i){}", &termo)).unwrap();

    linhas = linhas.into_iter()
        .filter(|x| 
            link.is_match(&remove_diacritics(x)) || 
            busca.is_match(&remove_diacritics(x)) || 
            busca.is_match(x) || 
            refer.is_match(x) || 
            local.is_match(x) || 
            x.is_empty())
        .collect();
}

/* Carrega o index das notas  */
for (index, linha) in linhas.iter().enumerate(){
    if link.is_match(linha) == false && 
       linha.is_empty() == false && 
       refer.is_match(linha) == false && 
       local.is_match(linha) == false{
            index_notas.push(index);
    }
}

/* Printa resultados */
for resultado in index_notas {
    let mut i = 0;
    let mut n = 0;
    let mut f = 0;
    let mut k = 0;
    println!("\n{}", &linhas[resultado].trim_start());
    let mut noref = false;
    let mut nolin = false;
    let mut noloc = false;
    while linhas[resultado+n].is_empty() == false { 
        n = n + 1; //bottom
        if resultado + n >= linhas.len() { break};
    }

    while refer.is_match(&linhas[resultado+i]) == false {
        i = i + 1; // incrementa i até achar ::ref
        if resultado + i >= linhas.len() - 1 {
            noref = true;
            break
        }
    };
    while local.is_match(&linhas[resultado+k]) == false {
        k = k + 1; // incrementa k até achar ::loc
        if resultado + k >= linhas.len() - 1 {
            noloc = true; // uma âncora ::loc não foi encontrada até o EOF
            break
        }
    };
    while link.is_match(&linhas[resultado+f]) == false {
        f = f + 1; // incrementa k até achar ::loc
        if resultado + f >= linhas.len() - 1 {
            nolin = true; // uma âncora ::loc não foi encontrada até o EOF
            break
        }
    };

    if noref == false && i <= n { // âncora limitada pelo bottom
        println!("\x1b[31;1m⚓︎ Referência: \x1b[0m\x1b[33m{}\x1b[0m", trim_ref(&linhas[resultado+i], 5));
    } 
    if noloc == false && k <= n { // âncora limitada pelo bottom
        println!("\x1b[32;1m⚓︎ Localização: \x1b[0m\x1b[33m{}\x1b[0m", trim_ref(&linhas[resultado+k], 5));
    }
    if nolin == false && f <= n { // âncora limitada pelo bottom
        println!("\x1b[32;1m⚓︎ Link: \x1b[0m\x1b[33m{}\x1b[0m", trim_ref(&linhas[resultado+f], 5));
    }
    if nolin == true &&
       noloc == true &&
       noref == true {
            println!("\x1b[31;1m :( nota sem âncora! \x1b[0m");
    }
}
}
