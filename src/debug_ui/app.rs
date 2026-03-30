use std::sync::mpsc::{Receiver, TryRecvError};
use std::time::Duration;

use super::player_panel::{self, PLAYER_PANEL_PREFIX};
use super::runtime_panel::{self, MONSTER_PANEL_PREFIX, NPC_PANEL_PREFIX};
use super::store::{DebugPanel, DebugPanelStore};
use super::{clear_sender, DebugUiMessage};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DebugView {
    Players,
    Npcs,
    Monsters,
}

impl DebugView {
    fn title(self) -> &'static str {
        match self {
            Self::Players => "Players",
            Self::Npcs => "NPCs",
            Self::Monsters => "Monsters",
        }
    }

    fn panel_prefix(self) -> &'static str {
        match self {
            Self::Players => PLAYER_PANEL_PREFIX,
            Self::Npcs => NPC_PANEL_PREFIX,
            Self::Monsters => MONSTER_PANEL_PREFIX,
        }
    }

    fn subtitle(self) -> &'static str {
        match self {
            Self::Players => "Visualizacao em tempo real dos jogadores conectados.",
            Self::Npcs => "Espaco reservado para monitoramento de NPCs em runtime.",
            Self::Monsters => "Espaco reservado para monitoramento de monsters em runtime.",
        }
    }
}

pub fn run_window(rx: Receiver<DebugUiMessage>) {
    let mut native_options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 820.0])
            .with_min_inner_size([980.0, 640.0])
            .with_title("Debug UI"),
        ..Default::default()
    };

    #[cfg(target_os = "windows")]
    {
        native_options.event_loop_builder = Some(Box::new(|builder| {
            use winit::platform::windows::EventLoopBuilderExtWindows;
            builder.with_any_thread(true);
        }));
    }

    let app = DebugUiApp::new(rx);
    let result = eframe::run_native("Debug UI", native_options, Box::new(|_cc| Ok(Box::new(app))));

    if let Err(err) = result {
        log::warn!("debug UI terminated with error: {:?}", err);
        clear_sender();
    }
}

struct DebugUiApp {
    rx: Receiver<DebugUiMessage>,
    store: DebugPanelStore,
    current_view: DebugView,
    selected_player_panel_id: Option<String>,
    selected_npc_panel_id: Option<String>,
    selected_monster_panel_id: Option<String>,
}

impl DebugUiApp {
    fn new(rx: Receiver<DebugUiMessage>) -> Self {
        Self {
            rx,
            store: DebugPanelStore::default(),
            current_view: DebugView::Players,
            selected_player_panel_id: None,
            selected_npc_panel_id: None,
            selected_monster_panel_id: None,
        }
    }

    fn selected_panel_id_mut(&mut self, view: DebugView) -> &mut Option<String> {
        match view {
            DebugView::Players => &mut self.selected_player_panel_id,
            DebugView::Npcs => &mut self.selected_npc_panel_id,
            DebugView::Monsters => &mut self.selected_monster_panel_id,
        }
    }

    fn selected_panel_id(&self, view: DebugView) -> Option<&str> {
        match view {
            DebugView::Players => self.selected_player_panel_id.as_deref(),
            DebugView::Npcs => self.selected_npc_panel_id.as_deref(),
            DebugView::Monsters => self.selected_monster_panel_id.as_deref(),
        }
    }

    fn ensure_valid_selection_for(&mut self, view: DebugView) {
        let prefix = view.panel_prefix();
        let is_valid = self
            .selected_panel_id(view)
            .map(|selected| self.store.contains(selected))
            .unwrap_or(false);

        if !is_valid {
            *self.selected_panel_id_mut(view) = self.store.first_panel_id_by_prefix(prefix);
        }
    }

    fn drain_updates(&mut self) {
        loop {
            match self.rx.try_recv() {
                Ok(DebugUiMessage::Upsert(panel)) => {
                    let panel_id = panel.panel_id.clone();
                    let prefix = panel_prefix_for_id(&panel_id);
                    let had_no_panels = self.store.is_empty_by_prefix(prefix);

                    self.store.upsert(panel);

                    let selected = self.selected_panel_id_mut(view_for_prefix(prefix));
                    if had_no_panels || selected.is_none() {
                        *selected = Some(panel_id);
                    }
                }
                Ok(DebugUiMessage::Remove(panel_id)) => {
                    let prefix = panel_prefix_for_id(&panel_id);
                    let view = view_for_prefix(prefix);
                    let removed_selected = self.selected_panel_id(view) == Some(panel_id.as_str());

                    self.store.remove(&panel_id);

                    if removed_selected {
                        *self.selected_panel_id_mut(view) =
                            self.store.first_panel_id_by_prefix(prefix);
                    }
                }
                Err(TryRecvError::Empty) => break,
                Err(TryRecvError::Disconnected) => break,
            }
        }

        self.ensure_valid_selection_for(DebugView::Players);
        self.ensure_valid_selection_for(DebugView::Npcs);
        self.ensure_valid_selection_for(DebugView::Monsters);
    }

