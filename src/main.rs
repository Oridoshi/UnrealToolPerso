use std::{fs, thread};
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use eframe::{egui, App};
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use rfd::FileDialog;
use winapi::ctypes::__int32;
use regex::Regex;
use winapi::um::winuser::PrintWindow;

struct MyApp {
    source_dir: String,
    destination_dir: String,
    output: Arc<Mutex<String>>,  // Partagé entre threads
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            source_dir: String::new(),
            destination_dir: String::new(),
            output: Arc::new(Mutex::new(String::new())), // Initialisation
        }
    }
}

impl MyApp {
    pub fn get_uproject_file(&self) -> Option<String> {
        use std::fs;

        if let Ok(entries) = fs::read_dir(&self.source_dir) {
            for entry in entries.flatten() {
                if let Some(extension) = entry.path().extension() {
                    if extension == "uproject" {
                        if let Some(file_name) = entry.file_name().to_str() {
                            return Some(file_name.to_string());
                        }
                    }
                }
            }
        }
        None
    }
}

fn truncate_text_to_10_lines(text: &str) -> String {
    // Divise le texte en lignes
    let lines: Vec<&str> = text.lines().collect();

    // Si le texte a plus de 10 lignes, on garde uniquement les 10 dernières
    if lines.len() > 10 {
        // On retourne uniquement les 10 dernières lignes
        lines[lines.len() - 10..].join("\n")
    } else {
        // Si le texte fait 10 lignes ou moins, on le laisse tel quel
        text.to_string()
    }
}


