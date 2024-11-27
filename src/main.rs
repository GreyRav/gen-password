use eframe::egui;

// Structure pour stocker l'état de notre application
struct PasswordGeneratorApp {
    input_text: String,
    generated_password: String,
}

impl Default for PasswordGeneratorApp {
    fn default() -> Self {
        Self {
            input_text: String::new(),
            generated_password: String::new(),
        }
    }
}

impl PasswordGeneratorApp {
    fn generate_password(&self, input: &str) -> String {
        // Faire les remplacements en premier
        let modified_input = input
            .replace("and", "&")
            .replace("at", "@")
            .replace("to", "2")
            .replace("for", "4")
            .replace("one", "1")
            .replace("too", "2")
            // Mots français conservés
            .replace("et", "&")
            .replace("de", "2")
            // Remplacements des caractères
            .replace('a', "@")
            .replace('i', "1")
            .replace('o', "0")
            .replace('s', "$")
            .replace('I', "1");
        
        let mut first_letters = Vec::new();
        let mut is_new_word = true;
        
        // Ensuite prendre les premières lettres
        for c in modified_input.chars() {
            if c.is_whitespace() {
                is_new_word = true;
                continue;
            }
            
            if c == '.' || c == ',' || c == '\'' || c == '!' || c == '?' {
                first_letters.push(c);
                continue;
            }
            
            if is_new_word {
                first_letters.push(c);
                is_new_word = false;
            }
        }
        
        first_letters.into_iter().collect()
    }
}

impl eframe::App for PasswordGeneratorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Titre
            ui.heading("Générateur de mot de passe");
            
            // Espacement
            ui.add_space(20.0);
            
            // Zone de texte multiligne pour la phrase
            ui.label("Entrez votre phrase :");
            let text_edit = egui::TextEdit::multiline(&mut self.input_text)
                .desired_rows(3)
                .desired_width(f32::INFINITY);
            ui.add(text_edit);
            
            // Bouton de génération
            if ui.button("Générer le mot de passe").clicked() {
                self.generated_password = self.generate_password(&self.input_text);
            }
            
            // Espacement
            ui.add_space(20.0);
            
            // Affichage du mot de passe généré
            if !self.generated_password.is_empty() {
                ui.horizontal(|ui| {
                    ui.label("Mot de passe généré :");
                    ui.label(egui::RichText::new(&self.generated_password)
                        .monospace()
                        .strong()
                        .size(18.0));
                    
                    // Bouton de copie
                    if ui.button("📋 Copier").clicked() {
                        ui.output_mut(|o| o.copied_text = self.generated_password.clone());
                    }
                });
            }
        });
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([600.0, 400.0])
            .with_min_inner_size([300.0, 200.0]),
        ..Default::default()
    };
    
    eframe::run_native(
        "Générateur de mot de passe",
        options,
        Box::new(|_cc| Ok(Box::new(PasswordGeneratorApp::default()))),
    )
}
