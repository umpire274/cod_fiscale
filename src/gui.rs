use std::process::exit;
use log::info;
use eframe::egui;
use crate::utils::{calcola_codice_fiscale,trova_codice_comune,carica_comuni,leggi_e_controlla_data_nascita,genera_codice_nome,genera_codice_cognome,genera_codice_data_nascita,genera_codice_controllo};


pub struct CodiceFiscaleApp {
    nome: String,
    cognome: String,
    sesso: char,
    data_nascita: String,
    comune: String,
    provincia: String,
    codice_fiscale: String,
    verifica: bool,
}

impl Default for CodiceFiscaleApp {
    fn default() -> Self {
        Self {
            nome: String::new(),
            cognome: String::new(),
            sesso: 'M',
            data_nascita: String::new(),
            comune: String::new(),
            provincia: String::new(),
            codice_fiscale: String::new(),
            verifica: false,
        }
    }
}

impl eframe::App for CodiceFiscaleApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Generatore Codice Fiscale");

            ui.horizontal(|ui| {
                ui.label("Cognome:");
                ui.text_edit_singleline(&mut self.cognome);
            });

            ui.horizontal(|ui| {
                ui.label("Nome:");
                ui.text_edit_singleline(&mut self.nome);
            });

            ui.horizontal(|ui| {
                ui.label("Sesso:");
                ui.radio_value(&mut self.sesso, 'M', "Maschio");
                ui.radio_value(&mut self.sesso, 'F', "Femmina");
            });

            ui.horizontal(|ui| {
                ui.label("Data di nascita (gg/mm/yyyy):");
                ui.text_edit_singleline(&mut self.data_nascita);
            });

            ui.horizontal(|ui| {
                ui.label("Comune di nascita:");
                ui.text_edit_singleline(&mut self.comune);
            });

            ui.horizontal(|ui| {
                ui.label("Provincia (sigla):");
                ui.text_edit_singleline(&mut self.provincia);
            });

            ui.checkbox(&mut self.verifica, "Verifica codice fiscale");

            if ui.button("Genera").clicked() {
                if self.verifica {
                    // Verifica codice fiscale
                    self.codice_fiscale = if self.codice_fiscale.len() == 16 {
                        "Codice fiscale valido".to_string()
                    } else {
                        "Codice fiscale non valido".to_string()
                    };
                } else {
                	// carico i comuni
            	    let comuni = match carica_comuni("comuni.json") {
				        Ok(data) => data,
				        Err(e) => {
				            eprintln!("Errore nel caricamento del file JSON: {}", e);
				            return;
				        }
				    };

                	// genero i vari codici
                	let codice_nome = genera_codice_nome(&self.nome);
                	let codice_cognome = genera_codice_cognome(&self.cognome);
					let mut codice_comune: Option<String> = None;
				    // Trova il codice comune
				    match trova_codice_comune(&comuni, &&self.comune, &self.provincia) {
				        Some(codice) => {
				            info!("Codice Belfiore per {}, {}: {}", self.comune, self.provincia, codice);
				            codice_comune = Some(codice);
				        }
				        None => {
				            println!("\nComune di {}, in provincia di {} non trovato.", self.comune, self.provincia);
				            exit(1);
				        }
				    }
					let mut codice_data = String::new();
					if let Some((giorno, mese, anno)) = leggi_e_controlla_data_nascita("Inserisci la data di nascita (dd/mm/yyyy):",&self.data_nascita) {
					        codice_data = genera_codice_data_nascita(anno, mese, giorno, self.sesso);
					    } else {
					        println!("Errore nella lettura della data.");
					    }

					let codice_base = format!(
				        "{}{}{}{}", 
				        codice_cognome, 
				        codice_nome, 
				        codice_data, 
				        codice_comune.as_deref().unwrap_or("XXXXX")
				    );
				    let codice_controllo = genera_codice_controllo(&codice_base);

                    // Genera codice fiscale
                    self.codice_fiscale = calcola_codice_fiscale(
                        &codice_cognome,
                        &codice_nome,
                        &codice_data,
                        &codice_comune.as_deref().unwrap_or("XXXX"),
                        codice_controllo
                    );
                }
            }

            if ui.button("Pulisci campi").clicked() {
                self.nome.clear();
                self.cognome.clear();
                self.sesso = 'M';
                self.data_nascita.clear();
                self.comune.clear();
                self.provincia.clear();
                self.codice_fiscale.clear();
            }

            ui.separator();
            ui.label("Codice Fiscale:");
            ui.text_edit_singleline(&mut self.codice_fiscale);
        });
    }
}
