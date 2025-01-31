use serde::{Deserialize, Serialize};
use serde_json::to_writer_pretty;
use std::fs::File;
use std::io::{self, BufReader, BufWriter};
use std::path::Path;
use std::time::Instant;

use egui::{Color32, Context, Key, Label, ScrollArea, Ui};

use rerun::external::{
    eframe,
    re_ui::{list_item, UiExt},
    re_viewer,
};

#[derive(Default)]
pub struct ControlStates {
    pub last_resource_update: Option<Instant>,
    pub controls_view: ControlsView,
    pub entity_path: String,
    pub position: (f32, f32, f32),
    pub half_size: (f32, f32, f32),
    pub radius: f32,
    pub dynamic_offset: f32,
    pub dynamic_radius: f32,

    pub annotation_text: String,
    pub protocol_text: String,
    pub error_text: String,
    annotations: Vec<Annotation>,
    protocol_anns: Vec<Annotation>,
    errors: Vec<Annotation>,

    protocols: Vec<Annotation>,
    pub next_annotation_id: usize,
}

#[derive(Default)]
pub struct ControlsView {
    pub key_sequence: Vec<String>,
}

pub struct Control {
    app: re_viewer::App,
    states: ControlStates,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Annotation {
    id: usize,
    frame_idx: u64,
    text: String,
}

#[derive(Serialize, Deserialize)]
struct AnnotationCollections {
    annotations: Vec<Annotation>,
    protocols: Vec<Annotation>,
    errors: Vec<Annotation>,
}

#[derive(Deserialize)]
struct ProtocolFile {
    protocol: Vec<String>,
    annotations: Option<Vec<Annotation>>,
}

impl eframe::App for Control {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        self.app.save(storage);
    }

    fn update(&mut self, ctx: &Context, frame: &mut eframe::Frame) {
        egui::SidePanel::right("Control Panel")
            .max_width(600.0)
            .resizable(false)
            .show(ctx, |ui| {
                ScrollArea::vertical().show(ui, |ui| {
                    self.ui(ui, ctx);
                });
            });

        self.app.update(ctx, frame);
    }
}

impl Control {
    pub fn new(app: re_viewer::App) -> Self {
        let mut control = Control {
            app,
            states: ControlStates {
                entity_path: "foo".to_string(),
                dynamic_radius: 0.1,
                next_annotation_id: 1,
                ..Default::default()
            },
        };

        let protocol_file_path =
            "/Users/q-weave/Weave/rerun/examples/rust/custom_callback/protocols/stage1.json";

        if let Err(e) = control.read_protocols_from_file(protocol_file_path) {
            rerun::external::re_log::error!(
                "Failed to read protocol file '{}': {}",
                protocol_file_path,
                e
            );
        }
        control
    }

    fn get_current_frame_index(&self) -> u64 {
        if let Some(frame_i64) = self.app.current_time_int() {
            frame_i64 as u64
        } else {
            0
        }
    }

    fn go_to_frame(&mut self, frame_idx: i64) {
        self.app.set_current_time_int(frame_idx);
    }

    fn remove_annotation_by_id(&mut self, id: usize) {
        if let Some(pos) = self.states.annotations.iter().position(|a| a.id == id) {
            self.states.annotations.remove(pos);
            rerun::external::re_log::info!("Removed annotation with ID {}", id);
        } else {
            rerun::external::re_log::warn!("Annotation with ID {} not found.", id);
        }
    }

    fn remove_error_by_id(&mut self, id: usize) {
        if let Some(pos) = self.states.errors.iter().position(|a| a.id == id) {
            self.states.errors.remove(pos);
            rerun::external::re_log::info!("Removed error with ID {}", id);
        } else {
            rerun::external::re_log::warn!("Error with ID {} not found.", id);
        }
    }

    fn remove_protocol_ann_by_id(&mut self, id: usize) {
        if let Some(pos) = self.states.protocol_anns.iter().position(|a| a.id == id) {
            self.states.protocol_anns.remove(pos);
            rerun::external::re_log::info!("Removed error with ID {}", id);
        } else {
            rerun::external::re_log::warn!("Error with ID {} not found.", id);
        }
    }