impl App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Outil de Build Unreal");

            // Champ pour le dossier source
            ui.label("Dossier source du projet:");
            ui.horizontal(|ui| {
                ui.text_edit_singleline(&mut self.source_dir);
                if ui.button("Browse...").clicked() {
                    if let Some(path) = FileDialog::new().pick_folder() {
                        self.source_dir = path.to_string_lossy().to_string();
                    }
                }
            });

            // Champ pour le dossier de destination
            ui.label("Dossier de destination des builds:");
            ui.horizontal(|ui| {
                ui.text_edit_singleline(&mut self.destination_dir);
                if ui.button("Browse...").clicked() {
                    if let Some(path) = FileDialog::new().pick_folder() {
                        self.destination_dir = path.to_string_lossy().to_string();
                    }
                }
            });

            ui.separator();

            // Vérifier si le dossier de source contient un fichier .uproject
            if let Some(uproject_file) = self.get_uproject_file() {
                // afficher les info du projet par rapport au .uproject
                let file = File::open(format!("{}/{}", self.source_dir, uproject_file)).expect("Failed to open .uproject file");
                let reader = BufReader::new(file);
                let project_info: serde_json::Value = serde_json::from_reader(reader).expect("Failed to parse .uproject file");

                //recupere le nom du projet via le tout ce qu'il y a avant le .uproject
                let project_name = uproject_file[..uproject_file.len() - 9].to_string();

                //recupere la version du moteur
                let mut engine_version= project_info["EngineAssociation"].as_str().unwrap_or("");
                let re = Regex::new(r"\{.*}").unwrap();
                if re.is_match(engine_version) {
                    engine_version = "From Source Version";
                }

                // recupere les plugins
                let empty_vec = vec![];
                let plugins = project_info["Plugins"].as_array().unwrap_or(&empty_vec);
                let mut plugins_list = String::new();
                for plugin in plugins {
                    plugins_list.push_str(&format!("{}{}\n", plugin["Name"].as_str().unwrap_or(""), if plugin["Enabled"].as_bool().unwrap_or(false) { " (Enabled)" } else { "" }));
                }


                // Affichage des informations du projet dans l'interface
                ui.heading("Informations du projet:");
                ui.label(format!("Nom du projet: {}", project_name));
                ui.label(format!("Version du moteur: {}", engine_version));
                ui.label("Plugins:");
                ui.label(plugins_list);

                ui.separator();

                // Si le dossier est valide, afficher les boutons d'action
                ui.horizontal(|ui| {
                    if ui.button("Compiler").clicked() {
                        let output_clone = Arc::clone(&self.output);
                        let source_dir_clone = self.source_dir.clone();
                        let uproject_file_clone = uproject_file.clone();
                        let project_name_clone = project_name.clone();

                        // Vérifie si on est sous windows
                        if(cfg!(target_os = "windows")) {
                            // Lancer le thread pour exécuter la commande
                            thread::spawn(move || {
                                let commandline = format!(".\\Engine\\Build\\BatchFiles\\Build.bat {} Development Win64 \"{}\\{}\" -WaitMutex", project_name_clone, source_dir_clone, uproject_file_clone);
                                let mut command = Command::new("powershell")
                                    .args(["/C", &commandline])
                                    .stdout(Stdio::piped())
                                    .spawn()
                                    .expect("Échec de l'exécution de la commande");

                                if let Some(stdout) = command.stdout.take() {
                                    let mut reader = BufReader::new(stdout);
                                    let mut buffer = Vec::new();

                                    // Lire la sortie ligne par ligne
                                    loop {
                                        buffer.clear();
                                        match reader.read_until(b'\n', &mut buffer) {
                                            Ok(0) => {
                                                let mut output = output_clone.lock().unwrap();
                                                output.push_str("COMPILATION TERMINEE\n");
                                                break
                                            },
                                            Ok(_) => {
                                                let line = String::from_utf8_lossy(&buffer);
                                                let mut output = output_clone.lock().unwrap();
                                                output.push_str(&format!("{}", line));
                                            }
                                            Err(e) => {
                                                eprintln!("Erreur de lecture : {}", e);
                                                break;
                                            }
                                        }
                                    }
                                }

                                let output = command.wait().expect("Erreur lors de l'attente de la commande");
                                if !output.success() {
                                    eprintln!("La commande a échoué avec le code : {}", output.code().unwrap_or(-1));
                                }
                            });
                        } else if cfg!(target_os = "macos") {
                            //TODO: Ajouter la commande pour MacOS
                        } else {
                            eprintln!("OS non supporté");
                        }
                    }

                    if !self.destination_dir.is_empty() && fs::metadata(&self.destination_dir).is_ok() {
                        if ui.button("Package").clicked() {
                            let output_clone = Arc::clone(&self.output);
                            let source_dir_clone = self.source_dir.clone();
                            let uproject_file_clone = uproject_file.clone();
                            let project_dest_clone = self.destination_dir.clone();

                             // Vérifie si on est sous windows
                            if cfg!(target_os = "windows")  {
                                // Lancer le thread pour exécuter la commande
                                thread::spawn(move || {
                                    let mut command = Command::new("powershell")
                                        .args(["/C", ".\\Engine\\Build\\BatchFiles\\RunUAT.bat",&format!("-ScriptsForProject=\"{}\\{}\"", source_dir_clone, uproject_file_clone), "BuildCookRun",  &format!("-project=\"{}\\{}\"", source_dir_clone, uproject_file_clone), "-noP4", "-clientconfig=Development", "-serverconfig=Development", "-nocompileeditor", "-utf8output", "-platform=Win64", "-build", "-cook", "-unversionedcookedcontent", "-stage", "-package", &format!("-stagingdirectory=\"{}\"", project_dest_clone), "-cmdline=\"-Messaging\""])
                                        .stdout(Stdio::piped())
                                        .spawn()
                                        .expect("Échec de l'exécution de la commande");

                                    if let Some(stdout) = command.stdout.take() {
                                        let mut reader = BufReader::new(stdout);
                                        let mut buffer = Vec::new();

                                        // Lire la sortie ligne par ligne
                                        loop {
                                            buffer.clear();
                                            match reader.read_until(b'\n', &mut buffer) {
                                                Ok(0) => {
                                                    let mut output = output_clone.lock().unwrap();
                                                    output.push_str("PACKAGING TERMINEE\n");
                                                    break
                                                },
                                                Ok(_) => {
                                                    let line = String::from_utf8_lossy(&buffer);
                                                    let mut output = output_clone.lock().unwrap();
                                                    output.push_str(&format!("{}", line));
                                                }
                                                Err(e) => {
                                                    eprintln!("Erreur de lecture : {}", e);
                                                    break;
                                                }
                                            }
                                        }
                                    }

                                    let output = command.wait().expect("Erreur lors de l'attente de la commande");
                                    if !output.success() {
                                        eprintln!("La commande a échoué avec le code : {}", output.code().unwrap_or(-1));
                                    }
                                });
                            } else if cfg!(target_os = "macos") {
                                //TODO: Ajouter la commande pour MacOS
                            }
                        }
                    } else {
                        ui.label("Veuillez choisir un dossier de destination valide pour package.");
                    }
                });

                // Demande de mise à jour de l'interface
                ctx.request_repaint();

                ui.label("Output:");
                let output_content = truncate_text_to_10_lines(&self.output.lock().unwrap());
                ui.monospace(&output_content);

            } else {
                // Sinon, afficher un message d'erreur
                ui.label("Le dossier source est invalide. Veuillez choisir un dossier contenant un fichier .uproject.");
            }
        });
    }
}

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Unreal Build Tool",
        native_options,
        Box::new(|_cc| Box::new(MyApp::default())),
    ).expect("Erreur de lancement de l'application");
}