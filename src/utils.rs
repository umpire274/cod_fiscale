use log::debug;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufReader, Error};
use std::path::Path;
use std::io;
use chrono::NaiveDate;
use chrono::Datelike;

fn estrai_consonanti_vocali(s: &str) -> (String, String) {
    let mut consonanti = String::new();
    let mut vocali = String::new();
    
    for c in s.chars() {
        if c.is_alphabetic() {
            match c {
                'A' | 'E' | 'I' | 'O' | 'U' | 'a' | 'e' | 'i' | 'o' | 'u' => vocali.push(c.to_ascii_uppercase()),
                _ => consonanti.push(c.to_ascii_uppercase()),
            }
        }
    }
    
    (consonanti, vocali)
}

pub fn genera_codice_nome(nome: &str) -> String {
    let (consonanti, vocali) = estrai_consonanti_vocali(nome);
    let mut codice = String::new();
    
    if consonanti.len() >= 4 {
        codice.push(consonanti.chars().nth(0).unwrap());
        codice.push(consonanti.chars().nth(2).unwrap());
        codice.push(consonanti.chars().nth(3).unwrap());
    } else {
        codice = format!("{}{}", consonanti, vocali);
        codice = codice.chars().take(3).collect();
        while codice.len() < 3 {
            codice.push('X');
        }
    }
    
    codice
}

pub fn genera_codice_cognome(cognome: &str) -> String {
    let (consonanti, vocali) = estrai_consonanti_vocali(cognome);
    let mut codice = String::new();
    if consonanti.len() >= 3 {
        codice = consonanti.chars().take(3).collect();
    } else {
        codice = format!("{}{}", consonanti, vocali);
        codice = codice.chars().take(3).collect();
        while codice.len() < 3 {
            codice.push('X');
        }
    }
    
    codice
}

/// Genera il codice della data di nascita per il codice fiscale.
pub fn genera_codice_data_nascita(anno: u32, mese: u32, giorno: u32, sesso: char) -> String {
    let anno_codice = format!("{:02}", anno % 100);
    let mese_codice = codice_mese(mese);
    let giorno_codice = if sesso == 'F' { giorno + 40 } else { giorno };
    format!("{}{}{:02}", anno_codice, mese_codice, giorno_codice)
}

/// Converte il numero del mese nel codice per il codice fiscale.
fn codice_mese(mese: u32) -> String {
    let codici = ["A", "B", "C", "D", "E", "H", "L", "M", "P", "R", "S", "T"];
    codici.get((mese - 1) as usize).unwrap_or(&"X").to_string()
}

pub fn genera_codice_controllo(codice: &str) -> char {
    let pari = [
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 
        10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 
        20, 21, 22, 23, 24, 25
    ];

    let dispari = [
        1, 0, 5, 7, 9, 13, 15, 17, 19, 21, 
        1, 0, 5, 7, 9, 13, 15, 17, 19, 21, 
        2, 4, 18, 20, 11, 3, 6, 8, 12, 14, 
        16, 10, 22, 25, 24, 23
    ];

    let alfabeto = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    
    let mut somma = 0;
    
    for (i, c) in codice.chars().enumerate() {
        if let Some(pos) = alfabeto.find(c) {
            somma += if i % 2 == 0 { dispari[pos] } else { pari[pos] };
        } else {
            panic!("Carattere non valido nel codice fiscale.");
        }
    }
    
    let resto = somma % 26;
    (b'A' + resto as u8) as char
}

/// Legge e verifica una data di nascita inserita dall'utente.
pub fn leggi_e_controlla_data_nascita(prompt: &str, data_nascita: &str) -> Option<(u32, u32, u32)> {
    let mut input = String::new();

    loop {
        if data_nascita.len() == 0 {
            println!("{}", prompt);
            io::stdin().read_line(&mut input).unwrap();
            input = input.trim().to_string();
        } else {
            input = data_nascita.to_string();
        }

        if let Some((giorno, mese, anno)) = valida_data(&input) {
            return Some((giorno, mese, anno));
        } else {
            println!("Data non valida. Inserisci una data nel formato dd/mm/yyyy.");
            input.clear();
        }
    }
}

/// Verifica se una data nel formato `dd/mm/yyyy` è valida.
fn valida_data(data: &str) -> Option<(u32, u32, u32)> {
    let parts: Vec<&str> = data.split('/').collect();
    if parts.len() != 3 {
        return None;
    }

if let (Ok(giorno), Ok(mese), Ok(anno)) = (parts[0].parse::<u32>(), parts[1].parse::<u32>(), parts[2].parse::<i32>()) {
        if let Some(date) = NaiveDate::from_ymd_opt(anno, mese, giorno) {
            return Some((date.day(), date.month(), date.year() as u32));
        }
    }
    
    None
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Comune {
    sigla_provincia: String,
    denominazione_ita: String,
    codice_belfiore: String,
}

pub fn carica_comuni<P: AsRef<Path>>(path: P) -> Result<Vec<Comune>, Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let comuni: Vec<Comune> = serde_json::from_reader(reader)?;
    debug!("Letti num. {} comuni", comuni.len());
    Ok(comuni)
}

pub fn trova_codice_comune(comuni: &[Comune], nome: &str, provincia: &str) -> Option<String> {
    debug!("nome comune: {}, provincia  : {}",nome,provincia);
    comuni.iter().find_map(|comune| {
        if comune.denominazione_ita.eq_ignore_ascii_case(nome) &&
           comune.sigla_provincia.eq_ignore_ascii_case(provincia) {
            Some(comune.codice_belfiore.clone())
        } else {
            None
        }
    })
}

pub fn calcola_codice_fiscale(cognome: &str, nome: &str, codice_data_nascita: &str, codice_comune: &str, codice_controllo: char) -> String {
    // Implementa la generazione del codice fiscale qui
    format!("{}{}{}{}{}", cognome, nome, codice_data_nascita, codice_comune, codice_controllo)
}