    /// The JSON file should have the following structure:
    /// {
    ///     "protocol": [
    ///         "Protocol text for key 1",
    ///         "Protocol text for key 2",
    ///         ...
    ///     ]
    /// }
    fn read_protocols_from_file(&mut self, file_path: &str) -> std::io::Result<()> {
        let path = Path::new(file_path);

        if !path.exists() {
            rerun::external::re_log::warn!(
                "Protocol file '{}' does not exist. No protocols loaded.",
                file_path
            );
            return Ok(());
        }

        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let protocol_file: ProtocolFile = serde_json::from_reader(reader)?;

        rerun::external::re_log::info!("Deserialized protocol_file: {:?}", protocol_file.protocol);

        for (i, text) in protocol_file.protocol.iter().enumerate() {
            let key = (i + 1) as u8;

            if key >= 1 && key <= 9 {
                if text.trim().is_empty() {
                    rerun::external::re_log::warn!(
                        "Protocol text for key '{}' is empty and will be ignored.",
                        key
                    );
                    continue;
                }

                self.states.protocols.push(Annotation {
                    id: 0,
                    frame_idx: 0,
                    text: text.clone(),
                });
            } else {
                rerun::external::re_log::warn!(
                    "Protocol key '{}' is out of valid range (1-9) and will be ignored.",
                    key
                );
            }
        }

        // Log loaded protocols
        for protocol in &self.states.protocols {
            rerun::external::re_log::info!(
                "Loaded Protocol - ID: {}, Text: '{}', Frame Index: {}",
                protocol.id,
                protocol.text,
                protocol.frame_idx
            );
        }

        rerun::external::re_log::info!(
            "Loaded {} protocols from '{}'.",
            self.states.protocols.len(),
            file_path
        );

        if let Some(annotations) = protocol_file.annotations {
            for annotation in annotations {
                if annotation.text.trim().is_empty() {
                    rerun::external::re_log::warn!(
                        "Annotation with ID '{}' has empty text and will be ignored.",
                        annotation.id
                    );
                    continue;
                }

                self.states.annotations.push(annotation.clone());

                rerun::external::re_log::info!(
                    "Loaded Annotation - ID: {}, Frame Index: {}, Text: '{}'",
                    annotation.id,
                    annotation.frame_idx,
                    annotation.text
                );
            }

            rerun::external::re_log::info!(
                "Loaded {} annotations from '{}'.",
                self.states.annotations.len(),
                file_path
            );
        } else {
            rerun::external::re_log::info!("No annotations section found in '{}'.", file_path);
        }
        Ok(())
    }

    fn get_recording_id(&self) -> Option<String> {
        self.app
            .active_recording_id()
            .map(|store_id| (*store_id.id).clone())
    }

    fn write_annotations(&self) -> io::Result<()> {
        if let Some(recording_id) = self.get_recording_id() {
            write_annotations_to_file(
                &recording_id,
                &self.states.annotations,
                &self.states.protocol_anns,
                &self.states.errors,
            )
        } else {
            Err(io::Error::new(
                io::ErrorKind::Other,
                "Recording ID not available",
            ))
        }
    }

    fn handle_keybindings(&mut self, ctx: &Context) {
        for num in 1..=9 {
            // let key_str = num.to_string(); // Already removed

            let key = match num {
                1 => Key::Num1,
                2 => Key::Num2,
                3 => Key::Num3,
                4 => Key::Num4,
                5 => Key::Num5,
                6 => Key::Num6,
                7 => Key::Num7,
                8 => Key::Num8,
                9 => Key::Num9,
                _ => continue,
            };

            if ctx.input(|i| i.key_pressed(key)) {
                let protocol_index = (num - 1) as usize;
                if protocol_index < self.states.protocols.len() {
                    let protocol = &self.states.protocols[protocol_index];
                    let frame_idx = self.get_current_frame_index();
                    let new_id = self.states.next_annotation_id;
                    self.states.next_annotation_id += 1;

                    let formatted_text: String =
                        format!("[{}] {}: {}", num, protocol.text, frame_idx);

                    rerun::external::re_log::info!(
                        "Added annotation via key {}: {}",
                        num,
                        &formatted_text
                    );

                    self.states.protocol_anns.insert(
                        0,
                        Annotation {
                            id: new_id,
                            frame_idx,
                            text: protocol.text.clone(),
                        },
                    );
                } else {
                    rerun::external::re_log::warn!("No protocol assigned to key {}", num);
                }
            }
        }
        if ctx.input(|i| i.key_pressed(Key::E) && i.modifiers.ctrl) {
            self.add_error();
        }
        if ctx.input(|i| i.key_pressed(Key::R) && i.modifiers.ctrl) {
            self.add_recover();
        }
        if ctx.input(|i| i.key_pressed(Key::F) && i.modifiers.ctrl) {
            self.add_fixed();
        }
    }

