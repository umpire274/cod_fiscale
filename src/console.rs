use crate::utils::{trova_codice_comune,carica_comuni,leggi_e_controlla_data_nascita,genera_codice_nome,genera_codice_cognome,genera_codice_data_nascita,genera_codice_controllo};
use std::process::exit;
use std::io;
use log::info;

pub fn console() {
    let comuni = match carica_comuni("comuni.json") {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Errore nel caricamento del file JSON: {}", e);
            return;
        }
    };

    let mut input = String::new();
    
    println!("Inserisci il nome:");
    io::stdin().read_line(&mut input).unwrap();
    let nome = input.trim().to_string();
    input.clear();
    
    println!("Inserisci il cognome:");
    io::stdin().read_line(&mut input).unwrap();
    let cognome = input.trim().to_string();
    input.clear();
    
    println!("Inserisci il sesso (M/F):");
    io::stdin().read_line(&mut input).unwrap();
    let sesso = input.trim().chars().next().unwrap();
    input.clear();
    
    let mut codice_data = String::new();
    if let Some((giorno, mese, anno)) = leggi_e_controlla_data_nascita("Inserisci la data di nascita (dd/mm/yyyy):", "") {
        codice_data = genera_codice_data_nascita(anno, mese, giorno, sesso);
    } else {
        println!("Errore nella lettura della data.");
    }

    println!("Inserisci il comune di nascita:");
    io::stdin().read_line(&mut input).unwrap();
    let nome_comune = input.trim().to_string();
    input.clear();
    println!("Inserisci la provincia di nascita:");
    io::stdin().read_line(&mut input).unwrap();
    let provincia = input.trim().to_string();

    let codice_nome = genera_codice_nome(&nome);
    let codice_cognome = genera_codice_cognome(&cognome);

    let mut codice_comune: Option<String> = None;
    // Trova il codice comune
    match trova_codice_comune(&comuni, &nome_comune, &provincia) {
        Some(codice) => {
            info!("Codice Belfiore per {}, {}: {}", nome_comune, provincia, codice);
            codice_comune = Some(codice);
        }
        None => {
            println!("\nComune di {}, in provincia di {} non trovato.", nome_comune, provincia);
            exit(1);
        }
    }

    // Stampa i codici
    info!("Codice Cognome: {}", codice_cognome);
    info!("Codice Nome: {}", codice_nome);
    info!("Codice Data di Nascita: {}", codice_data);
    info!("Codice Comune: {}", codice_comune.as_deref().unwrap_or("XXXX"));

    // Genera il codice fiscale
    let codice_base = format!(
        "{}{}{}{}", 
        codice_cognome, 
        codice_nome, 
        codice_data, 
        codice_comune.as_deref().unwrap_or("XXXXX")
    );
    let codice_controllo = genera_codice_controllo(&codice_base);
    let codice_fiscale = format!("{}{}", codice_base, codice_controllo);

    // Stampa il risultato finale
    info!("Carattere di controllo: {}", codice_controllo);
    println!("\nCodice Fiscale: {}\n", codice_fiscale);
}