    fn selected_panel(&self, view: DebugView) -> Option<&DebugPanel> {
        self.selected_panel_id(view)
            .and_then(|panel_id| self.store.get(panel_id))
    }

    fn render_view_selector(&mut self, ui: &mut eframe::egui::Ui) {
        ui.horizontal(|ui| {
            for view in [DebugView::Players, DebugView::Npcs, DebugView::Monsters] {
                let selected = self.current_view == view;
                if ui.selectable_label(selected, view.title()).clicked() {
                    self.current_view = view;
                }
            }
        });
    }

    fn render_sidebar(&mut self, ui: &mut eframe::egui::Ui) {
        let prefix = self.current_view.panel_prefix();
        let panels: Vec<(String, String, String)> = self
            .store
            .ordered_panels_by_prefix(prefix)
            .into_iter()
            .map(|panel| {
                (
                    panel.panel_id.clone(),
                    panel.title.clone(),
                    panel.updated_at.clone(),
                )
            })
            .collect();

        ui.heading(self.current_view.title());
        ui.label(format!("Total: {}", panels.len()));
        ui.separator();

        if panels.is_empty() {
            ui.label("Nenhum painel disponivel.");
            return;
        }

        eframe::egui::ScrollArea::vertical()
            .id_salt(format!("side_list:{}", self.current_view.title()))
            .show(ui, |ui| {
                for (panel_id, title, updated_at) in panels {
                    let selected =
                        self.selected_panel_id(self.current_view) == Some(panel_id.as_str());

                    eframe::egui::Frame::group(ui.style()).show(ui, |ui| {
                        let response =
                            ui.selectable_label(selected, format!("{title}  ({updated_at})"));
                        if response.clicked() {
                            *self.selected_panel_id_mut(self.current_view) = Some(panel_id.clone());
                        }

                        ui.small(panel_id);
                    });

                    ui.add_space(6.0);
                }
            });
    }

    fn render_current_view(&self, ui: &mut eframe::egui::Ui) {
        match self.current_view {
            DebugView::Players => {
                if let Some(panel) = self.selected_panel(DebugView::Players).cloned() {
                    eframe::egui::ScrollArea::vertical()
                        .id_salt(format!("selected_player_scroll:{}", panel.panel_id))
                        .show(ui, |ui| {
                            player_panel::render_player_content(ui, &panel);
                        });
                } else {
                    ui.add_space(16.0);
                    ui.heading("Nenhum jogador disponivel");
                    ui.label("Assim que um painel for publicado, ele aparecera aqui.");
                }
            }
            DebugView::Npcs => runtime_panel::render_placeholder(
                ui,
                "NPCs",
                DebugView::Npcs.subtitle(),
                self.selected_panel(DebugView::Npcs),
            ),
            DebugView::Monsters => runtime_panel::render_placeholder(
                ui,
                "Monsters",
                DebugView::Monsters.subtitle(),
                self.selected_panel(DebugView::Monsters),
            ),
        }
    }
}

impl eframe::App for DebugUiApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        self.drain_updates();
        ctx.request_repaint_after(Duration::from_millis(100));

        eframe::egui::TopBottomPanel::top("debug_ui_top_panel")
            .resizable(false)
            .show(ctx, |ui| {
                ui.heading("Debug UI");
                self.render_view_selector(ui);
                ui.label(self.current_view.subtitle());
            });

        eframe::egui::SidePanel::left("debug_ui_side_panel")
            .resizable(true)
            .default_width(280.0)
            .min_width(220.0)
            .show(ctx, |ui| {
                self.render_sidebar(ui);
            });

        eframe::egui::CentralPanel::default().show(ctx, |ui| {
            self.render_current_view(ui);
        });
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        clear_sender();
    }
}

fn panel_prefix_for_id(panel_id: &str) -> &'static str {
    if panel_id.starts_with(NPC_PANEL_PREFIX) {
        NPC_PANEL_PREFIX
    } else if panel_id.starts_with(MONSTER_PANEL_PREFIX) {
        MONSTER_PANEL_PREFIX
    } else {
        PLAYER_PANEL_PREFIX
    }
}

fn view_for_prefix(prefix: &str) -> DebugView {
    match prefix {
        NPC_PANEL_PREFIX => DebugView::Npcs,
        MONSTER_PANEL_PREFIX => DebugView::Monsters,
        _ => DebugView::Players,
    }
}