    fn add_error(&mut self) {
        let text = self.states.error_text.trim().to_string();
        let frame_index = self.get_current_frame_index();
        let new_id = self.states.next_annotation_id;
        self.states.next_annotation_id += 1;

        let annotation_text = if text.is_empty() {
            "Error Made".to_string()
        } else {
            format!("Error Made: {}", text)
        };

        self.states.errors.insert(
            0,
            Annotation {
                id: new_id,
                frame_idx: frame_index,
                text: annotation_text,
            },
        );

        if !text.is_empty() {
            self.states.error_text.clear();
        }
    }

    fn add_recover(&mut self) {
        let text = self.states.error_text.trim().to_string();
        let frame_index = self.get_current_frame_index();
        let new_id = self.states.next_annotation_id;
        self.states.next_annotation_id += 1;

        let annotation_text = if text.is_empty() {
            "Start Recovery".to_string()
        } else {
            format!("Start Recovery: {}", text)
        };

        self.states.errors.insert(
            0,
            Annotation {
                id: new_id,
                frame_idx: frame_index,
                text: annotation_text,
            },
        );

        if !text.is_empty() {
            self.states.error_text.clear();
        }
    }

    fn add_fixed(&mut self) {
        let text = self.states.error_text.trim().to_string();
        let frame_index = self.get_current_frame_index();
        let new_id = self.states.next_annotation_id;
        self.states.next_annotation_id += 1;

        let annotation_text = if text.is_empty() {
            "End Recovery".to_string()
        } else {
            format!("End Recovery: {}", text)
        };

        self.states.errors.insert(
            0,
            Annotation {
                id: new_id,
                frame_idx: frame_index,
                text: annotation_text,
            },
        );

        if !text.is_empty() {
            self.states.error_text.clear();
        }
    }

    fn ui(&mut self, ui: &mut Ui, ctx: &Context) {
        ui.spacing_mut().item_spacing.y = 9.0;

        ui.vertical_centered(|ui| {
            ui.strong("Control panel");
        });

        self.handle_keybindings(ctx);

        list_item::list_item_scope(ui, "protocol", |ui| {
            ui.section_collapsing_header("Protocol")
                .default_open(true)
                .show(ui, |ui| {
                    let protocol_anns = self.states.protocol_anns.clone();
                    let protocols = self.states.protocols.clone();

                    if protocol_anns.is_empty() && protocols.is_empty() {
                        ui.label("No protocols loaded.");
                    } else {
                        if !protocols.is_empty() {
                            ui.label(
                                "Press the corresponding number key to add a protocol annotation.",
                            );
                            ui.separator();
                            for (i, protocol) in protocols.iter().enumerate() {
                                let protocol_number = i + 1;
                                let protocol_text = &protocol.text;

                                ui.horizontal(|ui| {
                                    ui.add(
                                        Label::new(format!(
                                            "[{}]: {}",
                                            protocol_number, protocol_text
                                        ))
                                        .wrap(),
                                    );
                                });
                            }
                        }

                        if !protocol_anns.is_empty() {
                            ui.separator();
                            ui.label("Protocol Annotations:");
                            for annotation in protocol_anns {
                                ui.horizontal(|ui| {
                                    ui.add_sized(
                                        [500.0, 0.0],
                                        Label::new(format!(
                                            "{}: {}",
                                            annotation.frame_idx, annotation.text
                                        ))
                                        .wrap(),
                                    );

                                    if ui.button("Go").clicked() {
                                        self.go_to_frame(annotation.frame_idx as i64);
                                    }

                                    if ui.button("Rem").clicked() {
                                        self.remove_protocol_ann_by_id(annotation.id);
                                    }
                                });
                            }
                        }
                    }
                });
        });

        list_item::list_item_scope(ui, "annotations", |ui| {
            ui.section_collapsing_header("Annotations")
                .default_open(true)
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.label("Annotation text:");
                        ui.text_edit_singleline(&mut self.states.annotation_text);
                    });

                    ui.horizontal(|ui| {
                        if ui.button("Enter").clicked() {
                            let text = self.states.annotation_text.trim().to_string();
                            if !text.is_empty() {
                                let frame_index = self.get_current_frame_index();
                                let new_id = self.states.next_annotation_id;
                                self.states.next_annotation_id += 1;
                                self.states.annotations.insert(
                                    0,
                                    Annotation {
                                        id: new_id,
                                        frame_idx: frame_index,
                                        text,
                                    },
                                );
                                self.states.annotation_text.clear();
                            }
                        }

                        if ui.button("Submit").clicked() {
                            match self.write_annotations() {
                                Ok(_) => {
                                    rerun::external::re_log::info!(
                                        "Annotations successfully saved."
                                    );
                                    ui.colored_label(
                                        Color32::GREEN,
                                        "Annotations saved successfully.",
                                    );
                                }
                                Err(err) => {
                                    rerun::external::re_log::error!(
                                        "Failed to write annotations: {err}"
                                    );
                                    ui.colored_label(
                                        Color32::RED,
                                        format!("Error saving annotations: {}", err),
                                    );
                                }
                            }
                        }
                    });

                    ui.separator();

                    let annotations = self.states.annotations.clone();

                    for annotation in annotations {
                        ui.horizontal(|ui| {
                            ui.add_sized(
                                [500.0, 0.0],
                                Label::new(format!(
                                    "{}: {}",
                                    annotation.frame_idx, annotation.text
                                ))
                                .wrap(),
                            );

                            if ui.button("Go").clicked() {
                                self.go_to_frame(annotation.frame_idx as i64);
                            }

                            if ui.button("Rem").clicked() {
                                self.remove_annotation_by_id(annotation.id);
                            }
                        });
                    }
                });
        });

        list_item::list_item_scope(ui, "errors", |ui| {
            ui.section_collapsing_header("Errors")
                .default_open(true)
                .show(ui, |ui| {
                    ui.label(
                        "Control + <key> logs the error type. Text is optional but encouraged!",
                    );
                    ui.separator();
                    ui.horizontal(|ui| {
                        ui.label("Error text:");
                        ui.text_edit_singleline(&mut self.states.error_text);
                    });

                    ui.horizontal(|ui| {
                        if ui.button("[E] error").clicked() {
                            self.add_error();
                        }
                        if ui.button("[R] recover").clicked() {
                            self.add_recover();
                        }
                        if ui.button("[F] fixed").clicked() {
                            self.add_fixed();
                        }
                    });

                    ui.separator();

                    let errors = self.states.errors.clone();

                    for error in errors {
                        ui.horizontal(|ui| {
                            ui.add_sized(
                                [500.0, 0.0],
                                Label::new(format!("{}: {}", error.frame_idx, error.text)).wrap(),
                            );

                            if ui.button("Go").clicked() {
                                self.go_to_frame(error.frame_idx as i64);
                            }

                            if ui.button("Rem").clicked() {
                                self.remove_error_by_id(error.id);
                            }
                        });
                    }
                });
        });
    }
}

fn write_annotations_to_file(
    recording_id: &str,
    annotations: &[Annotation],
    protocols: &[Annotation],
    errors: &[Annotation],
) -> std::io::Result<()> {
    let file_name = format!("{}.json", recording_id);
    let file = File::create(&file_name)?;
    let writer = BufWriter::new(file);

    let consolidated = AnnotationCollections {
        annotations: annotations.iter().cloned().collect(),
        protocols: protocols.iter().cloned().collect(),
        errors: errors.iter().cloned().collect(),
    };

    to_writer_pretty(writer, &consolidated)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
}
