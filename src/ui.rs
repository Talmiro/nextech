use std::sync::{Arc, Mutex};

use eframe::egui;
use egui::{Color32, Margin, Rounding, Vec2};
use egui_extras::Column;

use crate::App;
use crate::db::{Database, ItemSalesSummary};
use crate::models;

// ─────────────────────────────────────────────────────────────
// NexTech Design System — zinc dark refinado
// Base 4pt: 4 / 8 / 12 / 16 / 24 / 32
// Superfícies: BG #121214, CARD #18181B, INPUT #1F1F24, BORDER #27272A
// Texto: TITLE #FAFAF9, BODY #D4D4D8, MUTED #A1A1AA
// Semântica: emerald #10B981, blue #60A5FA / #2563EB, amber #F59E0B, coral #F87171
// ─────────────────────────────────────────────────────────────

mod theme {
    use super::*;

    pub const BG: Color32 = Color32::from_rgb(18, 18, 20);
    pub const CARD: Color32 = Color32::from_rgb(24, 24, 27);
    pub const INPUT_BG: Color32 = Color32::from_rgb(31, 31, 36);
    pub const HOVER_BG: Color32 = Color32::from_rgb(34, 34, 39);
    pub const BORDER: Color32 = Color32::from_rgb(39, 39, 42);
    pub const BORDER_STRONG: Color32 = Color32::from_rgb(63, 63, 70);

    pub const TITLE: Color32 = Color32::from_rgb(250, 250, 249);
    pub const BODY: Color32 = Color32::from_rgb(212, 212, 216);
    pub const MUTED: Color32 = Color32::from_rgb(161, 161, 170);
    pub const FAINT: Color32 = Color32::from_rgb(113, 113, 122);

    pub const EMERALD: Color32 = Color32::from_rgb(16, 185, 129);
    pub const EMERALD_DEEP: Color32 = Color32::from_rgb(6, 78, 59);
    pub const EMERALD_SOFT_BG: Color32 = Color32::from_rgb(20, 46, 37);

    pub const BLUE: Color32 = Color32::from_rgb(96, 165, 250);
    pub const BLUE_DEEP: Color32 = Color32::from_rgb(37, 99, 235);
    pub const BLUE_DIM_BG: Color32 = Color32::from_rgb(30, 41, 59);

    pub const AMBER: Color32 = Color32::from_rgb(245, 158, 11);
    pub const AMBER_DIM_BG: Color32 = Color32::from_rgb(46, 32, 12);

    pub const CORAL: Color32 = Color32::from_rgb(248, 113, 113);
    pub const CORAL_DIM_BG: Color32 = Color32::from_rgb(52, 24, 26);

    pub const CARD_ROUNDING: f32 = 12.0;
    pub const INPUT_ROUNDING: f32 = 8.0;

    pub fn card_margin() -> Margin {
        Margin {
            left: 16.0,
            right: 16.0,
            top: 12.0,
            bottom: 12.0,
        }
    }

    pub fn form_margin() -> Margin {
        Margin {
            left: 16.0,
            right: 16.0,
            top: 16.0,
            bottom: 16.0,
        }
    }

    pub fn apply(ctx: &egui::Context) {
        let mut style = (*ctx.style()).clone();
        style.visuals.dark_mode = true;
        style.visuals.override_text_color = Some(BODY);
        style.visuals.window_fill = BG;
        style.visuals.panel_fill = BG;
        style.visuals.faint_bg_color = CARD;
        style.visuals.extreme_bg_color = INPUT_BG;
        style.visuals.code_bg_color = INPUT_BG;
        style.visuals.warn_fg_color = AMBER;
        style.visuals.error_fg_color = CORAL;
        style.visuals.hyperlink_color = BLUE;
        style.visuals.window_rounding = Rounding::same(CARD_ROUNDING);
        style.visuals.window_stroke = egui::Stroke::new(1.0, BORDER);
        style.visuals.menu_rounding = Rounding::same(8.0);
        style.visuals.button_frame = true;
        style.visuals.collapsing_header_frame = false;
        style.visuals.striped = true;
        style.visuals.clip_rect_margin = 4.0;

        style.visuals.widgets.noninteractive.bg_fill = CARD;
        style.visuals.widgets.noninteractive.weak_bg_fill = CARD;
        style.visuals.widgets.noninteractive.bg_stroke = egui::Stroke::new(1.0, BORDER);
        style.visuals.widgets.noninteractive.rounding = Rounding::same(8.0);
        style.visuals.widgets.noninteractive.fg_stroke = egui::Stroke::new(1.0, BODY);
        style.visuals.widgets.noninteractive.expansion = 0.0;

        style.visuals.widgets.inactive.bg_fill = INPUT_BG;
        style.visuals.widgets.inactive.weak_bg_fill = INPUT_BG;
        style.visuals.widgets.inactive.bg_stroke = egui::Stroke::new(1.0, BORDER);
        style.visuals.widgets.inactive.rounding = Rounding::same(INPUT_ROUNDING);
        style.visuals.widgets.inactive.fg_stroke = egui::Stroke::new(1.0, BODY);
        style.visuals.widgets.inactive.expansion = 0.0;

        style.visuals.widgets.hovered.bg_fill = HOVER_BG;
        style.visuals.widgets.hovered.weak_bg_fill = HOVER_BG;
        style.visuals.widgets.hovered.bg_stroke = egui::Stroke::new(1.0, BORDER_STRONG);
        style.visuals.widgets.hovered.rounding = Rounding::same(INPUT_ROUNDING);
        style.visuals.widgets.hovered.fg_stroke = egui::Stroke::new(1.0, TITLE);
        style.visuals.widgets.hovered.expansion = 0.0;

        style.visuals.widgets.active.bg_fill = BLUE_DIM_BG;
        style.visuals.widgets.active.weak_bg_fill = BLUE_DIM_BG;
        style.visuals.widgets.active.bg_stroke = egui::Stroke::new(1.0, BLUE);
        style.visuals.widgets.active.rounding = Rounding::same(INPUT_ROUNDING);
        style.visuals.widgets.active.fg_stroke = egui::Stroke::new(1.0, TITLE);
        style.visuals.widgets.active.expansion = 0.0;

        style.visuals.widgets.open.bg_fill = BLUE_DIM_BG;
        style.visuals.widgets.open.weak_bg_fill = BLUE_DIM_BG;
        style.visuals.widgets.open.bg_stroke = egui::Stroke::new(1.0, BLUE);
        style.visuals.widgets.open.rounding = Rounding::same(INPUT_ROUNDING);
        style.visuals.widgets.open.fg_stroke = egui::Stroke::new(1.0, TITLE);
        style.visuals.widgets.open.expansion = 0.0;

        style.visuals.selection.bg_fill = Color32::from_rgba_unmultiplied(37, 99, 235, 70);
        style.visuals.selection.stroke = egui::Stroke::new(1.0, BLUE);

        style.spacing.item_spacing = Vec2::new(12.0, 8.0);
        style.spacing.button_padding = Vec2::new(16.0, 8.0);
        style.spacing.window_margin = Margin::same(16.0);
        style.spacing.menu_margin = Margin::same(12.0);
        style.spacing.indent = 16.0;
        style.spacing.interact_size = Vec2::new(44.0, 28.0);
        style.spacing.text_edit_width = 280.0;
        style.spacing.combo_width = 180.0;

        ctx.set_style(style);
    }

    pub fn card_frame() -> egui::Frame {
        egui::Frame::none()
            .fill(CARD)
            .stroke(egui::Stroke::new(1.0, BORDER))
            .rounding(Rounding::same(CARD_ROUNDING))
            .inner_margin(card_margin())
    }

    pub fn form_frame() -> egui::Frame {
        egui::Frame::none()
            .fill(CARD)
            .stroke(egui::Stroke::new(1.0, BORDER))
            .rounding(Rounding::same(CARD_ROUNDING))
            .inner_margin(form_margin())
    }
}

use theme as ds;

pub enum AppRoute {
    Dashboard,
    POS,
    CashRegister,
    Fiado,
    Reports,
}

impl Clone for AppRoute {
    fn clone(&self) -> Self {
        match self {
            AppRoute::Dashboard => AppRoute::Dashboard,
            AppRoute::POS => AppRoute::POS,
            AppRoute::CashRegister => AppRoute::CashRegister,
            AppRoute::Fiado => AppRoute::Fiado,
            AppRoute::Reports => AppRoute::Reports,
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum DashboardPeriod {
    #[default]
    Diario,
    Mensal,
    Anual,
}

impl DashboardPeriod {
    pub fn label(&self) -> &'static str {
        match self {
            DashboardPeriod::Diario => "Diário",
            DashboardPeriod::Mensal => "Mensal",
            DashboardPeriod::Anual => "Anual",
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ReportExportScope {
    Diario,
    #[default]
    Mensal,
    Anual,
    Geral,
}

impl ReportExportScope {
    pub fn label(&self) -> &'static str {
        match self {
            ReportExportScope::Diario => "Dia",
            ReportExportScope::Mensal => "Mês",
            ReportExportScope::Anual => "Ano",
            ReportExportScope::Geral => "Geral",
        }
    }
}

/// Sub-abas da rota Relatórios. `Financeiro` preserva o consolidado
/// existente; `Reposicao` agrega itens por modelo (não-destrutiva).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ReportSubTab {
    #[default]
    Financeiro,
    Reposicao,
}

impl ReportSubTab {
    pub fn label(&self) -> &'static str {
        match self {
            ReportSubTab::Financeiro => "📈 Financeiro",
            ReportSubTab::Reposicao => "📦 Reposição de Estoque",
        }
    }
}

const SIDEBAR_WIDTH: f32 = 232.0;
// Breakpoint único da casca (TopBar/SidePanel/Central): mesma régua nos dois ramos.
const SIDEBAR_BREAKPOINT: f32 = 800.0;

// ── Pequenos construtores de componentes ──────────────────────

fn styled_singleline<'a>(text: &'a mut String, hint: &'a str) -> egui::TextEdit<'a> {
    egui::TextEdit::singleline(text)
        .font(egui::FontId::proportional(14.0))
        .hint_text(hint)
        .margin(Margin::symmetric(12.0, 8.0))
}

fn primary_button(label: &str) -> egui::Button<'_> {
    egui::Button::new(
        egui::RichText::new(label)
            .size(14.0)
            .strong()
            .color(ds::TITLE),
    )
    .fill(ds::BLUE_DEEP)
    .stroke(egui::Stroke::new(1.0, ds::BLUE))
    .rounding(Rounding::same(8.0))
}

fn success_button(label: &str) -> egui::Button<'_> {
    egui::Button::new(
        egui::RichText::new(label)
            .size(14.0)
            .strong()
            .color(ds::TITLE),
    )
    .fill(ds::EMERALD_DEEP)
    .stroke(egui::Stroke::new(1.0, ds::EMERALD))
    .rounding(Rounding::same(8.0))
}

fn ghost_button(label: &str, size: f32) -> egui::Button<'_> {
    egui::Button::new(egui::RichText::new(label).size(size).color(ds::BODY))
        .fill(Color32::TRANSPARENT)
        .stroke(egui::Stroke::new(1.0, ds::BORDER))
        .rounding(Rounding::same(8.0))
}

/// Rótulo com quebra automática: evita clipping de textos PT-BR longos
/// em qualquer largura disponível.
fn wrap_label(ui: &mut egui::Ui, text: &str, size: f32, color: Color32, strong: bool) {
    let rich = if strong {
        egui::RichText::new(text).size(size).strong().color(color)
    } else {
        egui::RichText::new(text).size(size).color(color)
    };
    ui.add(egui::Label::new(rich).wrap());
}

fn error_banner(ui: &mut egui::Ui, msg: &str) {
    ui.add_space(8.0);
    egui::Frame::none()
        .fill(ds::CORAL_DIM_BG)
        .stroke(egui::Stroke::new(1.0, ds::CORAL))
        .rounding(Rounding::same(8.0))
        .inner_margin(Margin::symmetric(12.0, 8.0))
        .show(ui, |ui| {
            // `wrap()` garante que mensagens longas quebrem em vez de vazar do alerta.
            ui.add(
                egui::Label::new(
                    egui::RichText::new(msg)
                        .size(13.0)
                        .strong()
                        .color(ds::CORAL),
                )
                .wrap(),
            );
        });
    ui.add_space(8.0);
}

fn success_banner(ui: &mut egui::Ui, msg: &str) {
    ui.add_space(8.0);
    egui::Frame::none()
        .fill(ds::EMERALD_SOFT_BG)
        .stroke(egui::Stroke::new(1.0, ds::EMERALD))
        .rounding(Rounding::same(8.0))
        .inner_margin(Margin::symmetric(12.0, 8.0))
        .show(ui, |ui| {
            ui.add(
                egui::Label::new(
                    egui::RichText::new(msg)
                        .size(13.0)
                        .strong()
                        .color(ds::EMERALD),
                )
                .wrap(),
            );
        });
    ui.add_space(8.0);
}

fn warning_line(ui: &mut egui::Ui, msg: String, color: Color32) {
    ui.add(egui::Label::new(egui::RichText::new(msg).size(12.0).color(color)).wrap());
    ui.add_space(4.0);
}

fn empty_state(ui: &mut egui::Ui, title: &str, hint: &str) {
    ds::card_frame().show(ui, |ui| {
        ui.vertical_centered(|ui| {
            ui.add_space(8.0);
            ui.add(
                egui::Label::new(
                    egui::RichText::new(title)
                        .size(14.0)
                        .strong()
                        .color(ds::TITLE),
                )
                .wrap(),
            );
            ui.add_space(4.0);
            ui.add(egui::Label::new(egui::RichText::new(hint).size(12.0).color(ds::MUTED)).wrap());
            ui.add_space(8.0);
        });
    });
}

/// Versão plana do estado vazio, para uso DENTRO de outro card
/// (evita card-in-card). Sem `Frame`, só tipografia + espaçamento.
fn empty_state_flat(ui: &mut egui::Ui, title: &str, hint: &str) {
    ui.vertical_centered(|ui| {
        ui.add_space(8.0);
        ui.add(
            egui::Label::new(
                egui::RichText::new(title)
                    .size(14.0)
                    .strong()
                    .color(ds::TITLE),
            )
            .wrap(),
        );
        ui.add_space(4.0);
        ui.add(egui::Label::new(egui::RichText::new(hint).size(12.0).color(ds::MUTED)).wrap());
        ui.add_space(8.0);
    });
}

// ── Layout responsivo (compensa SidePanel dentro do Central) ───
// Usa available_width real no momento da chamada; breakpoints por conteúdo.
const BP_WIDE: f32 = 860.0;
const BP_MEDIUM: f32 = 640.0;
// Respiro da grade KPI (grade 4pt) e teto anti-esticamento em ultrawide.
const KPI_GAP: f32 = 16.0;
const KPI_GRID_MAX_W: f32 = 1100.0;
// Teto de leitura do conteúdo das 5 telas (igual ao KPI): em monitores
// largos o conteúdo para de esticar e alinha à esquerda — padrão zero-risco
// dentro de ScrollArea (sem realimentação de medição).
const CONTENT_MAX_W: f32 = 1100.0;
// Altura mínima do conteúdo do card KPI (≈ 18 título + 8 + ~70 faixa de
// valor): todos os cards medem igual — fileiras uniformes, sem invasão
// vertical de uma fileira sobre a seguinte.
const KPI_CARD_MIN_H: f32 = 120.0;
// Banda morta dos ramos por container: entra no modo largo no limiar,
// só volta ao estreito `HYSTERESIS` px abaixo — sem flip no pixel exato.
const HYSTERESIS: f32 = 24.0;

/// Ramo responsivo com histerese (banda morta `HYSTERESIS`): entra no modo
/// largo no limiar e só volta ao estreito bem abaixo dele. `salt` deve ser
/// único por ponto de chamada (usa-se o `grid_id` onde houver). Elimina o
/// flip no pixel exato da fronteira, que alternava ramos inteiros — e com
/// eles o scroll e a largura medida — entre frames (jitter).
fn is_wide_hysteresis(ui: &egui::Ui, salt: impl std::hash::Hash, breakpoint: f32) -> bool {
    let id = egui::Id::new(salt);
    let prev_wide = ui
        .memory(|m| m.data.get_temp::<bool>(id))
        .unwrap_or_default();
    let avail = ui.available_width();
    let wide = if prev_wide {
        avail >= breakpoint - HYSTERESIS
    } else {
        // Sem histórico (primeiro frame): decisão direta, sem banda.
        avail >= breakpoint
    };
    ui.memory_mut(|m| m.data.insert_temp(id, wide));
    wide
}

/// Largura fluida de botão em linha: divide `avail` por `count` descontando gaps,
/// respeitando `min_w` sem jamais exceder `avail`.
fn fluid_button_width(avail: f32, count: usize, gap: f32, min_w: f32) -> f32 {
    let avail = avail.max(0.0);
    let count = count.max(1) as f32;
    let gaps = (count - 1.0) * gap;
    let w = (avail - gaps) / count;
    w.clamp(min_w.min(avail.max(1.0)), avail.max(1.0))
}

/// Tamanho fluido do valor do KPI: encolhe para caber sem truncar.
fn kpi_value_size(value: &str) -> f32 {
    let len = value.chars().count();
    if len > 14 {
        20.0
    } else if len > 10 {
        24.0
    } else {
        28.0
    }
}

/// Tenta bloquear o `Mutex` do banco sem pânico.
///
/// Usa `anyhow` + `?` na conversão do erro; quem chama exibe `error_banner`.
fn try_lock_db<'a>(
    db: &'a Arc<Mutex<Database>>,
) -> anyhow::Result<std::sync::MutexGuard<'a, Database>> {
    let guard = db
        .lock()
        .map_err(|e| anyhow::anyhow!("database lock poisoned: {e}"))?;
    Ok(guard)
}

/// Atalho para leitura: retorna `None` e registra o erro se o lock falhar.
fn lock_db<'a>(db: &'a Arc<Mutex<Database>>) -> Option<std::sync::MutexGuard<'a, Database>> {
    match try_lock_db(db) {
        Ok(guard) => Some(guard),
        Err(e) => {
            log::error!("database lock failed: {e:#}");
            None
        }
    }
}

pub fn render_app(ctx: &egui::Context, app: &mut App) {
    ds::apply(ctx);

    // Régua única da casca: mesmo breakpoint nos dois ramos (sem salto em 800px).
    let is_narrow = ctx.screen_rect().width() <= SIDEBAR_BREAKPOINT;

    egui::TopBottomPanel::top("topbar").show(ctx, |ui| {
        egui::Frame::none()
            .inner_margin(Margin::symmetric(16.0, 12.0))
            .show(ui, |ui| {
                ui.horizontal_wrapped(|ui| {
                    ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                        ui.label(
                            egui::RichText::new("⚡ NexTech")
                                .size(20.0)
                                .strong()
                                .color(ds::TITLE),
                        );
                        // Esconde o subtítulo em janelas estreitas para não comprimir a navegação.
                        if ui.available_width() >= 420.0 {
                            ui.add_space(8.0);
                            ui.add(
                                egui::Label::new(
                                    egui::RichText::new("PDV, Estoque e Controle Financeiro")
                                        .size(12.0)
                                        .color(ds::MUTED),
                                )
                                .wrap(),
                            );
                        }
                    });
                });
                // Janela estreita: navegação vira linha rolável dentro da
                // TopBar (flat, sem roubar largura do painel central).
                if is_narrow {
                    ui.add_space(8.0);
                    render_top_nav(ui, app);
                }
            });
    });

    if ctx.screen_rect().width() > SIDEBAR_BREAKPOINT {
        egui::SidePanel::left("sidebar")
            .resizable(false)
            .default_width(SIDEBAR_WIDTH)
            .max_width(SIDEBAR_WIDTH)
            .min_width(SIDEBAR_WIDTH)
            .show(ctx, |ui| {
                render_sidebar_contents(ui, app);
            });
    }

    egui::CentralPanel::default().show(ctx, |ui| {
        egui::Frame::none()
            .inner_margin(Margin::symmetric(16.0, 16.0))
            .show(ui, |ui| {
                ui.add_space(4.0);
                match &app.current_route {
                    AppRoute::Dashboard => render_dashboard(ui, app),
                    AppRoute::POS => render_pos(ui, app),
                    AppRoute::CashRegister => render_cash_register(ui, app),
                    AppRoute::Fiado => render_fiado(ui, app),
                    AppRoute::Reports => render_reports(ui, app),
                }
            });
    });
    app.toasts.show(ctx);
}

fn nav_targets() -> [(&'static str, &'static str); 5] {
    [
        ("📊", "Painel Geral"),
        ("🛒", "Vendas (PDV)"),
        ("💰", "Controle de Caixa"),
        ("📒", "Fiado / Recebíveis"),
        ("📈", "Relatórios"),
    ]
}

fn route_for_index(idx: usize) -> AppRoute {
    match idx {
        0 => AppRoute::Dashboard,
        1 => AppRoute::POS,
        2 => AppRoute::CashRegister,
        3 => AppRoute::Fiado,
        _ => AppRoute::Reports,
    }
}

fn is_route_index(current: &AppRoute, idx: usize) -> bool {
    matches!(
        (current, idx),
        (AppRoute::Dashboard, 0)
            | (AppRoute::POS, 1)
            | (AppRoute::CashRegister, 2)
            | (AppRoute::Fiado, 3)
            | (AppRoute::Reports, 4)
    )
}

fn render_top_nav(ui: &mut egui::Ui, app: &mut App) {
    // Linha de navegação rolável para janelas estreitas (vive na TopBar).
    // `id_salt` único: só existe uma instância por frame, mesmo com as
    // tabelas roláveis das rotas abaixo.
    egui::ScrollArea::horizontal()
        .id_salt("topnav_scroll")
        .auto_shrink([false, false])
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing = Vec2::new(8.0, 8.0);
                for (idx, (icon, label)) in nav_targets().iter().enumerate() {
                    let selected = is_route_index(&app.current_route, idx);
                    let mut btn = egui::Button::new(
                        egui::RichText::new(format!("{icon} {label}")).size(13.0),
                    )
                    .selected(selected)
                    .rounding(Rounding::same(8.0))
                    .min_size(Vec2::new(0.0, 40.0));
                    if selected {
                        btn = btn
                            .fill(ds::BLUE_DIM_BG)
                            .stroke(egui::Stroke::new(1.5, ds::BLUE));
                    } else {
                        btn = btn
                            .fill(ds::INPUT_BG)
                            .stroke(egui::Stroke::new(1.0, ds::BORDER));
                    }
                    if ui.add(btn).clicked() {
                        app.current_route = route_for_index(idx);
                    }
                }
            });
        });
}

fn render_sidebar_contents(ui: &mut egui::Ui, app: &mut App) {
    ui.add_space(16.0);
    ui.horizontal(|ui| {
        ui.add_space(4.0);
        ui.label(egui::RichText::new("🛠").size(28.0));
        ui.add_space(8.0);
        ui.vertical(|ui| {
            ui.label(
                egui::RichText::new("NexTech")
                    .size(20.0)
                    .strong()
                    .color(ds::TITLE),
            );
            ui.label(
                egui::RichText::new("Sistema PDV")
                    .size(12.0)
                    .color(ds::MUTED),
            );
        });
    });
    ui.add_space(12.0);
    ui.separator();
    ui.add_space(12.0);

    for (idx, (icon, label)) in nav_targets().iter().enumerate() {
        let selected = is_route_index(&app.current_route, idx);
        let mut btn =
            egui::Button::new(egui::RichText::new(format!("{icon}   {label}")).size(14.0))
                .selected(selected)
                .rounding(Rounding::same(8.0))
                .min_size(Vec2::new(ui.available_width(), 44.0));
        if selected {
            btn = btn
                .fill(ds::BLUE_DIM_BG)
                .stroke(egui::Stroke::new(1.5, ds::BLUE));
        } else {
            btn = btn
                .fill(Color32::TRANSPARENT)
                .stroke(egui::Stroke::new(1.0, Color32::TRANSPARENT));
        }
        if ui.add(btn).clicked() {
            app.current_route = route_for_index(idx);
        }
        ui.add_space(8.0);
    }
    ui.add_space(12.0);
    ui.separator();
    ui.add_space(12.0);
    ui.add(
        egui::Label::new(
            egui::RichText::new("v0.1.0  •  PDV moderno")
                .size(12.0)
                .color(ds::FAINT),
        )
        .wrap(),
    );
    ui.add_space(8.0);
}

fn fmt_money(v: f64) -> String {
    format!("R$ {v:.2}")
}

fn translate_payment(method: &models::PaymentMethod) -> &str {
    match method {
        models::PaymentMethod::Cash => "Dinheiro",
        models::PaymentMethod::Card => "Cartão",
        models::PaymentMethod::Pix => "Pix",
        models::PaymentMethod::Fiado => "Fiado",
    }
}

fn status_label(status: &models::SaleStatus) -> &'static str {
    status.label_ptbr()
}

fn status_color(status: &models::SaleStatus) -> Color32 {
    match status {
        models::SaleStatus::Quitado => ds::EMERALD,
        models::SaleStatus::Parcial => ds::AMBER,
        models::SaleStatus::Pendente => ds::CORAL,
    }
}

fn source_label(source: &models::CashSource) -> &str {
    source.short_label()
}

fn format_datetime_br(s: &str) -> String {
    let s = s.trim();
    for fmt in ["%Y-%m-%d %H:%M:%S", "%Y-%m-%d %H:%M"] {
        if let Ok(dt) = chrono::NaiveDateTime::parse_from_str(s, fmt) {
            return dt.format("%d/%m/%Y %H:%M").to_string();
        }
    }
    s.to_string()
}

fn format_month_br(ym: &str) -> String {
    let mut it = ym.splitn(2, '-');
    match (it.next(), it.next()) {
        (Some(y), Some(m)) => format!("{m}/{y}"),
        _ => ym.to_string(),
    }
}

fn search_bar(ui: &mut egui::Ui, hint: &str, query: &mut String) {
    ds::card_frame().show(ui, |ui| {
        search_bar_inner(ui, hint, query);
    });
    ui.add_space(4.0);
}

/// Conteúdo da busca sem `Frame`: usar dentro de cards existentes
/// para garantir zero card-in-card.
fn search_bar_inner(ui: &mut egui::Ui, hint: &str, query: &mut String) {
    // Ramo com histerese + linha com quebra: o campo mede o restante APÓS o
    // rótulo, então larguras calculadas nunca realimentam a medição.
    if is_wide_hysteresis(ui, "search_bar", 480.0) {
        ui.horizontal_wrapped(|ui| {
            ui.spacing_mut().item_spacing = Vec2::new(8.0, 8.0);
            ui.add(
                egui::Label::new(
                    egui::RichText::new(format!("🔍 {hint}"))
                        .size(13.0)
                        .color(ds::MUTED),
                )
                .wrap(),
            );
            // Largura fluida: ocupa o que sobra sem fixar 180–280px.
            let avail = ui.available_width();
            let btn_w = 96.0;
            let input_w = (avail - btn_w - 16.0).clamp(140.0, avail.max(140.0));
            ui.add(styled_singleline(query, "Digite para filtrar…").desired_width(input_w));
            if ui
                .add(ghost_button("✖ Limpar", 13.0).min_size(Vec2::new(btn_w, 40.0)))
                .clicked()
            {
                query.clear();
            }
        });
    } else {
        ui.vertical(|ui| {
            ui.add(
                egui::Label::new(
                    egui::RichText::new(format!("🔍 {hint}"))
                        .size(13.0)
                        .color(ds::MUTED),
                )
                .wrap(),
            );
            ui.add_space(4.0);
            ui.add(
                styled_singleline(query, "Digite para filtrar…")
                    .desired_width(ui.available_width()),
            );
            ui.add_space(8.0);
            if ui
                .add(ghost_button("✖ Limpar", 13.0).min_size(Vec2::new(ui.available_width(), 40.0)))
                .clicked()
            {
                query.clear();
            }
        });
    }
}

enum DayInputState {
    Empty,
    Partial,
    Valid(String),
    Invalid,
}

/// Normaliza separadores (/, ., espaço → -) e a ordem BR DD-MM-AAAA
/// para o padrão SQLite AAAA-MM-DD. Retorna None se irreconhecível.
fn normalized_day(raw: &str) -> Option<String> {
    let mut s = raw.trim().replace(['/', '.', ' '], "-");
    while s.contains("--") {
        s = s.replace("--", "-");
    }
    let parts: Vec<&str> = s.split('-').collect();
    if parts.len() != 3 || parts.iter().any(|p| p.is_empty()) {
        return None;
    }
    let canonical = if parts[0].len() == 4 {
        format!("{:0>4}-{:0>2}-{:0>2}", parts[0], parts[1], parts[2])
    } else if parts[2].len() == 4 && parts[0].len() <= 2 {
        format!("{:0>4}-{:0>2}-{:0>2}", parts[2], parts[1], parts[0])
    } else {
        return None;
    };
    Some(canonical)
}

fn day_input_state(raw: &str) -> DayInputState {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return DayInputState::Empty;
    }
    if let Some(norm) = normalized_day(trimmed) {
        if chrono::NaiveDate::parse_from_str(&norm, "%Y-%m-%d").is_ok() {
            return DayInputState::Valid(norm);
        }
        if norm.len() >= 10 {
            return DayInputState::Invalid;
        }
        return DayInputState::Partial;
    }
    if trimmed.len() < 10
        && trimmed
            .chars()
            .all(|c| c.is_ascii_digit() || matches!(c, '-' | '/' | '.' | ' '))
    {
        DayInputState::Partial
    } else {
        DayInputState::Invalid
    }
}

fn day_filter_row(ui: &mut egui::Ui, day: &mut String, applied: &mut String, query: &mut String) {
    ds::card_frame().show(ui, |ui| {
        day_filter_row_inner(ui, day, applied, query);
    });
    ui.add_space(4.0);
}

/// Conteúdo do filtro de data sem `Frame`: usar dentro de cards existentes.
fn day_filter_row_inner(
    ui: &mut egui::Ui,
    day: &mut String,
    applied: &mut String,
    query: &mut String,
) {
    if is_wide_hysteresis(ui, "day_filter", BP_MEDIUM) {
        ui.horizontal_wrapped(|ui| {
            ui.spacing_mut().item_spacing = Vec2::new(8.0, 8.0);
            ui.add(
                egui::Label::new(
                    egui::RichText::new("📅 Data (AAAA-MM-DD):")
                        .size(13.0)
                        .color(ds::MUTED),
                )
                .wrap(),
            );
            // Campo fluido em vez de 140px fixos: usa o espaço restante.
            let avail = ui.available_width();
            let input_w = (avail.min(220.0)).clamp(120.0, avail.max(120.0));
            ui.add(styled_singleline(day, "AAAA-MM-DD…").desired_width(input_w));
            if ui
                .add(ghost_button("📅 Hoje", 13.0).min_size(Vec2::new(0.0, 40.0)))
                .clicked()
            {
                let today = models::hoje_local();
                *day = today.clone();
                *applied = today;
            }
            if ui
                .add(ghost_button("📋 Ver Tudo", 13.0).min_size(Vec2::new(0.0, 40.0)))
                .clicked()
            {
                day.clear();
                applied.clear();
            }
            if ui
                .add(ghost_button("✖", 13.0).min_size(Vec2::new(0.0, 40.0)))
                .clicked()
            {
                day.clear();
                query.clear();
            }
        });
    } else {
        ui.vertical(|ui| {
            ui.add(
                egui::Label::new(
                    egui::RichText::new("📅 Data (AAAA-MM-DD):")
                        .size(13.0)
                        .color(ds::MUTED),
                )
                .wrap(),
            );
            ui.add_space(4.0);
            ui.add(styled_singleline(day, "AAAA-MM-DD…").desired_width(ui.available_width()));
            ui.add_space(8.0);
            ui.horizontal_wrapped(|ui| {
                if ui
                    .add(ghost_button("📅 Hoje", 13.0).min_size(Vec2::new(0.0, 40.0)))
                    .clicked()
                {
                    let today = models::hoje_local();
                    *day = today.clone();
                    *applied = today;
                }
                if ui
                    .add(ghost_button("📋 Ver Tudo", 13.0).min_size(Vec2::new(0.0, 40.0)))
                    .clicked()
                {
                    day.clear();
                    applied.clear();
                }
                if ui
                    .add(ghost_button("✖ Limpar data", 13.0).min_size(Vec2::new(0.0, 40.0)))
                    .clicked()
                {
                    day.clear();
                    query.clear();
                }
            });
        });
    }
}

fn section_header(ui: &mut egui::Ui, icon: &str, title: &str, subtitle: &str) {
    ui.add_space(16.0);
    // Emblema e título em widgets próprios com respiro 8.0 (grade 4pt):
    // o ícone nunca cola ou sobrepõe o texto, mesmo com quebra de linha.
    ui.horizontal_wrapped(|ui| {
        ui.spacing_mut().item_spacing = Vec2::new(8.0, 4.0);
        ui.add(egui::Label::new(
            egui::RichText::new(icon).size(18.0).color(ds::TITLE),
        ));
        ui.add(
            egui::Label::new(
                egui::RichText::new(title)
                    .size(18.0)
                    .strong()
                    .color(ds::TITLE),
            )
            .wrap(),
        );
    });
    if !subtitle.is_empty() {
        ui.add_space(4.0);
        wrap_label(ui, subtitle, 12.0, ds::MUTED, false);
    }
    ui.add_space(12.0);
}

fn page_header(ui: &mut egui::Ui, icon: &str, title: &str, subtitle: &str) {
    ui.horizontal_wrapped(|ui| {
        ui.spacing_mut().item_spacing = Vec2::new(8.0, 4.0);
        ui.add(egui::Label::new(
            egui::RichText::new(icon).size(24.0).color(ds::TITLE),
        ));
        ui.add(
            egui::Label::new(
                egui::RichText::new(title)
                    .size(24.0)
                    .strong()
                    .color(ds::TITLE),
            )
            .wrap(),
        );
    });
    ui.add_space(4.0);
    wrap_label(ui, subtitle, 12.0, ds::MUTED, false);
    ui.add_space(12.0);
}

fn format_day_br(day: &str) -> String {
    let mut it = day.splitn(3, '-');
    match (it.next(), it.next(), it.next()) {
        (Some(y), Some(m), Some(d)) => format!("{d}/{m}/{y}"),
        _ => day.to_string(),
    }
}

fn apply_sales_query(sales: Vec<models::Sale>, query: &str) -> Vec<models::Sale> {
    let q = query.trim().to_lowercase();
    if q.is_empty() {
        return sales;
    }
    sales
        .into_iter()
        .filter(|s| {
            s.id.to_string().contains(q.as_str())
                || s.name.to_lowercase().contains(q.as_str())
                || s.client_name.to_lowercase().contains(q.as_str())
                || s.phone.to_lowercase().contains(q.as_str())
                || s.imei.to_lowercase().contains(q.as_str())
        })
        .collect()
}

fn table_header_label(text: &str) -> egui::RichText {
    egui::RichText::new(text)
        .size(12.0)
        .strong()
        .color(ds::MUTED)
}

fn table_cell(text: String) -> egui::RichText {
    egui::RichText::new(text).size(13.0).color(ds::BODY)
}

// Larguras fixas das colunas de texto livre: constantes eliminam a
// realimentação medida→quebra→medida do `remainder` dentro do scroll
// horizontal (as células usam `.truncate()`, linha única com elipse;
// o egui exibe o texto completo no tooltip ao passar o mouse).
const TABLE_TEXT_COL_W: f32 = 160.0;
const TABLE_DESC_COL_W: f32 = 200.0;

/// Tabela de vendas com ação de reimpressão do recibo por linha.
/// Retorna a venda cujo `🖨️` foi clicado (o chamador abre o recibo) —
/// evita clonar a lista inteira a cada frame.
/// Ação do menu "..." nas linhas de venda (tabelas + fiado).
#[derive(Debug, Clone, Copy)]
enum SaleRowAction {
    /// Reimprimir recibo (recarrega `items` via `get_sale_by_id`).
    Reprint(i64),
    /// Pedir confirmação de exclusão (id + total p/ mensagem).
    Delete { id: i64, total: f64 },
}

/// Alvo da confirmação de exclusão (`AppData::pending_delete`).
#[derive(Debug, Clone, Copy)]
pub struct PendingDelete {
    pub id: i64,
    pub total: f64,
}

fn sales_table(ui: &mut egui::Ui, sales: &[models::Sale]) -> Option<SaleRowAction> {
    if sales.is_empty() {
        empty_state(
            ui,
            "Nenhuma venda encontrada",
            "Ajuste a busca ou registre uma nova venda acima — ela aparece aqui.",
        );
        return None;
    }
    let mut action: Option<SaleRowAction> = None;
    ds::card_frame().show(ui, |ui| {
        egui::ScrollArea::horizontal()
            .id_salt("sales_table_scroll")
            .auto_shrink([false, false])
            .show(ui, |ui| {
                egui_extras::TableBuilder::new(ui)
                    .id_salt("sales_table")
                    .striped(true)
                    // A rolagem vertical pertence à página (`dashboard/pos_scroll`);
                    // a interna do TableBuilder é desligada para não aninhar
                    // dois scrollers verticais sob o `ScrollArea::horizontal`.
                    .vscroll(false)
                    .column(Column::initial(56.0).clip(true))
                    .column(Column::initial(TABLE_TEXT_COL_W).clip(true))
                    .column(Column::initial(120.0).clip(true))
                    .column(Column::initial(56.0).clip(true))
                    .column(Column::initial(96.0).clip(true))
                    .column(Column::initial(116.0).clip(true))
                    .column(Column::initial(96.0).clip(true))
                    .column(Column::initial(104.0).clip(true))
                    .column(Column::initial(136.0).clip(true))
                    .column(Column::initial(64.0).clip(true))
                    .header(32.0, |mut header| {
                        header.col(|ui| {
                            ui.add(egui::Label::new(table_header_label("ID")).wrap());
                        });
                        header.col(|ui| {
                            ui.add(egui::Label::new(table_header_label("Nome")).wrap());
                        });
                        header.col(|ui| {
                            ui.add(egui::Label::new(table_header_label("Cliente")).wrap());
                        });
                        header.col(|ui| {
                            ui.add(egui::Label::new(table_header_label("Qtd")).wrap());
                        });
                        header.col(|ui| {
                            ui.add(egui::Label::new(table_header_label("Unitário")).wrap());
                        });
                        header.col(|ui| {
                            ui.add(egui::Label::new(table_header_label("Total")).wrap());
                        });
                        header.col(|ui| {
                            ui.add(egui::Label::new(table_header_label("Pagamento")).wrap());
                        });
                        header.col(|ui| {
                            ui.add(egui::Label::new(table_header_label("Situação")).wrap());
                        });
                        header.col(|ui| {
                            ui.add(egui::Label::new(table_header_label("Data/Hora")).wrap());
                        });
                        header.col(|ui| {
                            ui.add(egui::Label::new(table_header_label("Ações")).wrap());
                        });
                    })
                    .body(|mut body| {
                        for s in sales {
                            // Linha única truncada nas colunas de texto livre:
                            // altura da linha constante, sem realimentação.
                            body.row(52.0, |mut row| {
                                row.col(|ui| {
                                    ui.add(egui::Label::new(table_cell(s.id.to_string())).wrap());
                                });
                                row.col(|ui| {
                                    ui.add(egui::Label::new(table_cell(s.name.clone())).truncate());
                                });
                                row.col(|ui| {
                                    let cliente = if s.client_name.trim().is_empty() {
                                        "—".to_string()
                                    } else {
                                        s.client_name.clone()
                                    };
                                    ui.add(egui::Label::new(table_cell(cliente)).truncate());
                                });
                                row.col(|ui| {
                                    ui.add(
                                        egui::Label::new(table_cell(s.quantity.to_string())).wrap(),
                                    );
                                });
                                row.col(|ui| {
                                    ui.add(
                                        egui::Label::new(table_cell(models::unit_price_display(s)))
                                            .wrap(),
                                    );
                                });
                                row.col(|ui| {
                                    ui.add(
                                        egui::Label::new(
                                            egui::RichText::new(fmt_money(s.total_price))
                                                .size(13.0)
                                                .strong()
                                                .color(ds::EMERALD),
                                        )
                                        .wrap(),
                                    );
                                });
                                row.col(|ui| {
                                    ui.add(
                                        egui::Label::new(table_cell(
                                            translate_payment(&s.payment_method).to_string(),
                                        ))
                                        .wrap(),
                                    );
                                });
                                row.col(|ui| {
                                    let color = status_color(&s.status);
                                    ui.add(
                                        egui::Label::new(
                                            egui::RichText::new(status_label(&s.status))
                                                .size(13.0)
                                                .strong()
                                                .color(color),
                                        )
                                        .wrap(),
                                    );
                                });
                                row.col(|ui| {
                                    ui.add(
                                        egui::Label::new(table_cell(format_datetime_br(
                                            &s.created_at,
                                        )))
                                        .wrap(),
                                    );
                                });
                                row.col(|ui| {
                                    ui.menu_button("...", |ui| {
                                        if ui.button("🖨️ Visualizar / Reimprimir Recibo").clicked()
                                        {
                                            action = Some(SaleRowAction::Reprint(s.id));
                                            ui.close_menu();
                                        }
                                        let del = egui::Button::new(
                                            egui::RichText::new("🗑️ Excluir Venda")
                                                .size(13.0)
                                                .color(ds::CORAL),
                                        );
                                        if ui.add(del).clicked() {
                                            action = Some(SaleRowAction::Delete {
                                                id: s.id,
                                                total: s.total_price,
                                            });
                                            ui.close_menu();
                                        }
                                    });
                                });
                            });
                        }
                    });
            });
    });
    action
}

fn cash_table(ui: &mut egui::Ui, entries: &[models::MovimentoCaixa]) {
    if entries.is_empty() {
        empty_state(
            ui,
            "Nenhuma movimentação encontrada",
            "Registre uma entrada, saída ou perda acima — o histórico aparece aqui.",
        );
        return;
    }
    ds::card_frame().show(ui, |ui| {
        egui::ScrollArea::horizontal()
            .id_salt("cash_table_scroll")
            .auto_shrink([false, false])
            .show(ui, |ui| {
                egui_extras::TableBuilder::new(ui)
                    .id_salt("cash_table")
                    .striped(true)
                    // Mesma razão de `sales_table`: só o wrapper horizontal
                    // rola; a página (`cash_scroll`) cuida do vertical.
                    .vscroll(false)
                    .column(Column::initial(56.0).clip(true))
                    .column(Column::initial(116.0).clip(true))
                    .column(Column::initial(96.0).clip(true))
                    .column(Column::initial(TABLE_DESC_COL_W).clip(true))
                    .column(Column::initial(96.0).clip(true))
                    .column(Column::initial(136.0).clip(true))
                    .header(32.0, |mut h| {
                        h.col(|ui| {
                            ui.add(egui::Label::new(table_header_label("ID")).wrap());
                        });
                        h.col(|ui| {
                            ui.add(egui::Label::new(table_header_label("Valor")).wrap());
                        });
                        h.col(|ui| {
                            ui.add(egui::Label::new(table_header_label("Tipo")).wrap());
                        });
                        h.col(|ui| {
                            ui.add(egui::Label::new(table_header_label("Descrição")).wrap());
                        });
                        h.col(|ui| {
                            ui.add(egui::Label::new(table_header_label("Origem")).wrap());
                        });
                        h.col(|ui| {
                            ui.add(egui::Label::new(table_header_label("Data/Hora")).wrap());
                        });
                    })
                    .body(|mut body| {
                        for e in entries {
                            body.row(52.0, |mut row| {
                                row.col(|ui| {
                                    ui.add(egui::Label::new(table_cell(e.id.to_string())).wrap());
                                });
                                row.col(|ui| {
                                    let color = match e.tipo {
                                        models::TipoMovimentoCaixa::Entrada => ds::EMERALD,
                                        models::TipoMovimentoCaixa::Saida => ds::AMBER,
                                        models::TipoMovimentoCaixa::Perda => ds::CORAL,
                                    };
                                    ui.add(
                                        egui::Label::new(
                                            egui::RichText::new(fmt_money(e.amount))
                                                .size(13.0)
                                                .strong()
                                                .color(color),
                                        )
                                        .wrap(),
                                    );
                                });
                                row.col(|ui| {
                                    let (label, color) = match e.tipo {
                                        models::TipoMovimentoCaixa::Entrada => {
                                            ("Entrada", ds::EMERALD)
                                        }
                                        models::TipoMovimentoCaixa::Saida => ("Saída", ds::AMBER),
                                        models::TipoMovimentoCaixa::Perda => ("Perda", ds::CORAL),
                                    };
                                    ui.add(
                                        egui::Label::new(
                                            egui::RichText::new(label)
                                                .size(13.0)
                                                .strong()
                                                .color(color),
                                        )
                                        .wrap(),
                                    );
                                });
                                row.col(|ui| {
                                    ui.add(
                                        egui::Label::new(table_cell(e.description.clone()))
                                            .truncate(),
                                    );
                                });
                                row.col(|ui| {
                                    ui.add(
                                        egui::Label::new(table_cell(
                                            source_label(&e.source).to_string(),
                                        ))
                                        .wrap(),
                                    );
                                });
                                row.col(|ui| {
                                    ui.add(
                                        egui::Label::new(table_cell(format_datetime_br(
                                            &e.created_at,
                                        )))
                                        .wrap(),
                                    );
                                });
                            });
                        }
                    });
            });
    });
}

/// Tabela de perdas (uso plano, sem `Frame` próprio): quando usada de forma
/// isolada, o chamador envolve em `ds::card_frame()`; dentro de `report_card`
/// usa-se diretamente para garantir zero card-in-card.
fn loss_table_inner(
    ui: &mut egui::Ui,
    losses: &[models::Loss],
    scroll_salt: &str,
    table_salt: &str,
) {
    if losses.is_empty() {
        empty_state_flat(
            ui,
            "Nenhuma perda registrada",
            "Que ótimo — quando houver uma perda, ela aparece aqui com origem e data.",
        );
        return;
    }
    // `both` explícito é o único scroller vertical (cap 280px); o interno
    // do TableBuilder é desligado para não aninhar verticais.
    egui::ScrollArea::both()
        .id_salt(scroll_salt)
        .max_height(280.0)
        .auto_shrink([false, false])
        .show(ui, |ui| {
            egui_extras::TableBuilder::new(ui)
                .id_salt(table_salt)
                .striped(true)
                .vscroll(false)
                .column(Column::initial(56.0).clip(true))
                .column(Column::initial(116.0).clip(true))
                .column(Column::initial(TABLE_DESC_COL_W).clip(true))
                .column(Column::initial(96.0).clip(true))
                .column(Column::initial(136.0).clip(true))
                .header(32.0, |mut h| {
                    h.col(|ui| {
                        ui.add(egui::Label::new(table_header_label("ID")).wrap());
                    });
                    h.col(|ui| {
                        ui.add(egui::Label::new(table_header_label("Valor")).wrap());
                    });
                    h.col(|ui| {
                        ui.add(egui::Label::new(table_header_label("Descrição")).wrap());
                    });
                    h.col(|ui| {
                        ui.add(egui::Label::new(table_header_label("Origem")).wrap());
                    });
                    h.col(|ui| {
                        ui.add(egui::Label::new(table_header_label("Data/Hora")).wrap());
                    });
                })
                .body(|mut body| {
                    for l in losses {
                        body.row(52.0, |mut row| {
                            row.col(|ui| {
                                ui.add(egui::Label::new(table_cell(l.id.to_string())).wrap());
                            });
                            row.col(|ui| {
                                ui.add(
                                    egui::Label::new(
                                        egui::RichText::new(fmt_money(l.amount))
                                            .size(13.0)
                                            .strong()
                                            .color(ds::CORAL),
                                    )
                                    .wrap(),
                                );
                            });
                            row.col(|ui| {
                                ui.add(
                                    egui::Label::new(table_cell(l.description.clone())).truncate(),
                                );
                            });
                            row.col(|ui| {
                                ui.add(
                                    egui::Label::new(table_cell(
                                        source_label(&l.source).to_string(),
                                    ))
                                    .wrap(),
                                );
                            });
                            row.col(|ui| {
                                ui.add(
                                    egui::Label::new(table_cell(format_datetime_br(&l.created_at)))
                                        .wrap(),
                                );
                            });
                        });
                    }
                });
        });
}

/// Coluna de conteúdo com teto de largura: limita o estiramento horizontal
/// em monitores largos (enterprise, alinhado à esquerda) sem afetar telas
/// estreitas. Mesmo padrão do `kpi_grid` (`set_max_width` + clamp), seguro
/// dentro de `ScrollArea` vertical — sem `allocate_ui_with_layout`, que
/// realimentaria medição↔largura no scroller.
fn content_column(ui: &mut egui::Ui, add_contents: impl FnOnce(&mut egui::Ui)) {
    let w = ui.available_width().clamp(0.0, CONTENT_MAX_W);
    ui.vertical(|ui| {
        ui.set_max_width(w);
        add_contents(ui);
    });
}

fn render_dashboard(ui: &mut egui::Ui, app: &mut App) {
    egui::ScrollArea::vertical()
        .id_salt("dashboard_scroll")
        .auto_shrink([false; 2])
        .show(ui, |ui| {
            content_column(ui, |ui| {
                page_header(
                    ui,
                    "📊",
                    "Painel Geral",
                    "Acompanhe o desempenho da loja por período",
                );
                ui.add_space(4.0);

                ds::card_frame().show(ui, |ui| {
                    ui.horizontal_wrapped(|ui| {
                        ui.spacing_mut().item_spacing = Vec2::new(8.0, 8.0);
                        ui.add(
                            egui::Label::new(
                                egui::RichText::new("Período:").size(13.0).color(ds::MUTED),
                            )
                            .wrap(),
                        );
                        for period in [
                            DashboardPeriod::Diario,
                            DashboardPeriod::Mensal,
                            DashboardPeriod::Anual,
                        ] {
                            let selected = app.data.dashboard_period == period;
                            if ui.selectable_label(selected, period.label()).clicked() {
                                app.data.dashboard_period = period;
                            }
                        }
                    });
                });
                ui.add_space(12.0);

                let db = Arc::clone(&app.db_inner);
                let period = app.data.dashboard_period;
                match period {
                    DashboardPeriod::Diario => {
                        let days: Vec<String> = match lock_db(&db) {
                            Some(guard) => guard.distinct_days(),
                            None => {
                                error_banner(ui, "⚠️ Banco de dados temporariamente indisponível.");
                                Vec::new()
                            }
                        };
                        // O padrão (hoje local / escolha do usuário) sobrevive ao render:
                        // sem snap para o último dia com dados — dia sem movimento
                        // exibe o estado vazio correspondente.
                        ds::card_frame().show(ui, |ui| {
                            ui.horizontal_wrapped(|ui| {
                                ui.spacing_mut().item_spacing = Vec2::new(8.0, 8.0);
                                ui.add(
                                    egui::Label::new(
                                        egui::RichText::new("Dia:").size(13.0).color(ds::MUTED),
                                    )
                                    .wrap(),
                                );
                                if days.is_empty() {
                                    ui.add(
                                        egui::Label::new(
                                            egui::RichText::new("Sem registros ainda.")
                                                .size(13.0)
                                                .color(ds::MUTED),
                                        )
                                        .wrap(),
                                    );
                                } else {
                                    let day_label = format_day_br(&app.data.selected_day);
                                    egui::ComboBox::from_id_salt("dashboard_day")
                                        .selected_text(day_label)
                                        .show_ui(ui, |ui| {
                                            for d in &days {
                                                ui.selectable_value(
                                                    &mut app.data.selected_day,
                                                    d.clone(),
                                                    format_day_br(d),
                                                );
                                            }
                                        });
                                }
                            });
                        });
                        ui.add_space(12.0);

                        let day = app.data.selected_day.clone();
                        let (p_sales, p_in, p_out, p_losses, p_net, p_count) = match lock_db(&db) {
                            Some(guard) => (
                                guard.total_sales_for_day(&day),
                                guard.total_cash_in_for_day(&day),
                                guard.total_cash_out_for_day(&day),
                                guard.total_losses_for_day(&day),
                                guard.net_balance_for_day(&day),
                                guard.count_sales_for_day(&day),
                            ),
                            None => {
                                error_banner(ui, "⚠️ Banco de dados temporariamente indisponível.");
                                (0.0, 0.0, 0.0, 0.0, 0.0, 0)
                            }
                        };
                        let saldo_devedor = match lock_db(&db) {
                            Some(guard) => guard.total_receivables_global(),
                            None => 0.0,
                        };

                        dashboard_kpis(
                            ui,
                            &format_day_br(&day),
                            p_sales,
                            p_in,
                            p_out,
                            p_net,
                            p_losses,
                            p_count,
                            saldo_devedor,
                        );

                        ui.add_space(12.0);
                        ui.separator();
                        section_header(
                            ui,
                            "🧾",
                            "Vendas do dia",
                            &format!(
                                "Transações de {} • filtro de busca aplicável",
                                format_day_br(&day)
                            ),
                        );

                        let period_sales = match lock_db(&db) {
                            Some(guard) => guard.get_sales_for_day(&day),
                            None => {
                                error_banner(ui, "⚠️ Banco de dados temporariamente indisponível.");
                                Vec::new()
                            }
                        };
                        let q = app.data.sales_query.clone();
                        app.data.sales = apply_sales_query(period_sales, &q);

                        search_bar(
                            ui,
                            "Buscar por ID, nome, cliente, telefone ou IMEI:",
                            &mut app.data.sales_query,
                        );
                        ui.add_space(8.0);

                        if let Some(a) = sales_table(ui, &app.data.sales) {
                            handle_row_action(app, a);
                        }
                        render_pending_delete(ui, app);
                    }
                    DashboardPeriod::Mensal => {
                        let months: Vec<String> = match lock_db(&db) {
                            Some(guard) => guard.distinct_months(),
                            None => {
                                error_banner(ui, "⚠️ Banco de dados temporariamente indisponível.");
                                Vec::new()
                            }
                        };
                        // Sem snap para o último mês com dados (ver comentário do dia).
                        ds::card_frame().show(ui, |ui| {
                            ui.horizontal_wrapped(|ui| {
                                ui.spacing_mut().item_spacing = Vec2::new(8.0, 8.0);
                                ui.add(
                                    egui::Label::new(
                                        egui::RichText::new("Mês:").size(13.0).color(ds::MUTED),
                                    )
                                    .wrap(),
                                );
                                if months.is_empty() {
                                    ui.add(
                                        egui::Label::new(
                                            egui::RichText::new("Sem registros ainda.")
                                                .size(13.0)
                                                .color(ds::MUTED),
                                        )
                                        .wrap(),
                                    );
                                } else {
                                    let month_label = format_month_br(&app.data.selected_month);
                                    egui::ComboBox::from_id_salt("dashboard_month")
                                        .selected_text(month_label)
                                        .show_ui(ui, |ui| {
                                            for m in &months {
                                                ui.selectable_value(
                                                    &mut app.data.selected_month,
                                                    m.clone(),
                                                    format_month_br(m),
                                                );
                                            }
                                        });
                                }
                            });
                        });
                        ui.add_space(12.0);

                        let month = app.data.selected_month.clone();
                        let (p_sales, p_in, p_out, p_losses, p_net, p_count) = match lock_db(&db) {
                            Some(guard) => (
                                guard.total_sales_for_month(&month),
                                guard.total_cash_in_for_month(&month),
                                guard.total_cash_out_for_month(&month),
                                guard.total_losses_for_month(&month),
                                guard.net_balance_for_month(&month),
                                guard.count_sales_for_month(&month),
                            ),
                            None => {
                                error_banner(ui, "⚠️ Banco de dados temporariamente indisponível.");
                                (0.0, 0.0, 0.0, 0.0, 0.0, 0)
                            }
                        };
                        let saldo_devedor = match lock_db(&db) {
                            Some(guard) => guard.total_receivables_global(),
                            None => 0.0,
                        };

                        dashboard_kpis(
                            ui,
                            &format_month_br(&month),
                            p_sales,
                            p_in,
                            p_out,
                            p_net,
                            p_losses,
                            p_count,
                            saldo_devedor,
                        );

                        ui.add_space(12.0);
                        ui.separator();
                        section_header(
                            ui,
                            "🧾",
                            "Vendas do mês",
                            &format!(
                                "Transações de {} • filtro de busca aplicável",
                                format_month_br(&month)
                            ),
                        );

                        let period_sales = match lock_db(&db) {
                            Some(guard) => guard.get_sales_for_month(&month),
                            None => {
                                error_banner(ui, "⚠️ Banco de dados temporariamente indisponível.");
                                Vec::new()
                            }
                        };
                        let q = app.data.sales_query.clone();
                        app.data.sales = apply_sales_query(period_sales, &q);

                        search_bar(
                            ui,
                            "Buscar por ID, nome, cliente, telefone ou IMEI:",
                            &mut app.data.sales_query,
                        );
                        ui.add_space(8.0);

                        if let Some(a) = sales_table(ui, &app.data.sales) {
                            handle_row_action(app, a);
                        }
                        render_pending_delete(ui, app);
                    }
                    DashboardPeriod::Anual => {
                        let years: Vec<String> = match lock_db(&db) {
                            Some(guard) => guard.distinct_years(),
                            None => {
                                error_banner(ui, "⚠️ Banco de dados temporariamente indisponível.");
                                Vec::new()
                            }
                        };
                        // Sem snap para o último ano com dados (ver comentário do dia).
                        ds::card_frame().show(ui, |ui| {
                            ui.horizontal_wrapped(|ui| {
                                ui.spacing_mut().item_spacing = Vec2::new(8.0, 8.0);
                                ui.add(
                                    egui::Label::new(
                                        egui::RichText::new("Ano:").size(13.0).color(ds::MUTED),
                                    )
                                    .wrap(),
                                );
                                if years.is_empty() {
                                    ui.add(
                                        egui::Label::new(
                                            egui::RichText::new("Sem registros ainda.")
                                                .size(13.0)
                                                .color(ds::MUTED),
                                        )
                                        .wrap(),
                                    );
                                } else {
                                    let year_label = app.data.selected_year.clone();
                                    egui::ComboBox::from_id_salt("dashboard_year")
                                        .selected_text(year_label)
                                        .show_ui(ui, |ui| {
                                            for y in &years {
                                                ui.selectable_value(
                                                    &mut app.data.selected_year,
                                                    y.clone(),
                                                    y.clone(),
                                                );
                                            }
                                        });
                                }
                            });
                        });
                        ui.add_space(12.0);

                        let year = app.data.selected_year.clone();
                        let (p_sales, p_in, p_out, p_losses, p_net, p_count) = match lock_db(&db) {
                            Some(guard) => (
                                guard.total_sales_for_year(&year),
                                guard.total_cash_in_for_year(&year),
                                guard.total_cash_out_for_year(&year),
                                guard.total_losses_for_year(&year),
                                guard.net_balance_for_year(&year),
                                guard.count_sales_for_year(&year),
                            ),
                            None => {
                                error_banner(ui, "⚠️ Banco de dados temporariamente indisponível.");
                                (0.0, 0.0, 0.0, 0.0, 0.0, 0)
                            }
                        };
                        let saldo_devedor = match lock_db(&db) {
                            Some(guard) => guard.total_receivables_global(),
                            None => 0.0,
                        };

                        dashboard_kpis(
                            ui,
                            &year,
                            p_sales,
                            p_in,
                            p_out,
                            p_net,
                            p_losses,
                            p_count,
                            saldo_devedor,
                        );

                        ui.add_space(12.0);
                        ui.separator();
                        section_header(
                            ui,
                            "🧾",
                            "Vendas do ano",
                            &format!("Transações de {year} • filtro de busca aplicável"),
                        );

                        let period_sales = match lock_db(&db) {
                            Some(guard) => guard.get_sales_for_year(&year),
                            None => {
                                error_banner(ui, "⚠️ Banco de dados temporariamente indisponível.");
                                Vec::new()
                            }
                        };
                        let q = app.data.sales_query.clone();
                        app.data.sales = apply_sales_query(period_sales, &q);

                        search_bar(
                            ui,
                            "Buscar por ID, nome, cliente, telefone ou IMEI:",
                            &mut app.data.sales_query,
                        );
                        ui.add_space(8.0);

                        if let Some(a) = sales_table(ui, &app.data.sales) {
                            handle_row_action(app, a);
                        }
                        render_pending_delete(ui, app);
                    }
                }
                ui.add_space(16.0);
            });
        });
}

#[allow(clippy::too_many_arguments)]
fn dashboard_kpis(
    ui: &mut egui::Ui,
    period_label: &str,
    sales: f64,
    cash_in: f64,
    cash_out: f64,
    balance: f64,
    losses: f64,
    count: i64,
    saldo_devedor: f64,
) {
    wrap_label(
        ui,
        &format!("Métricas — {period_label} • {count} venda(s)"),
        12.0,
        ds::MUTED,
        false,
    );
    ui.add_space(8.0);

    // Grade responsiva uniforme (ver `kpi_grid`): colunas iguais por
    // construção, sem scroll forçado nem realimentação de medição.
    let kpis: [(&str, String, Color32); 6] = [
        (
            "Total em Vendas",
            crate::export::fmt_reais_br(sales),
            ds::BLUE,
        ),
        (
            "Entradas",
            crate::export::fmt_reais_br(cash_in),
            ds::EMERALD,
        ),
        ("Saídas", crate::export::fmt_reais_br(cash_out), ds::AMBER),
        ("Perdas", crate::export::fmt_reais_br(losses), ds::CORAL),
        (
            "Saldo Atual",
            crate::export::fmt_reais_br(balance),
            ds::EMERALD,
        ),
        (
            "Saldo Devedor",
            crate::export::fmt_reais_br(saldo_devedor),
            ds::AMBER,
        ),
    ];
    kpi_grid(ui, 3, &kpis);

    ui.add_space(12.0);
    if count == 0 {
        warning_line(
            ui,
            "⚠️ Nenhuma venda no período selecionado.".to_string(),
            ds::AMBER,
        );
    }
    if saldo_devedor > 0.0 {
        warning_line(
            ui,
            format!(
                "📒 Há {} em fiado a receber (global, fora do filtro de período).",
                fmt_money(saldo_devedor)
            ),
            ds::AMBER,
        );
    }
    if losses > 0.0 {
        warning_line(
            ui,
            format!(
                "⚠️ Perdas no período: {} — verifique os motivos.",
                fmt_money(losses)
            ),
            ds::CORAL,
        );
    }
}

fn kpi(ui: &mut egui::Ui, title: &str, value: &str, color: Color32) {
    // 1. Measure the exact column width BEFORE the frame
    let col_w = ui.available_width().max(0.0);
    let margin = ds::card_margin();

    egui::Frame::none()
        .fill(ds::CARD)
        .stroke(egui::Stroke::new(1.0, ds::BORDER))
        .rounding(egui::Rounding::same(ds::CARD_ROUNDING))
        .inner_margin(margin)
        .show(ui, |ui| {
            // 2. EXACT INNER SIZING: Subtract margins so the outer Frame perfectly fits the column
            let inner_w = (col_w - margin.left - margin.right).max(0.0);
            ui.set_min_size(egui::vec2(inner_w, KPI_CARD_MIN_H));

            // 3. Render contents cleanly, packing naturally top-down
            ui.vertical(|ui| {
                // Title row (wrapped + top-aligned: long titles flow to a
                // second line instead of clipping to "…").
                ui.horizontal_wrapped(|ui| {
                    ui.spacing_mut().item_spacing = egui::vec2(8.0, 0.0);
                    ui.label(egui::RichText::new("●").size(10.0).color(color));
                    ui.add(
                        egui::Label::new(egui::RichText::new(title).size(12.0).color(ds::MUTED))
                            .wrap(),
                    );
                });

                ui.add_space(8.0);

                // Value row
                ui.add(
                    egui::Label::new(
                        egui::RichText::new(value)
                            .size(kpi_value_size(value))
                            .strong()
                            .color(color),
                    )
                    .wrap(),
                );
            });
        });
}

/// Nº de colunas do KPI por largura, com histerese anti-vibração na
/// fronteira (evita 1 frame de flip quando a scrollbar aparece/some).
/// Limiares: ≥900px = 3 colunas, ≥600px = 2, senão 1. Capped em `max_cols`.
fn kpi_num_cols(ui: &egui::Ui, max_cols: usize) -> usize {
    let want = if is_wide_hysteresis(ui, "kpi_cols_3", 900.0) {
        3
    } else if is_wide_hysteresis(ui, "kpi_cols_2", 600.0) {
        2
    } else {
        1
    };
    want.min(max_cols.max(1)).max(1)
}

/// Grade de KPIs em colunas matematicamente exatas: `ui.columns` reparte o
/// retângulo em faixas iguais por aritmética interna — sem decisão de quebra
/// por widget (adeus wrap prematuro) e sem tabela rígida. Fileiras via
/// `chunks()`; respiro só ENTRE fileiras; teto de 1100px anti-esticamento.
fn kpi_grid(ui: &mut egui::Ui, max_cols: usize, cards: &[(&str, String, Color32)]) {
    if cards.is_empty() {
        return;
    }
    let num_cols = kpi_num_cols(ui, max_cols);
    // Teto em ultrawide, alinhado à esquerda: dentro dele, as colunas
    // dividem igualmente e os cards nunca esticam ao infinito.
    let grid_w = ui.available_width().clamp(0.0, KPI_GRID_MAX_W);
    ui.vertical(|ui| {
        ui.set_max_width(grid_w);
        let rows = cards.chunks(num_cols).len();
        for (i, row_cards) in cards.chunks(num_cols).enumerate() {
            ui.columns(row_cards.len(), |columns| {
                for ((title, value, color), col) in row_cards.iter().zip(columns) {
                    kpi(col, title, value, *color);
                }
            });
            if i + 1 < rows {
                ui.add_space(KPI_GAP);
            }
        }
    });
}

/// Seletor do tipo de atendimento (Venda Rápida / Serviço / Aparelho).
/// Reusa o padrão visual da barra de pagamento: selecionado em destaque
/// esmeralda, 44px de altura mínima, ramo fluido wide / empilhado narrow.
fn sale_type_selector(
    ui: &mut egui::Ui,
    sale_type: &mut models::SaleType,
    sale_error: &mut String,
) {
    use models::SaleType;
    const OPTIONS: [(SaleType, &str); 3] = [
        (SaleType::Rapida, "⚡ Venda Rápida"),
        (SaleType::Servico, "🔧 Serviço"),
        (SaleType::Aparelho, "📱 Aparelho"),
    ];
    let button = |selected: bool, label: &str, w: f32| {
        let mut btn = egui::Button::new(egui::RichText::new(label).size(14.0).strong())
            .selected(selected)
            .rounding(Rounding::same(8.0))
            .min_size(Vec2::new(w, 44.0));
        if selected {
            btn = btn
                .fill(ds::EMERALD_SOFT_BG)
                .stroke(egui::Stroke::new(1.5, ds::EMERALD));
        } else {
            btn = btn
                .fill(ds::INPUT_BG)
                .stroke(egui::Stroke::new(1.0, ds::BORDER));
        }
        btn
    };
    if is_wide_hysteresis(ui, "pos_type", 560.0) {
        let btn_w = fluid_button_width(ui.available_width(), 3, 8.0, 130.0);
        ui.horizontal_wrapped(|ui| {
            ui.spacing_mut().item_spacing = Vec2::new(8.0, 8.0);
            for (variant, label) in OPTIONS {
                let selected = *sale_type == variant;
                if ui.add(button(selected, label, btn_w)).clicked() && !selected {
                    *sale_type = variant;
                    sale_error.clear();
                }
            }
        });
    } else {
        ui.vertical(|ui| {
            for (variant, label) in OPTIONS {
                let selected = *sale_type == variant;
                if ui
                    .add(button(selected, label, ui.available_width()))
                    .clicked()
                    && !selected
                {
                    *sale_type = variant;
                    sale_error.clear();
                }
                ui.add_space(8.0);
            }
        });
    }
}

/// Impressão digital do rascunho do PDV (anti-vazamento do recibo).
/// Qualquer edição — texto, tipo, pagamento ou fiado — muda o `Debug` do
/// `NewSale` e invalida o botão de impressão da venda anterior.
fn pos_form_fp(data: &models::NewSale) -> String {
    format!("{data:?}")
}

/// Toast de erro curto (4s), para falhas de ação nas tabelas.
fn toast_error(app: &mut App, msg: String) {
    app.toasts.add(egui_toast::Toast {
        text: msg.into(),
        kind: egui_toast::ToastKind::Error,
        options: egui_toast::ToastOptions::default().duration(std::time::Duration::from_secs(4)),
        ..Default::default()
    });
}

/// Executa a ação do menu "..." das tabelas de venda.
fn handle_row_action(app: &mut App, action: SaleRowAction) {
    match action {
        SaleRowAction::Reprint(id) => {
            // Recarrega do banco: as linhas da tabela não têm `items`.
            let db = Arc::clone(&app.db_inner);
            let full = match lock_db(&db) {
                Some(guard) => guard.get_sale_by_id(id),
                None => None,
            };
            match full {
                Some(sale) => print_sale_receipt(app, &sale),
                None => toast_error(app, format!("⚠️ Venda #{id} não encontrada.")),
            }
        }
        SaleRowAction::Delete { id, total } => {
            app.data.pending_delete = Some(PendingDelete { id, total });
        }
    }
}

/// Cartão de confirmação de exclusão (logo abaixo da tabela chamadora).
/// Métricas e listas se corrigem sozinhas no próximo frame (somas ao vivo).
fn render_pending_delete(ui: &mut egui::Ui, app: &mut App) {
    let Some(target) = app.data.pending_delete else {
        return;
    };
    ui.add_space(8.0);
    egui::Frame::none()
        .fill(ds::CORAL_DIM_BG)
        .stroke(egui::Stroke::new(1.0, ds::CORAL))
        .rounding(Rounding::same(8.0))
        .inner_margin(Margin::symmetric(12.0, 8.0))
        .show(ui, |ui| {
            ui.add(
                egui::Label::new(
                    egui::RichText::new(format!(
                        "Excluir Venda #{} ({})? Esta ação atualizará o caixa e os relatórios.",
                        target.id,
                        fmt_money(target.total)
                    ))
                    .size(13.0)
                    .strong()
                    .color(ds::CORAL),
                )
                .wrap(),
            );
            ui.add_space(8.0);
            ui.horizontal_wrapped(|ui| {
                ui.spacing_mut().item_spacing = Vec2::new(8.0, 8.0);
                let confirm = egui::Button::new(
                    egui::RichText::new("🗑️ Confirmar Exclusão")
                        .size(14.0)
                        .strong()
                        .color(ds::CORAL),
                )
                .fill(ds::CORAL_DIM_BG)
                .stroke(egui::Stroke::new(1.5, ds::CORAL))
                .rounding(Rounding::same(8.0))
                .min_size(Vec2::new(0.0, 44.0));
                if ui.add(confirm).clicked() {
                    let db = Arc::clone(&app.db_inner);
                    match lock_db(&db) {
                        Some(mut guard) => match guard.delete_sale(target.id) {
                            Ok(()) => {
                                app.data.pending_delete = None;
                                if app
                                    .data
                                    .last_sale
                                    .as_ref()
                                    .is_some_and(|s| s.id == target.id)
                                {
                                    app.data.last_sale = None;
                                }
                                app.toasts.add(egui_toast::Toast {
                                    text: format!("Venda #{} excluída.", target.id).into(),
                                    kind: egui_toast::ToastKind::Success,
                                    options: egui_toast::ToastOptions::default()
                                        .duration(std::time::Duration::from_secs(3)),
                                    ..Default::default()
                                });
                            }
                            Err(e) => {
                                log::error!("Delete error for sale #{}: {e:#}", target.id);
                                toast_error(app, format!("⚠️ Não foi possível excluir: {e}"));
                            }
                        },
                        None => {
                            toast_error(
                                app,
                                "⚠️ Banco de dados temporariamente indisponível.".to_string(),
                            );
                        }
                    }
                }
                if ui
                    .add(ghost_button("Cancelar", 13.0).min_size(Vec2::new(0.0, 44.0)))
                    .clicked()
                {
                    app.data.pending_delete = None;
                }
            });
        });
    ui.add_space(8.0);
}

/// Abre o recibo/termo de garantia da venda no navegador padrão.
/// Sucesso e falha viram toast (a tabela não tem espaço para banners).
fn print_sale_receipt(app: &mut App, sale: &models::Sale) {
    let sale_id = sale.id;
    match crate::receipt::open_receipt_in_browser(sale) {
        Ok(()) => {
            app.toasts.add(egui_toast::Toast {
                text: format!("Recibo da venda #{sale_id} aberto no navegador.").into(),
                kind: egui_toast::ToastKind::Success,
                options: egui_toast::ToastOptions::default()
                    .duration(std::time::Duration::from_secs(3)),
                ..Default::default()
            });
        }
        Err(msg) => {
            log::error!("Receipt error for sale #{sale_id}: {msg}");
            app.toasts.add(egui_toast::Toast {
                text: msg.into(),
                kind: egui_toast::ToastKind::Error,
                options: egui_toast::ToastOptions::default()
                    .duration(std::time::Duration::from_secs(4)),
                ..Default::default()
            });
        }
    }
}

/// Linha de item do PDV multi-item: nome/qtd/unitário (+IMEI no Aparelho),
/// prévia do subtotal e lixeira (só com 2+ linhas, para nunca zerar o form).
#[allow(clippy::too_many_arguments)]
fn pos_item_row(
    ui: &mut egui::Ui,
    idx: usize,
    row_count: usize,
    item: &mut models::NewSaleItem,
    sale_type: models::SaleType,
    wide: bool,
    delete_idx: &mut Option<usize>,
    sale_error: &mut String,
) {
    ui.horizontal_wrapped(|ui| {
        ui.spacing_mut().item_spacing = Vec2::new(8.0, 8.0);
        wrap_label(ui, &format!("Item {}", idx + 1), 13.0, ds::TITLE, true);
        if row_count > 1 {
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui
                    .add(ghost_button("🗑️", 14.0).min_size(Vec2::new(48.0, 36.0)))
                    .on_hover_text("Remover este item")
                    .clicked()
                {
                    *delete_idx = Some(idx);
                    sale_error.clear();
                }
            });
        }
    });
    ui.add_space(4.0);
    if wide {
        egui::Grid::new(format!("pos_item_{idx}"))
            .num_columns(2)
            .min_col_width(150.0)
            .spacing(Vec2::new(12.0, 12.0))
            .show(ui, |ui| {
                pos_item_fields(ui, idx, item, sale_type, f32::INFINITY);
            });
    } else {
        ui.vertical(|ui| {
            pos_item_fields(ui, idx, item, sale_type, ui.available_width());
        });
    }
    // Prévia do subtotal (só quando parseável, para não poluir o form).
    let q: f64 = item.quantity.trim().parse().unwrap_or(0.0);
    let p: f64 = item
        .unit_price
        .trim()
        .replace(',', ".")
        .parse()
        .unwrap_or(0.0);
    if q > 0.0 && p > 0.0 {
        ui.add_space(4.0);
        wrap_label(
            ui,
            &format!("Subtotal: {}", fmt_money(q * p)),
            12.0,
            ds::MUTED,
            false,
        );
    }
}

/// Campos de uma linha de item (grade wide / empilhada narrow).
/// `w` infinito = ramo grade (usa `end_row`); finito = ramo empilhado.
/// No Aparelho, só o item #1 exige IMEI (linhas extras aceitam acessórios).
fn pos_item_fields(
    ui: &mut egui::Ui,
    idx: usize,
    item: &mut models::NewSaleItem,
    sale_type: models::SaleType,
    w: f32,
) {
    let imei_label = if idx == 0 {
        "IMEI / Nº de Série (obrigatório — garantia):"
    } else {
        "IMEI / Nº de Série (opcional):"
    };
    if w.is_infinite() {
        ui.add(
            egui::Label::new(
                egui::RichText::new(format!("{}:", models::item_label(sale_type)))
                    .size(13.0)
                    .color(ds::MUTED),
            )
            .wrap(),
        );
        ui.add(styled_singleline(&mut item.name, models::item_hint(sale_type)).desired_width(w));
        ui.end_row();
        ui.add(
            egui::Label::new(
                egui::RichText::new("Quantidade:")
                    .size(13.0)
                    .color(ds::MUTED),
            )
            .wrap(),
        );
        ui.add(styled_singleline(&mut item.quantity, "Ex.: 1…").desired_width(w));
        ui.end_row();
        ui.add(
            egui::Label::new(
                egui::RichText::new("Valor Unitário (R$):")
                    .size(13.0)
                    .color(ds::MUTED),
            )
            .wrap(),
        );
        ui.add(styled_singleline(&mut item.unit_price, "Ex.: 149,90…").desired_width(w));
        ui.end_row();
        if sale_type == models::SaleType::Aparelho {
            ui.add(
                egui::Label::new(egui::RichText::new(imei_label).size(13.0).color(ds::MUTED))
                    .wrap(),
            );
            ui.add(styled_singleline(&mut item.imei, "Ex.: 355123456789012…").desired_width(w));
            ui.end_row();
        }
    } else {
        wrap_label(
            ui,
            &format!("{}:", models::item_label(sale_type)),
            13.0,
            ds::MUTED,
            false,
        );
        ui.add_space(4.0);
        ui.add(styled_singleline(&mut item.name, models::item_hint(sale_type)).desired_width(w));
        ui.add_space(8.0);
        wrap_label(ui, "Quantidade:", 13.0, ds::MUTED, false);
        ui.add_space(4.0);
        ui.add(styled_singleline(&mut item.quantity, "Ex.: 1…").desired_width(w));
        ui.add_space(8.0);
        wrap_label(ui, "Valor Unitário (R$):", 13.0, ds::MUTED, false);
        ui.add_space(4.0);
        ui.add(styled_singleline(&mut item.unit_price, "Ex.: 149,90…").desired_width(w));
        if sale_type == models::SaleType::Aparelho {
            ui.add_space(8.0);
            wrap_label(ui, imei_label, 13.0, ds::MUTED, false);
            ui.add_space(4.0);
            ui.add(styled_singleline(&mut item.imei, "Ex.: 355123456789012…").desired_width(w));
        }
    }
}

/// Soma dos subtotais parseáveis das linhas (prévia do total geral).
fn pos_total_calc(items: &[models::NewSaleItem]) -> f64 {
    items
        .iter()
        .map(|it| {
            let q: f64 = it.quantity.trim().parse().unwrap_or(0.0);
            let p: f64 = it
                .unit_price
                .trim()
                .replace(',', ".")
                .parse()
                .unwrap_or(0.0);
            if q > 0.0 && p > 0.0 { q * p } else { 0.0 }
        })
        .sum()
}

fn render_pos(ui: &mut egui::Ui, app: &mut App) {
    egui::ScrollArea::vertical()
        .id_salt("pos_scroll")
        .auto_shrink([false; 2])
        .show(ui, |ui| {
            content_column(ui, |ui| {
                page_header(
                    ui,
                    "🛒",
                    "Registro de Venda",
                    "Cadastre produtos e serviços vendidos",
                );
                ui.add_space(4.0);

                let db = Arc::clone(&app.db_inner);
                let data = &mut app.data.new_sale;

                // Auto-reset anti-vazamento: qualquer edição no rascunho após
                // o registro invalida o botão de impressão da venda anterior.
                if app.data.last_sale.is_some() && pos_form_fp(data) != app.data.last_sale_fp {
                    app.data.last_sale = None;
                }

                ds::form_frame().show(ui, |ui| {
                    wrap_label(ui, "Tipo de Atendimento:", 13.0, ds::TITLE, true);
                    ui.add_space(8.0);
                    sale_type_selector(ui, &mut data.sale_type, &mut app.data.sale_error);
                    ui.add_space(4.0);
                    wrap_label(ui, data.sale_type.label_ptbr(), 12.0, ds::MUTED, false);
                    ui.add_space(12.0);
                    // 1 linha por padrão (venda simples ultrarrápida); a
                    // lixeira some na linha única para nunca zerar o form.
                    if data.items.is_empty() {
                        data.items.push(models::NewSaleItem::default());
                    }
                    let wide = is_wide_hysteresis(ui, "pos_form", BP_MEDIUM);
                    let sale_type = data.sale_type;
                    let mut delete_idx: Option<usize> = None;
                    let row_count = data.items.len();
                    for (idx, item) in data.items.iter_mut().enumerate() {
                        if idx > 0 {
                            ui.add_space(4.0);
                            ui.separator();
                            ui.add_space(4.0);
                        }
                        pos_item_row(
                            ui,
                            idx,
                            row_count,
                            item,
                            sale_type,
                            wide,
                            &mut delete_idx,
                            &mut app.data.sale_error,
                        );
                    }
                    if let Some(idx) = delete_idx {
                        data.items.remove(idx);
                    }
                    // IMEI fora do Aparelho: limpa resíduos de troca de tipo
                    // (mesma semântica do form antigo de item único).
                    if data.sale_type != models::SaleType::Aparelho {
                        for item in data.items.iter_mut() {
                            item.imei.clear();
                        }
                    }
                    ui.add_space(8.0);
                    if ui
                        .add(
                            ghost_button("➕ Adicionar Outro Item", 13.0)
                                .min_size(Vec2::new(ui.available_width(), 44.0)),
                        )
                        .clicked()
                    {
                        data.items.push(models::NewSaleItem::default());
                        app.data.sale_error.clear();
                    }

                    ui.add_space(16.0);
                    // Barra primária: só Dinheiro, Cartão e Pix. No fiado, o meio
                    // escolhido aqui é o da entrada (valor recebido agora).
                    let metodo_titulo = if data.is_fiado {
                        "Meio da entrada (valor recebido agora):"
                    } else {
                        "Forma de Pagamento:"
                    };
                    wrap_label(ui, metodo_titulo, 13.0, ds::TITLE, true);
                    ui.add_space(8.0);
                    if is_wide_hysteresis(ui, "pos_pay", 560.0) {
                        // Linha fluida com quebra: nunca comprime além do mínimo legível.
                        let btn_w = fluid_button_width(ui.available_width(), 3, 8.0, 140.0);
                        ui.horizontal_wrapped(|ui| {
                            ui.spacing_mut().item_spacing = Vec2::new(8.0, 8.0);
                            for (variant, label) in [
                                (models::PaymentMethod::Cash, "💵 Dinheiro"),
                                (models::PaymentMethod::Card, "💳 Cartão"),
                                (models::PaymentMethod::Pix, "📱 Pix"),
                            ] {
                                let selected = data.payment_method.as_ref() == Some(&variant);
                                let mut btn = egui::Button::new(
                                    egui::RichText::new(label).size(14.0).strong(),
                                )
                                .selected(selected)
                                .rounding(Rounding::same(8.0))
                                .min_size(Vec2::new(btn_w, 44.0));
                                if selected {
                                    btn = btn
                                        .fill(ds::EMERALD_SOFT_BG)
                                        .stroke(egui::Stroke::new(1.5, ds::EMERALD));
                                } else {
                                    btn = btn
                                        .fill(ds::INPUT_BG)
                                        .stroke(egui::Stroke::new(1.0, ds::BORDER));
                                }
                                if ui.add(btn).clicked() {
                                    data.payment_method = Some(variant);
                                    app.data.sale_error.clear();
                                }
                            }
                        });
                    } else {
                        ui.vertical(|ui| {
                            for (variant, label) in [
                                (models::PaymentMethod::Cash, "💵 Dinheiro"),
                                (models::PaymentMethod::Card, "💳 Cartão"),
                                (models::PaymentMethod::Pix, "📱 Pix"),
                            ] {
                                let selected = data.payment_method.as_ref() == Some(&variant);
                                let mut btn = egui::Button::new(
                                    egui::RichText::new(label).size(14.0).strong(),
                                )
                                .selected(selected)
                                .rounding(Rounding::same(8.0))
                                .min_size(Vec2::new(ui.available_width(), 44.0));
                                if selected {
                                    btn = btn
                                        .fill(ds::EMERALD_SOFT_BG)
                                        .stroke(egui::Stroke::new(1.5, ds::EMERALD));
                                } else {
                                    btn = btn
                                        .fill(ds::INPUT_BG)
                                        .stroke(egui::Stroke::new(1.0, ds::BORDER));
                                }
                                if ui.add(btn).clicked() {
                                    data.payment_method = Some(variant);
                                    app.data.sale_error.clear();
                                }
                                ui.add_space(8.0);
                            }
                        });
                    }
                    ui.add_space(8.0);
                    match &data.payment_method {
                        Some(pm) => {
                            let label = match pm {
                                models::PaymentMethod::Cash => "💵 Dinheiro",
                                models::PaymentMethod::Card => "💳 Cartão",
                                models::PaymentMethod::Pix => "📱 Pix",
                                // Legado: exibido se o banco contiver vendas antigas;
                                // a barra acima não oferece mais esta opção.
                                models::PaymentMethod::Fiado => "📒 Fiado",
                            };
                            wrap_label(
                                ui,
                                &format!("Pagamento selecionado: {label}"),
                                13.0,
                                ds::EMERALD,
                                true,
                            );
                        }
                        None => {
                            wrap_label(
                                ui,
                                "Nenhum pagamento selecionado — toque em uma opção acima.",
                                13.0,
                                ds::MUTED,
                                false,
                            );
                        }
                    }

                    // Total geral = soma dos subtotais parseáveis das linhas.
                    let total_calc = pos_total_calc(&data.items);
                    ui.add_space(12.0);
                    ui.horizontal_wrapped(|ui| {
                        ui.spacing_mut().item_spacing = Vec2::new(8.0, 8.0);
                        let mut fiado = data.is_fiado;
                        if ui
                            .checkbox(&mut fiado, "📒 Venda fiado / pagamento parcial")
                            .changed()
                        {
                            data.is_fiado = fiado;
                            app.data.sale_error.clear();
                        }
                    });
                    // Campos condicionais por tipo de atendimento: Serviço e
                    // Aparelho exigem cliente (garantia); Aparelho exige
                    // também IMEI/série. Venda Rápida mantém o form enxuto.
                    let needs_client = data.is_fiado || data.sale_type.requires_client();
                    if needs_client {
                        ui.add_space(8.0);
                        let client_label = if data.sale_type.requires_client() {
                            "Nome do Cliente (obrigatório — garantia):"
                        } else {
                            "Cliente (obrigatório no fiado):"
                        };
                        wrap_label(ui, client_label, 13.0, ds::MUTED, false);
                        ui.add_space(4.0);
                        ui.add(
                            styled_singleline(&mut data.client_name, "Ex.: Maria Silva…")
                                .desired_width(f32::INFINITY),
                        );
                    } else {
                        data.client_name.clear();
                    }
                    if data.sale_type != models::SaleType::Rapida {
                        ui.add_space(8.0);
                        wrap_label(ui, "Telefone (opcional):", 13.0, ds::MUTED, false);
                        ui.add_space(4.0);
                        ui.add(
                            styled_singleline(&mut data.phone, "Ex.: (11) 99999-0000…")
                                .desired_width(f32::INFINITY),
                        );
                    } else {
                        data.phone.clear();
                    }
                    if data.is_fiado {
                        ui.add_space(8.0);
                        wrap_label(ui, "Valor recebido agora (R$):", 13.0, ds::MUTED, false);
                        ui.add_space(4.0);
                        ui.add(
                            styled_singleline(
                                &mut data.paid_amount,
                                "Ex.: 100,00 ou 0 para pendente…",
                            )
                            .desired_width(f32::INFINITY),
                        );
                        ui.add_space(4.0);
                        let pago: f64 = data
                            .paid_amount
                            .trim()
                            .replace(',', ".")
                            .parse()
                            .unwrap_or(0.0);
                        let restante = (total_calc - pago).max(0.0);
                        // Destino da entrada segue a regra do caixa: só dinheiro
                        // físico vai para a gaveta; cartão/Pix vão para o banco.
                        let destino = match &data.payment_method {
                            Some(models::PaymentMethod::Cash) | None => "gaveta",
                            Some(_) => "banco",
                        };
                        wrap_label(
                            ui,
                            &format!(
                                "📒 Restante em aberto: {} • {} entra hoje ({destino}).",
                                fmt_money(restante),
                                fmt_money(pago.max(0.0))
                            ),
                            12.0,
                            ds::AMBER,
                            false,
                        );
                    } else {
                        data.paid_amount.clear();
                    }
                    ui.add_space(8.0);
                    // Faixa de total plana (sem borda): evita card-in-card dentro do formulário.
                    egui::Frame::none()
                        .fill(ds::INPUT_BG)
                        .rounding(Rounding::same(8.0))
                        .inner_margin(Margin::symmetric(12.0, 8.0))
                        .show(ui, |ui| {
                            ui.add(
                                egui::Label::new(
                                    egui::RichText::new(format!(
                                        "💰 Total Calculado: {}",
                                        fmt_money(total_calc)
                                    ))
                                    .size(14.0)
                                    .strong()
                                    .color(ds::TITLE),
                                )
                                .wrap(),
                            );
                        });
                });

                if !app.data.sale_error.is_empty() {
                    error_banner(ui, &app.data.sale_error.clone());
                } else {
                    ui.add_space(8.0);
                }

                if ui
                    .add(
                        success_button("✅ Registrar Venda")
                            .min_size(Vec2::new(ui.available_width(), 44.0)),
                    )
                    .clicked()
                {
                    if data.payment_method.is_none() {
                        app.data.sale_error = if data.is_fiado {
                            "⚠️ Selecione o meio da entrada (Dinheiro, Cartão ou Pix).".to_string()
                        } else {
                            "⚠️ Selecione a forma de pagamento.".to_string()
                        };
                    } else if data.items.is_empty() {
                        app.data.sale_error = "⚠️ Adicione ao menos 1 item à venda.".to_string();
                    } else if data.sale_type.requires_client() && data.client_name.trim().is_empty()
                    {
                        app.data.sale_error =
                            "⚠️ Informe o nome do cliente (obrigatório para garantia).".to_string();
                    } else {
                        // Validação por linha (o banco revalida tudo).
                        let mut row_err: Option<String> = None;
                        for (idx, it) in data.items.iter().enumerate() {
                            let n = idx + 1;
                            if it.name.trim().is_empty() {
                                row_err = Some(format!("⚠️ Item {n}: informe a descrição."));
                                break;
                            }
                            let q_ok = it.quantity.trim().parse::<i64>().is_ok_and(|v| v > 0);
                            let p_ok = it
                                .unit_price
                                .trim()
                                .replace(',', ".")
                                .parse::<f64>()
                                .is_ok_and(|v| v.is_finite() && v > 0.0);
                            if !q_ok || !p_ok {
                                row_err = Some(format!(
                                    "⚠️ Item {n}: quantidade e valor unitário válidos."
                                ));
                                break;
                            }
                            if idx == 0
                                && data.sale_type.requires_imei()
                                && it.imei.trim().is_empty()
                            {
                                row_err = Some(
                                    "⚠️ Item 1: informe o IMEI/nº de série do aparelho."
                                        .to_string(),
                                );
                                break;
                            }
                        }
                        if let Some(msg) = row_err {
                            app.data.sale_error = msg;
                        } else {
                            // Normaliza vírgula BR para ponto antes de persistir.
                            for it in data.items.iter_mut() {
                                it.unit_price = it.unit_price.trim().replace(',', ".");
                                it.name = it.name.trim().to_string();
                                it.imei = it.imei.trim().to_string();
                            }
                            if !data.paid_amount.trim().is_empty() {
                                data.paid_amount = data.paid_amount.trim().replace(',', ".");
                            }
                            data.client_name = data.client_name.trim().to_string();
                            data.phone = data.phone.trim().to_string();
                            // Validação rápida do parcial na UI (o banco revalida tudo).
                            let total_preview = pos_total_calc(&data.items);
                            let pago_preview: Option<f64> = if data.paid_amount.trim().is_empty() {
                                None
                            } else {
                                data.paid_amount.trim().parse().ok()
                            };
                            let fiado_err: Option<String> = if data.is_fiado {
                                if let Some(p) = pago_preview {
                                    if p < 0.0 || p - total_preview > 1e-6 {
                                        Some(
                                            "⚠️ Valor recebido deve estar entre R$ 0,00 e o total."
                                                .to_string(),
                                        )
                                    } else if p < total_preview - 1e-6
                                        && data.client_name.is_empty()
                                    {
                                        Some(
                                            "⚠️ Informe o nome do cliente para o fiado."
                                                .to_string(),
                                        )
                                    } else {
                                        None
                                    }
                                } else if data.client_name.is_empty() {
                                    Some("⚠️ Informe o nome do cliente para o fiado.".to_string())
                                } else {
                                    None
                                }
                            } else {
                                None
                            };
                            if let Some(msg) = fiado_err {
                                app.data.sale_error = msg;
                            } else {
                                match lock_db(&db) {
                                    Some(mut db_guard) => match db_guard.add_sale(data) {
                                        Ok(new_id) => {
                                            // Guarda a venda recém-criada para o
                                            // recibo: o ID vem do insert acima.
                                            app.data.last_sale = db_guard.get_sale_by_id(new_id);
                                            data.items = vec![models::NewSaleItem::default()];
                                            data.payment_method = None;
                                            data.client_name.clear();
                                            data.phone.clear();
                                            data.paid_amount.clear();
                                            data.is_fiado = false;
                                            // Linha de base do anti-vazamento: o
                                            // formulário limpo; qualquer digitação
                                            // a partir daqui derruba `last_sale`.
                                            app.data.last_sale_fp = pos_form_fp(data);
                                            app.data.sale_error.clear();
                                            app.toasts.add(egui_toast::Toast {
                                                text: "Venda registrada com sucesso!".into(),
                                                kind: egui_toast::ToastKind::Success,
                                                options: egui_toast::ToastOptions::default()
                                                    .duration(std::time::Duration::from_secs(3)),
                                                ..Default::default()
                                            });
                                        }
                                        Err(e) => {
                                            app.data.sale_error =
                                                format!("⚠️ Não foi possível registrar: {e}");
                                            log::error!("Sale error: {e:#}");
                                        }
                                    },
                                    None => {
                                        app.data.sale_error =
                                            "⚠️ Banco de dados temporariamente indisponível."
                                                .to_string();
                                    }
                                }
                            }
                        }
                    }
                }

                // Banner de sucesso pós-venda: só existe enquanto o rascunho
                // está intacto (ver anti-vazamento no topo do `render_pos`).
                if let Some(last) = app.data.last_sale.clone() {
                    ui.add_space(8.0);
                    egui::Frame::none()
                        .fill(ds::EMERALD_SOFT_BG)
                        .stroke(egui::Stroke::new(1.0, ds::EMERALD))
                        .rounding(Rounding::same(8.0))
                        .inner_margin(Margin::symmetric(12.0, 8.0))
                        .show(ui, |ui| {
                            ui.horizontal_wrapped(|ui| {
                                ui.spacing_mut().item_spacing = Vec2::new(8.0, 8.0);
                                ui.add(
                                    egui::Label::new(
                                        egui::RichText::new(format!(
                                            "✅ Venda #{} registrada com sucesso! ({})",
                                            last.id,
                                            last.sale_type.label_ptbr()
                                        ))
                                        .size(13.0)
                                        .strong()
                                        .color(ds::EMERALD),
                                    )
                                    .wrap(),
                                );
                                if ui
                                    .add(
                                        success_button("🖨️ Imprimir Garantia")
                                            .min_size(Vec2::new(0.0, 44.0)),
                                    )
                                    .clicked()
                                {
                                    print_sale_receipt(app, &last);
                                }
                            });
                        });
                }

                ui.add_space(16.0);
                ui.separator();
                section_header(
                    ui,
                    "🧾",
                    "Histórico de Vendas",
                    "Todas as transações com busca por ID, nome ou data",
                );

                {
                    let q = app.data.sales_query.clone();
                    let day_input = app.data.pos_day.clone();
                    match day_input_state(&day_input) {
                        DayInputState::Empty => app.data.pos_day_applied.clear(),
                        DayInputState::Valid(norm) => app.data.pos_day_applied = norm,
                        DayInputState::Partial | DayInputState::Invalid => {}
                    }
                    let applied = app.data.pos_day_applied.clone();
                    let db = Arc::clone(&app.db_inner);
                    match lock_db(&db) {
                        Some(guard) => {
                            if applied.is_empty() {
                                app.data.sales = guard.get_sales_filtered(&q);
                            } else {
                                let sales = guard.get_sales_for_day(&applied);
                                app.data.sales = apply_sales_query(sales, &q);
                            }
                        }
                        None => {
                            error_banner(ui, "⚠️ Banco de dados temporariamente indisponível.");
                        }
                    }
                }

                search_bar(
                    ui,
                    "Buscar por ID, nome, cliente, telefone ou IMEI:",
                    &mut app.data.sales_query,
                );
                ui.add_space(8.0);
                day_filter_row(
                    ui,
                    &mut app.data.pos_day,
                    &mut app.data.pos_day_applied,
                    &mut app.data.sales_query,
                );
                if matches!(day_input_state(&app.data.pos_day), DayInputState::Invalid) {
                    error_banner(ui, "⚠️ Data inválida. Use o formato AAAA-MM-DD.");
                }
                ui.add_space(8.0);

                if let Some(a) = sales_table(ui, &app.data.sales) {
                    handle_row_action(app, a);
                }
                render_pending_delete(ui, app);
                ui.add_space(16.0);
            });
        });
}

fn fiado_status_options() -> [&'static str; 3] {
    ["Todos em aberto", "Pendente", "Parcial"]
}

fn fiado_status_filter(idx: usize) -> Option<models::SaleStatus> {
    match idx {
        1 => Some(models::SaleStatus::Pendente),
        2 => Some(models::SaleStatus::Parcial),
        _ => None,
    }
}

/// Seletor do meio de pagamento do abate (Dinheiro, Cartão ou Pix).
/// Reusa o padrão visual da barra do PDV: selecionado em destaque esmeralda.
fn abate_method_selector(ui: &mut egui::Ui, app: &mut App) {
    let options = [
        (models::PaymentMethod::Cash, "💵 Dinheiro"),
        (models::PaymentMethod::Card, "💳 Cartão"),
        (models::PaymentMethod::Pix, "📱 Pix"),
    ];
    if is_wide_hysteresis(ui, "abate_method", 560.0) {
        let btn_w = fluid_button_width(ui.available_width(), 3, 8.0, 130.0);
        ui.horizontal_wrapped(|ui| {
            ui.spacing_mut().item_spacing = Vec2::new(8.0, 8.0);
            for (variant, label) in options {
                let selected = app.data.fiado_abate_method.as_ref() == Some(&variant);
                let mut btn = egui::Button::new(egui::RichText::new(label).size(14.0).strong())
                    .selected(selected)
                    .rounding(Rounding::same(8.0))
                    .min_size(Vec2::new(btn_w, 44.0));
                if selected {
                    btn = btn
                        .fill(ds::EMERALD_SOFT_BG)
                        .stroke(egui::Stroke::new(1.5, ds::EMERALD));
                } else {
                    btn = btn
                        .fill(ds::INPUT_BG)
                        .stroke(egui::Stroke::new(1.0, ds::BORDER));
                }
                if ui.add(btn).clicked() {
                    app.data.fiado_abate_method = Some(variant);
                    app.data.fiado_feedback.clear();
                }
            }
        });
    } else {
        ui.vertical(|ui| {
            for (variant, label) in options {
                let selected = app.data.fiado_abate_method.as_ref() == Some(&variant);
                let mut btn = egui::Button::new(egui::RichText::new(label).size(14.0).strong())
                    .selected(selected)
                    .rounding(Rounding::same(8.0))
                    .min_size(Vec2::new(ui.available_width(), 44.0));
                if selected {
                    btn = btn
                        .fill(ds::EMERALD_SOFT_BG)
                        .stroke(egui::Stroke::new(1.5, ds::EMERALD));
                } else {
                    btn = btn
                        .fill(ds::INPUT_BG)
                        .stroke(egui::Stroke::new(1.0, ds::BORDER));
                }
                if ui.add(btn).clicked() {
                    app.data.fiado_abate_method = Some(variant);
                    app.data.fiado_feedback.clear();
                }
                ui.add_space(8.0);
            }
        });
    }
    match &app.data.fiado_abate_method {
        Some(m) => {
            let destino = match m {
                models::PaymentMethod::Cash => "gaveta",
                _ => "banco",
            };
            wrap_label(
                ui,
                &format!(
                    "Meio selecionado: {} • entra hoje ({destino}).",
                    m.label_ptbr()
                ),
                13.0,
                ds::EMERALD,
                true,
            );
        }
        None => {
            wrap_label(
                ui,
                "Nenhum meio selecionado — padrão: Dinheiro (gaveta).",
                13.0,
                ds::MUTED,
                false,
            );
        }
    }
}

fn render_fiado(ui: &mut egui::Ui, app: &mut App) {
    egui::ScrollArea::vertical()
        .id_salt("fiado_scroll")
        .auto_shrink([false; 2])
        .show(ui, |ui| {
            content_column(ui, |ui| {
            page_header(
                ui,
                "📒",
                "Fiado / Recebíveis",
                "Contas a receber • abatimentos entram no caixa de hoje",
            );
            ui.add_space(4.0);

            let db = Arc::clone(&app.db_inner);
            let (saldo_devedor, qtd_aberto) = match lock_db(&db) {
                Some(guard) => (
                    guard.total_receivables_global(),
                    guard.get_pending_debts("", None).len(),
                ),
                None => {
                    error_banner(ui, "⚠️ Banco de dados temporariamente indisponível.");
                    (0.0, 0)
                }
            };
            let fiado_kpis = [
                (
                    "Saldo Devedor (global)",
                    fmt_money(saldo_devedor),
                    ds::AMBER,
                ),
                ("Contas em aberto", format!("{qtd_aberto}"), ds::TITLE),
            ];
            kpi_grid(ui, 2, &fiado_kpis);
            ui.add_space(12.0);

            ds::card_frame().show(ui, |ui| {
                search_bar_inner(
                    ui,
                    "Buscar por ID, produto ou cliente:",
                    &mut app.data.fiado_query,
                );
                ui.add_space(8.0);
                ui.horizontal_wrapped(|ui| {
                    ui.spacing_mut().item_spacing = Vec2::new(8.0, 8.0);
                    ui.add(
                        egui::Label::new(
                            egui::RichText::new("Situação:").size(13.0).color(ds::MUTED),
                        )
                        .wrap(),
                    );
                    let opts = fiado_status_options();
                    let sel = opts
                        .get(app.data.fiado_status_idx)
                        .copied()
                        .unwrap_or(opts[0]);
                    egui::ComboBox::from_id_salt("fiado_status")
                        .selected_text(sel)
                        .show_ui(ui, |ui| {
                            for (i, label) in opts.iter().enumerate() {
                                ui.selectable_value(&mut app.data.fiado_status_idx, i, *label);
                            }
                        });
                    if ui
                        .add(ghost_button("✖ Limpar", 13.0).min_size(Vec2::new(96.0, 40.0)))
                        .clicked()
                    {
                        app.data.fiado_query.clear();
                        app.data.fiado_status_idx = 0;
                    }
                });
            });
            ui.add_space(12.0);

            let status_f = fiado_status_filter(app.data.fiado_status_idx);
            let query = app.data.fiado_query.clone();
            let debts: Vec<models::Sale> = match lock_db(&db) {
                Some(guard) => guard.get_pending_debts(&query, status_f.as_ref()),
                None => {
                    error_banner(ui, "⚠️ Banco de dados temporariamente indisponível.");
                    Vec::new()
                }
            };

            section_header(
                ui,
                "🧾",
                "Contas em aberto",
                &format!(
                    "{} conta(s) • toque em Abater para registrar pagamento",
                    debts.len()
                ),
            );

            if debts.is_empty() {
                empty_state(
                    ui,
                    "Nenhuma conta em aberto",
                    "Quando houver fiado parcial ou pendente, ele aparece aqui para abate.",
                );
            } else {
                ds::card_frame().show(ui, |ui| {
                    egui::ScrollArea::horizontal()
                        .id_salt("fiado_table_scroll")
                        .auto_shrink([false, false])
                        .show(ui, |ui| {
                            egui_extras::TableBuilder::new(ui)
                                .id_salt("fiado_table")
                                .striped(true)
                                .vscroll(false)
                                .column(Column::initial(56.0).clip(true))
                                .column(Column::initial(TABLE_TEXT_COL_W).clip(true))
                                .column(Column::initial(120.0).clip(true))
                                .column(Column::initial(104.0).clip(true))
                                .column(Column::initial(104.0).clip(true))
                                .column(Column::initial(104.0).clip(true))
                                .column(Column::initial(96.0).clip(true))
                                .column(Column::initial(120.0).clip(true))
                                .column(Column::initial(160.0).clip(true))
                                .header(32.0, |mut h| {
                                    for t in [
                                        "ID",
                                        "Cliente",
                                        "Produto",
                                        "Total",
                                        "Pago",
                                        "Restante",
                                        "Situação",
                                        "Data/Hora",
                                        "Ação",
                                    ] {
                                        h.col(|ui| {
                                            ui.add(egui::Label::new(table_header_label(t)).wrap());
                                        });
                                    }
                                })
                                .body(|mut body| {
                                    for d in &debts {
                                        let selected = app.data.fiado_selected == Some(d.id);
                                        body.row(52.0, |mut row| {
                                            row.col(|ui| {
                                                ui.add(
                                                    egui::Label::new(table_cell(d.id.to_string()))
                                                        .wrap(),
                                                );
                                            });
                                            row.col(|ui| {
                                                ui.add(
                                                    egui::Label::new(table_cell(
                                                        d.client_name.clone(),
                                                    ))
                                                    .truncate(),
                                                );
                                            });
                                            row.col(|ui| {
                                                ui.add(
                                                    egui::Label::new(table_cell(d.name.clone()))
                                                        .truncate(),
                                                );
                                            });
                                            row.col(|ui| {
                                                ui.add(
                                                    egui::Label::new(table_cell(fmt_money(
                                                        d.total_price,
                                                    )))
                                                    .wrap(),
                                                );
                                            });
                                            row.col(|ui| {
                                                ui.add(
                                                    egui::Label::new(table_cell(fmt_money(
                                                        d.paid_amount,
                                                    )))
                                                    .wrap(),
                                                );
                                            });
                                            row.col(|ui| {
                                                ui.add(
                                                    egui::Label::new(
                                                        egui::RichText::new(fmt_money(
                                                            d.remaining(),
                                                        ))
                                                        .size(13.0)
                                                        .strong()
                                                        .color(ds::AMBER),
                                                    )
                                                    .wrap(),
                                                );
                                            });
                                            row.col(|ui| {
                                                let color = status_color(&d.status);
                                                ui.add(
                                                    egui::Label::new(
                                                        egui::RichText::new(status_label(
                                                            &d.status,
                                                        ))
                                                        .size(13.0)
                                                        .strong()
                                                        .color(color),
                                                    )
                                                    .wrap(),
                                                );
                                            });
                                            row.col(|ui| {
                                                ui.add(
                                                    egui::Label::new(table_cell(
                                                        format_datetime_br(&d.created_at),
                                                    ))
                                                    .wrap(),
                                                );
                                            });
                                            row.col(|ui| {
                                                ui.horizontal(|ui| {
                                                    ui.spacing_mut().item_spacing =
                                                        Vec2::new(8.0, 8.0);
                                                    let label = if selected {
                                                        "✓ Selecionado"
                                                    } else {
                                                        "Abater"
                                                    };
                                                    if ui
                                                        .add(
                                                            ghost_button(label, 12.0)
                                                                .min_size(Vec2::new(88.0, 36.0)),
                                                        )
                                                        .clicked()
                                                    {
                                                        app.data.fiado_selected = Some(d.id);
                                                        app.data.fiado_abate.clear();
                                                        app.data.fiado_abate_method = None;
                                                        app.data.fiado_feedback.clear();
                                                    }
                                                    if ui
                                                        .add(
                                                            ghost_button("🖨️", 14.0)
                                                                .min_size(Vec2::new(48.0, 36.0)),
                                                        )
                                                        .on_hover_text(
                                                            "Imprimir garantia / recibo",
                                                        )
                                                        .clicked()
                                                    {
                                                        // Recarrega do banco: a linha
                                                        // do fiado não tem `items`.
                                                        let db = Arc::clone(&app.db_inner);
                                                        let full = match lock_db(&db) {
                                                            Some(guard) => {
                                                                guard.get_sale_by_id(d.id)
                                                            }
                                                            None => None,
                                                        };
                                                        match full {
                                                            Some(sale) => print_sale_receipt(
                                                                app, &sale,
                                                            ),
                                                            None => toast_error(
                                                                app,
                                                                format!(
                                                                    "⚠️ Venda #{} não encontrada.",
                                                                    d.id
                                                                ),
                                                            ),
                                                        }
                                                    }
                                                });
                                            });
                                        });
                                    }
                                });
                        });
                });
            }
            ui.add_space(12.0);

            // Formulário de abate da conta selecionada.
            if let Some(sel_id) = app.data.fiado_selected {
                let sel: Option<models::Sale> = match lock_db(&db) {
                    Some(guard) => guard.get_sale_by_id(sel_id),
                    None => None,
                };
                match sel {
                    None => {
                        app.data.fiado_selected = None;
                    }
                    Some(s) if !s.is_debt() => {
                        success_banner(ui, &format!("✅ Conta #{} já está quitada.", s.id));
                        app.data.fiado_selected = None;
                    }
                    Some(s) => {
                        // Largura medida UMA vez acima do form: o botão de ação
                        // reusada sem remediar dentro do ScrollArea (sem loop).
                        let form_avail = ui.available_width().max(0.0);
                        ds::form_frame().show(ui, |ui| {
                            wrap_label(
                                ui,
                                &format!(
                                    "Abater conta #{} • {} • restante {}",
                                    s.id,
                                    s.client_name,
                                    fmt_money(s.remaining())
                                ),
                                14.0,
                                ds::TITLE,
                                true,
                            );
                            ui.add_space(4.0);
                            wrap_label(
                                ui,
                                "O valor entra no caixa de hoje (Dinheiro→gaveta, Cartão/Pix→banco) e reduz o restante.",
                                12.0,
                                ds::MUTED,
                                false,
                            );
                            ui.add_space(8.0);
                            ui.add(
                                styled_singleline(&mut app.data.fiado_abate, "Ex.: 50,00…")
                                    .desired_width(f32::INFINITY),
                            );
                            ui.add_space(12.0);
                            wrap_label(ui, "Meio deste abate:", 13.0, ds::TITLE, true);
                            ui.add_space(8.0);
                            abate_method_selector(ui, app);
                        });
                        if !app.data.fiado_feedback.is_empty() {
                            let msg = app.data.fiado_feedback.clone();
                            if app.data.fiado_is_error {
                                error_banner(ui, &msg);
                            } else {
                                success_banner(ui, &msg);
                            }
                        } else {
                            ui.add_space(8.0);
                        }
                        let btn_label =
                            format!("✅ Registrar Abate (restante {})", fmt_money(s.remaining()));
                        if ui
                            .add(
                                success_button(&btn_label)
                                    .min_size(Vec2::new(form_avail, 44.0)),
                            )
                            .clicked()
                        {
                            let raw = app.data.fiado_abate.trim().replace(',', ".");
                            // Sem seleção explícita, o abate entra como Dinheiro (gaveta).
                            let method = app
                                .data
                                .fiado_abate_method
                                .clone()
                                .unwrap_or(models::PaymentMethod::Cash);
                            match raw.parse::<f64>() {
                                Ok(v) if v.is_finite() && v > 0.0 => match lock_db(&db) {
                                    Some(mut guard) => {
                                        match guard.register_abate_fiado(s.id, v, &method) {
                                            Ok(novo) => {
                                                app.data.fiado_abate.clear();
                                                app.data.fiado_abate_method = None;
                                                app.data.fiado_is_error = false;
                                                if novo <= 1e-6 {
                                                    app.data.fiado_feedback = format!(
                                                        "✅ Conta #{} quitada com abate de {} ({}).",
                                                        s.id,
                                                        fmt_money(v),
                                                        method.label_ptbr()
                                                    );
                                                    app.data.fiado_selected = None;
                                                } else {
                                                    app.data.fiado_feedback = format!(
                                                        "✅ Abate de {} ({}) registrado. Restante: {}.",
                                                        fmt_money(v),
                                                        method.label_ptbr(),
                                                        fmt_money(novo)
                                                    );
                                                }
                                            }
                                            Err(e) => {
                                                app.data.fiado_feedback =
                                                    format!("⚠️ Não foi possível abater: {e}");
                                                app.data.fiado_is_error = true;
                                                log::error!("Abate error: {e:#}");
                                            }
                                        }
                                    }
                                    None => {
                                        app.data.fiado_feedback =
                                            "⚠️ Banco de dados temporariamente indisponível."
                                                .to_string();
                                        app.data.fiado_is_error = true;
                                    }
                                },
                                _ => {
                                    app.data.fiado_feedback =
                                        "⚠️ Informe um valor válido maior que zero.".to_string();
                                    app.data.fiado_is_error = true;
                                }
                            }
                        }
                    }
                }
            }
            ui.add_space(16.0);
            });
        });
}

fn render_cash_register(ui: &mut egui::Ui, app: &mut App) {
    egui::ScrollArea::vertical()
        .id_salt("cash_scroll")
        .auto_shrink([false; 2])
        .show(ui, |ui| {
            content_column(ui, |ui| {
            page_header(
                ui,
                "💰",
                "Controle de Caixa",
                "Entradas, saídas e dinheiro em gaveta",
            );
            ui.add_space(4.0);

            let db = Arc::clone(&app.db_inner);
            let data = &mut app.data.new_cash_entry;

            ds::form_frame().show(ui, |ui| {
                if is_wide_hysteresis(ui, "cash_form", BP_MEDIUM) {
                    egui::Grid::new("cash_form")
                        .num_columns(2)
                        .min_col_width(150.0)
                        .spacing(Vec2::new(12.0, 12.0))
                        .show(ui, |ui| {
                            ui.add(
                                egui::Label::new(
                                    egui::RichText::new("Valor (R$):")
                                        .size(13.0)
                                        .color(ds::MUTED),
                                )
                                .wrap(),
                            );
                            ui.add(
                                styled_singleline(&mut data.amount, "Ex.: 50,00…")
                                    .desired_width(f32::INFINITY),
                            );
                            ui.end_row();
                            ui.add(
                                egui::Label::new(
                                    egui::RichText::new("Descrição / Motivo:")
                                        .size(13.0)
                                        .color(ds::MUTED),
                                )
                                .wrap(),
                            );
                            ui.add(
                                styled_singleline(&mut data.description, "Ex.: Reforço de caixa…")
                                    .desired_width(f32::INFINITY),
                            );
                            ui.end_row();
                        });
                } else {
                    ui.vertical(|ui| {
                        wrap_label(ui, "Valor (R$):", 13.0, ds::MUTED, false);
                        ui.add_space(4.0);
                        ui.add(
                            styled_singleline(&mut data.amount, "Ex.: 50,00…")
                                .desired_width(ui.available_width()),
                        );
                        ui.add_space(8.0);
                        wrap_label(ui, "Descrição / Motivo:", 13.0, ds::MUTED, false);
                        ui.add_space(4.0);
                        ui.add(
                            styled_singleline(&mut data.description, "Ex.: Reforço de caixa…")
                                .desired_width(ui.available_width()),
                        );
                    });
                }

                ui.add_space(16.0);
                wrap_label(ui, "Tipo de movimentação:", 13.0, ds::TITLE, true);
                ui.add_space(8.0);
                if is_wide_hysteresis(ui, "cash_kind", 560.0) {
                    let kind_w = fluid_button_width(ui.available_width(), 3, 8.0, 140.0);
                    ui.horizontal_wrapped(|ui| {
                        ui.spacing_mut().item_spacing = Vec2::new(8.0, 8.0);
                        for (kind, label) in [
                            (models::MovementKind::Entrada, "➕ Entrada"),
                            (models::MovementKind::Saida, "➖ Saída / Despesa"),
                            (models::MovementKind::Perda, "⚠️ Perda / Prejuízo"),
                        ] {
                            let selected = app.data.movement_kind == kind;
                            let accent = match kind {
                                models::MovementKind::Entrada => ds::EMERALD,
                                models::MovementKind::Saida => ds::AMBER,
                                models::MovementKind::Perda => ds::CORAL,
                                models::MovementKind::Nenhum => ds::BORDER,
                            };
                            let soft_bg = match kind {
                                models::MovementKind::Entrada => ds::EMERALD_SOFT_BG,
                                models::MovementKind::Saida => ds::AMBER_DIM_BG,
                                models::MovementKind::Perda => ds::CORAL_DIM_BG,
                                models::MovementKind::Nenhum => ds::INPUT_BG,
                            };
                            let mut btn =
                                egui::Button::new(egui::RichText::new(label).size(14.0).strong())
                                    .selected(selected)
                                    .rounding(Rounding::same(8.0))
                                    .min_size(Vec2::new(kind_w, 44.0));
                            if selected {
                                btn = btn
                                    .fill(soft_bg)
                                    .stroke(egui::Stroke::new(1.5, accent));
                            } else {
                                btn = btn
                                    .fill(ds::INPUT_BG)
                                    .stroke(egui::Stroke::new(1.0, ds::BORDER));
                            }
                            if ui.add(btn).clicked() {
                                app.data.movement_kind = kind;
                                app.data.movement_error.clear();
                            }
                        }
                    });
                } else {
                    ui.vertical(|ui| {
                        for (kind, label) in [
                            (models::MovementKind::Entrada, "➕ Entrada"),
                            (models::MovementKind::Saida, "➖ Saída / Despesa"),
                            (models::MovementKind::Perda, "⚠️ Perda / Prejuízo"),
                        ] {
                            let selected = app.data.movement_kind == kind;
                            let accent = match kind {
                                models::MovementKind::Entrada => ds::EMERALD,
                                models::MovementKind::Saida => ds::AMBER,
                                models::MovementKind::Perda => ds::CORAL,
                                models::MovementKind::Nenhum => ds::BORDER,
                            };
                            let soft_bg = match kind {
                                models::MovementKind::Entrada => ds::EMERALD_SOFT_BG,
                                models::MovementKind::Saida => ds::AMBER_DIM_BG,
                                models::MovementKind::Perda => ds::CORAL_DIM_BG,
                                models::MovementKind::Nenhum => ds::INPUT_BG,
                            };
                            let mut btn =
                                egui::Button::new(egui::RichText::new(label).size(14.0).strong())
                                    .selected(selected)
                                    .rounding(Rounding::same(8.0))
                                    .min_size(Vec2::new(ui.available_width(), 44.0));
                            if selected {
                                btn = btn
                                    .fill(soft_bg)
                                    .stroke(egui::Stroke::new(1.5, accent));
                            } else {
                                btn = btn
                                    .fill(ds::INPUT_BG)
                                    .stroke(egui::Stroke::new(1.0, ds::BORDER));
                            }
                            if ui.add(btn).clicked() {
                                app.data.movement_kind = kind;
                                app.data.movement_error.clear();
                            }
                            ui.add_space(8.0);
                        }
                    });
                }

                let show_source = !matches!(
                    app.data.movement_kind,
                    models::MovementKind::Nenhum
                );
                if show_source {
                    let source_title = if app.data.movement_kind == models::MovementKind::Entrada {
                        "Onde entra o valor?"
                    } else {
                        "De onde sai o valor?"
                    };
                    ui.add_space(12.0);
                    wrap_label(ui, source_title, 13.0, ds::TITLE, true);
                    ui.add_space(8.0);
                    if is_wide_hysteresis(ui, "cash_source", 480.0) {
                        // Rótulos curtos (Gaveta/Banco) + legenda inline sempre
                        // visível, sem nenhum popup de hover.
                        let src_w = fluid_button_width(ui.available_width(), 2, 8.0, 150.0);
                        ui.horizontal_wrapped(|ui| {
                            ui.spacing_mut().item_spacing = Vec2::new(8.0, 8.0);
                            for variant in [models::CashSource::Drawer, models::CashSource::Bank] {
                                let selected = app.data.movement_source.as_ref() == Some(&variant);
                                let mut btn = egui::Button::new(
                                    egui::RichText::new(variant.short_label()).size(13.0),
                                )
                                .selected(selected)
                                .rounding(Rounding::same(8.0))
                                .min_size(Vec2::new(src_w, 44.0));
                                if selected {
                                    btn = btn
                                        .fill(ds::BLUE_DIM_BG)
                                        .stroke(egui::Stroke::new(1.5, ds::BLUE));
                                } else {
                                    btn = btn
                                        .fill(ds::INPUT_BG)
                                        .stroke(egui::Stroke::new(1.0, ds::BORDER));
                                }
                                if ui.add(btn).clicked() {
                                    app.data.movement_source = Some(variant);
                                    app.data.movement_error.clear();
                                }
                            }
                        });
                        ui.add_space(8.0);
                        wrap_label(
                            ui,
                            "💵 Gaveta = dinheiro físico • 🏦 Banco = conta / Pix.",
                            12.0,
                            ds::MUTED,
                            false,
                        );
                    } else {
                        ui.vertical(|ui| {
                            for variant in [models::CashSource::Drawer, models::CashSource::Bank] {
                                let selected = app.data.movement_source.as_ref() == Some(&variant);
                                let mut btn = egui::Button::new(
                                    egui::RichText::new(variant.short_label()).size(13.0),
                                )
                                .selected(selected)
                                .rounding(Rounding::same(8.0))
                                .min_size(Vec2::new(ui.available_width(), 44.0));
                                if selected {
                                    btn = btn
                                        .fill(ds::BLUE_DIM_BG)
                                        .stroke(egui::Stroke::new(1.5, ds::BLUE));
                                } else {
                                    btn = btn
                                        .fill(ds::INPUT_BG)
                                        .stroke(egui::Stroke::new(1.0, ds::BORDER));
                                }
                                if ui.add(btn).clicked() {
                                    app.data.movement_source = Some(variant);
                                    app.data.movement_error.clear();
                                }
                                ui.add_space(8.0);
                            }
                            wrap_label(
                                ui,
                                "💵 Gaveta = dinheiro físico • 🏦 Banco = conta / Pix.",
                                12.0,
                                ds::MUTED,
                                false,
                            );
                        });
                    }
                }
            });

            if !app.data.movement_error.is_empty() {
                error_banner(ui, &app.data.movement_error.clone());
            } else {
                ui.add_space(8.0);
            }

            if ui
                .add(primary_button("💵 Registrar Movimentação").min_size(Vec2::new(
                    ui.available_width(),
                    44.0,
                )))
                .clicked()
            {
                let kind = app.data.movement_kind.clone();
                let needs_source = !matches!(kind, models::MovementKind::Nenhum);
                if kind == models::MovementKind::Nenhum
                    || (needs_source && app.data.movement_source.is_none())
                {
                    app.data.movement_error =
                        "⚠️ Selecione o tipo de movimentação e a origem do valor.".to_string();
                } else if data.amount.trim().is_empty()
                    || data
                        .amount
                        .trim()
                        .replace(',', ".")
                        .parse::<f64>()
                        .map_or(true, |v| v <= 0.0)
                {
                    app.data.movement_error =
                        "⚠️ Informe um valor válido maior que zero.".to_string();
                } else if data.description.trim().is_empty() {
                    app.data.movement_error =
                        "⚠️ Informe a descrição/motivo da movimentação.".to_string();
                } else {
                    data.amount = data.amount.trim().replace(',', ".");
                    let source = app.data.movement_source.clone().unwrap_or_default();
                    match lock_db(&db) {
                        Some(mut db_guard) => {
                            let result = match kind {
                                models::MovementKind::Entrada => {
                                    data.is_entry = true;
                                    db_guard.add_cash_entry(data, &source)
                                }
                                models::MovementKind::Saida => {
                                    data.is_entry = false;
                                    db_guard.add_cash_entry(data, &source)
                                }
                                models::MovementKind::Perda => {
                                    let loss = models::NewLoss {
                                        amount: data.amount.clone(),
                                        description: data.description.clone(),
                                    };
                                    db_guard.add_loss(&loss, &source)
                                }
                                models::MovementKind::Nenhum => Ok(()),
                            };
                            match result {
                                Ok(()) => {
                                    data.amount.clear();
                                    data.description.clear();
                                    data.is_entry = true;
                                    app.data.movement_kind = models::MovementKind::Nenhum;
                                    app.data.movement_source = None;
                                    app.data.movement_error.clear();
                                }
                                Err(e) => {
                                    app.data.movement_error =
                                        format!("⚠️ Não foi possível registrar: {e}");
                                    log::error!("Cash movement error: {e:#}");
                                }
                            }
                        }
                        None => {
                            app.data.movement_error =
                                "⚠️ Banco de dados temporariamente indisponível.".to_string();
                        }
                    }
                }
            }

            ui.add_space(16.0);
            ui.separator();
            section_header(
                ui,
                "💵",
                "Saldo em Dinheiro (gaveta)",
                "Vendas em dinheiro + entradas da gaveta − saídas/perdas da gaveta • banco não altera",
            );

            {
                let (drawer, cash_sales) = match lock_db(&db) {
                    Some(guard) => (guard.cash_in_drawer(), guard.total_cash_sales()),
                    None => {
                        error_banner(ui, "⚠️ Banco de dados temporariamente indisponível.");
                        (0.0, 0.0)
                    }
                };
                let drawer_kpis = [
                    (
                        "Dinheiro em gaveta (total)",
                        fmt_money(drawer),
                        ds::EMERALD,
                    ),
                    ("Vendas em dinheiro", fmt_money(cash_sales), ds::BLUE),
                ];
                kpi_grid(ui, 2, &drawer_kpis);
            }

            ui.add_space(16.0);
            ui.separator();
            section_header(
                ui,
                "🧾",
                "Histórico de Caixa",
                "Entradas, saídas e perdas • busca por ID, descrição ou data",
            );

            {
                let q = app.data.cash_query.clone();
                let day_input = app.data.cash_day.clone();
                match day_input_state(&day_input) {
                    DayInputState::Empty => app.data.cash_day_applied.clear(),
                    DayInputState::Valid(norm) => app.data.cash_day_applied = norm,
                    DayInputState::Partial | DayInputState::Invalid => {}
                }
                let applied = app.data.cash_day_applied.clone();
                let db = Arc::clone(&app.db_inner);
                match lock_db(&db) {
                    Some(guard) => {
                        if applied.is_empty() {
                            app.data.cash_entries = guard.get_historico_caixa(&q);
                        } else {
                            app.data.cash_entries =
                                guard.get_historico_caixa_for_day(&q, &applied);
                        }
                    }
                    None => {
                        error_banner(ui, "⚠️ Banco de dados temporariamente indisponível.");
                    }
                }
            }

            search_bar(ui, "Buscar por ID ou descrição:", &mut app.data.cash_query);
            ui.add_space(8.0);
            day_filter_row(
                ui,
                &mut app.data.cash_day,
                &mut app.data.cash_day_applied,
                &mut app.data.cash_query,
            );
            if matches!(
                day_input_state(&app.data.cash_day),
                DayInputState::Invalid
            ) {
                error_banner(ui, "⚠️ Data inválida. Use o formato AAAA-MM-DD.");
            }
            ui.add_space(8.0);

            cash_table(ui, &app.data.cash_entries);
            ui.add_space(16.0);
            });
        });
}

fn report_summary_cells(
    ui: &mut egui::Ui,
    label: &str,
    value: &str,
    value_color: Color32,
    strong: bool,
) {
    ui.add(egui::Label::new(egui::RichText::new(label).size(13.0).color(ds::MUTED)).wrap());
    let text = if strong {
        egui::RichText::new(value)
            .size(14.0)
            .strong()
            .color(value_color)
    } else {
        egui::RichText::new(value).size(13.0).color(value_color)
    };
    ui.add(egui::Label::new(text).wrap());
}

/// Largura mínima para rótulo e valor lado a lado sem esmagar letras.
/// Abaixo disso, cada resumo vira bloco vertical de 2 linhas
/// (rótulo em cima, valor embaixo). Acima de `SUMMARY_TWO_PAIRS_MIN`,
/// dois pares por linha; entre os dois, um par por linha.
const SUMMARY_SIDE_BY_SIDE_MIN: f32 = 520.0;
const SUMMARY_TWO_PAIRS_MIN: f32 = 760.0;

fn summary_stacked_block(
    ui: &mut egui::Ui,
    label: &str,
    value: &str,
    value_color: Color32,
    strong: bool,
) {
    ui.add(egui::Label::new(egui::RichText::new(label).size(13.0).color(ds::MUTED)).wrap());
    // Ritmo vertical na grade 4pt: 4.0 intra-bloco, 12.0 entre blocos.
    ui.add_space(4.0);
    let text = if strong {
        egui::RichText::new(value)
            .size(14.0)
            .strong()
            .color(value_color)
    } else {
        egui::RichText::new(value).size(13.0).color(value_color)
    };
    ui.add(egui::Label::new(text).wrap());
}

/// Grade de resumo responsiva: bloco vertical de 2 linhas no estreito
/// (< 520px), lado a lado com 1 par/linha até 760px e 2 pares/linha acima.
/// Elimina o esmagamento vertical de letras do `Grid` rígido anterior.
/// Limiares com histerese por `grid_id`: a troca de modo altera a altura
/// (e com ela o scroll vertical), então sem banda morta o modo vibraria.
fn summary_pairs_grid(ui: &mut egui::Ui, grid_id: &str, items: &[(&str, String, Color32, bool)]) {
    if !is_wide_hysteresis(ui, ("summary_side", grid_id), SUMMARY_SIDE_BY_SIDE_MIN) {
        for (i, (label, value, color, strong)) in items.iter().enumerate() {
            summary_stacked_block(ui, label, value, *color, *strong);
            if i + 1 < items.len() {
                ui.add_space(12.0);
            }
        }
        return;
    }
    let pairs_per_row = if is_wide_hysteresis(ui, ("summary_two", grid_id), SUMMARY_TWO_PAIRS_MIN) {
        2
    } else {
        1
    };
    let num_columns = pairs_per_row * 2;
    egui::Grid::new(grid_id)
        .num_columns(num_columns)
        .min_col_width(180.0)
        .spacing(Vec2::new(12.0, 8.0))
        .show(ui, |ui| {
            let mut in_row = 0;
            for (label, value, color, strong) in items {
                report_summary_cells(ui, label, value, *color, *strong);
                in_row += 1;
                if in_row >= pairs_per_row {
                    ui.end_row();
                    in_row = 0;
                }
            }
            if in_row != 0 {
                ui.end_row();
            }
        });
}

#[allow(clippy::too_many_arguments)]
fn consolidado_geral_card(
    ui: &mut egui::Ui,
    total_sales_all: f64,
    total_sales_filtered: f64,
    filter_name: &str,
    cash_in: f64,
    cash_out: f64,
    losses: f64,
    drawer: f64,
    balance: f64,
    balance_color: Color32,
    saldo_devedor: f64,
) {
    ds::form_frame().show(ui, |ui| {
        ui.add(
            egui::Label::new(
                egui::RichText::new("Consolidado geral")
                    .size(14.0)
                    .strong()
                    .color(ds::TITLE),
            )
            .wrap(),
        );
        ui.add_space(8.0);
        // 1 par por linha no estreito, 2 pares por linha no largo — sem linhas vazias.
        let vendas_filtradas_label = format!("Total em Vendas ({filter_name})");
        let items: Vec<(&str, String, Color32, bool)> = vec![
            (
                "Total em Vendas (Todas):",
                fmt_money(total_sales_all),
                ds::BODY,
                false,
            ),
            (
                vendas_filtradas_label.as_str(),
                fmt_money(total_sales_filtered),
                ds::BLUE,
                false,
            ),
            ("Entradas:", fmt_money(cash_in), ds::EMERALD, false),
            ("Saídas:", fmt_money(cash_out), ds::AMBER, false),
            ("Perdas:", fmt_money(losses), ds::CORAL, false),
            ("Saldo em Dinheiro:", fmt_money(drawer), ds::EMERALD, false),
            ("Saldo Atual:", fmt_money(balance), balance_color, true),
            (
                "Saldo Devedor (fiado):",
                fmt_money(saldo_devedor),
                ds::AMBER,
                true,
            ),
        ];
        summary_pairs_grid(ui, "consolidado_grid", &items);
    });
}

#[allow(clippy::too_many_arguments)]
fn report_card(
    ui: &mut egui::Ui,
    icon: &str,
    titulo: &str,
    subtitulo: &str,
    add_contents: impl FnOnce(&mut egui::Ui),
) {
    ds::form_frame().show(ui, |ui| {
        ui.horizontal_wrapped(|ui| {
            ui.spacing_mut().item_spacing = Vec2::new(8.0, 4.0);
            ui.add(egui::Label::new(
                egui::RichText::new(icon).size(14.0).color(ds::TITLE),
            ));
            ui.add(
                egui::Label::new(
                    egui::RichText::new(titulo)
                        .size(14.0)
                        .strong()
                        .color(ds::TITLE),
                )
                .wrap(),
            );
        });
        if !subtitulo.is_empty() {
            ui.add_space(4.0);
            wrap_label(ui, subtitulo, 12.0, ds::MUTED, false);
        }
        ui.add_space(8.0);
        ui.separator();
        ui.add_space(8.0);
        add_contents(ui);
    });
}

fn vendas_pagamento_grid(ui: &mut egui::Ui, grid_id: &str, methods: &[(&str, f64)]) {
    // Mesmo padrão do consolidado: bloco vertical no estreito (< 520px),
    // com histerese por `grid_id` contra vibração na fronteira.
    if !is_wide_hysteresis(ui, ("paggrid_side", grid_id), SUMMARY_SIDE_BY_SIDE_MIN) {
        for (i, (name, total)) in methods.iter().enumerate() {
            summary_stacked_block(ui, name, &fmt_money(*total), ds::BODY, true);
            if i + 1 < methods.len() {
                ui.add_space(12.0);
            }
        }
        return;
    }
    egui::Grid::new(grid_id)
        .num_columns(2)
        .min_col_width(200.0)
        .spacing(Vec2::new(12.0, 8.0))
        .show(ui, |ui| {
            for (name, total) in methods {
                ui.add(
                    egui::Label::new(egui::RichText::new(*name).size(13.0).color(ds::MUTED)).wrap(),
                );
                ui.add(
                    egui::Label::new(
                        egui::RichText::new(fmt_money(*total))
                            .size(13.0)
                            .strong()
                            .color(ds::BODY),
                    )
                    .wrap(),
                );
                ui.end_row();
            }
        });
}

#[allow(clippy::too_many_arguments)]
fn resumo_periodo_grid(
    ui: &mut egui::Ui,
    grid_id: &str,
    vendas_label: String,
    vendas: f64,
    entradas: f64,
    saidas: f64,
    perdas: f64,
    saldo: f64,
    saldo_color: Color32,
) {
    let items: Vec<(&str, String, Color32, bool)> = vec![
        (vendas_label.as_str(), fmt_money(vendas), ds::BODY, false),
        ("Entradas:", fmt_money(entradas), ds::EMERALD, false),
        ("Saídas:", fmt_money(saidas), ds::AMBER, false),
        ("Perdas:", fmt_money(perdas), ds::CORAL, false),
        ("Saldo Atual:", fmt_money(saldo), saldo_color, true),
    ];
    // Reúso da grade responsiva: empilha em 2 linhas abaixo de 520px.
    summary_pairs_grid(ui, grid_id, &items);
}

fn detalhamento_grid_content(
    ui: &mut egui::Ui,
    scroll_salt: &str,
    salt: &str,
    linhas: &[(String, f64, f64, f64, f64, f64)],
    primeira_coluna: &str,
) {
    // Scroller explícito único (horizontal + vertical cap 220px) com salt
    // contextual: a tabela exige ~600px e vive em colunas de ~400px.
    // O `TableBuilder` interno tem `vscroll(false)` e NENHUM `max_scroll_height`
    // para não criar scroll implícito com ID automático (causa do banner vermelho).
    egui::ScrollArea::both()
        .id_salt(scroll_salt)
        .max_height(220.0)
        .auto_shrink([false, false])
        .show(ui, |ui| {
            egui_extras::TableBuilder::new(ui)
                .id_salt(salt)
                .striped(true)
                .vscroll(false)
                // Primeira coluna fixa (rótulos curtos: "10/2025", "2026"):
                // `remainder` aqui realimentava medida↔largura no scroll.
                .column(Column::initial(120.0).clip(true))
                .column(Column::initial(104.0).clip(true))
                .column(Column::initial(104.0).clip(true))
                .column(Column::initial(104.0).clip(true))
                .column(Column::initial(104.0).clip(true))
                .column(Column::initial(104.0).clip(true))
                .header(28.0, |mut h| {
                    h.col(|ui| {
                        ui.add(egui::Label::new(table_header_label(primeira_coluna)).wrap());
                    });
                    h.col(|ui| {
                        ui.add(egui::Label::new(table_header_label("Vendas")).wrap());
                    });
                    h.col(|ui| {
                        ui.add(egui::Label::new(table_header_label("Entradas")).wrap());
                    });
                    h.col(|ui| {
                        ui.add(egui::Label::new(table_header_label("Saídas")).wrap());
                    });
                    h.col(|ui| {
                        ui.add(egui::Label::new(table_header_label("Perdas")).wrap());
                    });
                    h.col(|ui| {
                        ui.add(egui::Label::new(table_header_label("Saldo")).wrap());
                    });
                })
                .body(|mut body| {
                    for (rotulo, s, i, o, l, n) in linhas {
                        let n_color = if *n < 0.0 { ds::CORAL } else { ds::BODY };
                        body.row(36.0, |mut row| {
                            row.col(|ui| {
                                ui.add(egui::Label::new(table_cell(rotulo.clone())).wrap());
                            });
                            row.col(|ui| {
                                ui.add(egui::Label::new(table_cell(fmt_money(*s))).wrap());
                            });
                            row.col(|ui| {
                                ui.add(egui::Label::new(table_cell(fmt_money(*i))).wrap());
                            });
                            row.col(|ui| {
                                ui.add(egui::Label::new(table_cell(fmt_money(*o))).wrap());
                            });
                            row.col(|ui| {
                                ui.add(
                                    egui::Label::new(
                                        egui::RichText::new(fmt_money(*l))
                                            .size(13.0)
                                            .color(ds::CORAL),
                                    )
                                    .wrap(),
                                );
                            });
                            row.col(|ui| {
                                ui.add(
                                    egui::Label::new(
                                        egui::RichText::new(fmt_money(*n))
                                            .size(13.0)
                                            .color(n_color),
                                    )
                                    .wrap(),
                                );
                            });
                        });
                    }
                });
        });
}

fn monthly_revenue_card(ui: &mut egui::Ui, app: &mut App) {
    let db = Arc::clone(&app.db_inner);
    let months: Vec<String> = match lock_db(&db) {
        Some(guard) => guard.distinct_months(),
        None => {
            error_banner(ui, "⚠️ Banco de dados temporariamente indisponível.");
            Vec::new()
        }
    };
    report_card(
        ui,
        "📅",
        "Receita Mensal",
        "Faturamento e saldo líquido mês a mês",
        |ui| {
            if months.is_empty() {
                empty_state_flat(
                    ui,
                    "Sem registros mensais ainda",
                    "Assim que houver vendas, o consolidado mensal aparece aqui.",
                );
                return;
            }
            // Sem snap para o último mês com dados: o padrão (hoje local) e a
            // escolha do usuário sobrevivem ao render; mês sem movimento exibe
            // o estado vazio correspondente.
            let month_label = format_month_br(&app.data.selected_month);
            ui.horizontal_wrapped(|ui| {
                ui.spacing_mut().item_spacing = Vec2::new(8.0, 8.0);
                ui.add(
                    egui::Label::new(egui::RichText::new("Mês:").size(13.0).color(ds::MUTED))
                        .wrap(),
                );
                egui::ComboBox::from_id_salt("month_filter")
                    .selected_text(month_label)
                    .show_ui(ui, |ui| {
                        for m in &months {
                            ui.selectable_value(
                                &mut app.data.selected_month,
                                m.clone(),
                                format_month_br(m),
                            );
                        }
                    });
            });
            ui.add_space(12.0);

            let month = app.data.selected_month.clone();
            let (m_sales, m_in, m_out, m_losses, m_net) = match lock_db(&db) {
                Some(guard) => (
                    guard.total_sales_for_month(&month),
                    guard.total_cash_in_for_month(&month),
                    guard.total_cash_out_for_month(&month),
                    guard.total_losses_for_month(&month),
                    guard.net_balance_for_month(&month),
                ),
                None => {
                    error_banner(ui, "⚠️ Banco de dados temporariamente indisponível.");
                    (0.0, 0.0, 0.0, 0.0, 0.0)
                }
            };

            let m_color = if m_net < 0.0 { ds::CORAL } else { ds::EMERALD };
            resumo_periodo_grid(
                ui,
                "resumo_mes_grid",
                format!("Vendas ({})", format_month_br(&month)),
                m_sales,
                m_in,
                m_out,
                m_losses,
                m_net,
                m_color,
            );
            ui.add_space(12.0);

            let linhas: Vec<(String, f64, f64, f64, f64, f64)> = match lock_db(&db) {
                Some(guard) => months
                    .iter()
                    .map(|m| {
                        (
                            format_month_br(m),
                            guard.total_sales_for_month(m),
                            guard.total_cash_in_for_month(m),
                            guard.total_cash_out_for_month(m),
                            guard.total_losses_for_month(m),
                            guard.net_balance_for_month(m),
                        )
                    })
                    .collect(),
                None => {
                    error_banner(ui, "⚠️ Banco de dados temporariamente indisponível.");
                    Vec::new()
                }
            };
            wrap_label(ui, "Detalhamento por mês:", 12.0, ds::TITLE, true);
            ui.add_space(8.0);
            detalhamento_grid_content(ui, "det_mes_scroll", "det_mes_table", &linhas, "Mês");
        },
    );
}

fn annual_revenue_card(ui: &mut egui::Ui, app: &mut App) {
    let db = Arc::clone(&app.db_inner);
    let years: Vec<String> = match lock_db(&db) {
        Some(guard) => guard.distinct_years(),
        None => {
            error_banner(ui, "⚠️ Banco de dados temporariamente indisponível.");
            Vec::new()
        }
    };
    report_card(
        ui,
        "🗓️",
        "Receita Anual",
        "Consolidado ano a ano com saldo líquido",
        |ui| {
            if years.is_empty() {
                empty_state_flat(
                    ui,
                    "Sem registros anuais ainda",
                    "Assim que houver vendas, o consolidado anual aparece aqui.",
                );
                return;
            }
            // Sem snap para o último ano com dados (mesma razão do mês).
            let year_label = app.data.selected_year.clone();
            ui.horizontal_wrapped(|ui| {
                ui.spacing_mut().item_spacing = Vec2::new(8.0, 8.0);
                ui.add(
                    egui::Label::new(egui::RichText::new("Ano:").size(13.0).color(ds::MUTED))
                        .wrap(),
                );
                egui::ComboBox::from_id_salt("year_filter")
                    .selected_text(year_label)
                    .show_ui(ui, |ui| {
                        for y in &years {
                            ui.selectable_value(&mut app.data.selected_year, y.clone(), y.clone());
                        }
                    });
            });
            ui.add_space(12.0);

            let year = app.data.selected_year.clone();
            let (y_sales, y_in, y_out, y_losses, y_net) = match lock_db(&db) {
                Some(guard) => (
                    guard.total_sales_for_year(&year),
                    guard.total_cash_in_for_year(&year),
                    guard.total_cash_out_for_year(&year),
                    guard.total_losses_for_year(&year),
                    guard.net_balance_for_year(&year),
                ),
                None => {
                    error_banner(ui, "⚠️ Banco de dados temporariamente indisponível.");
                    (0.0, 0.0, 0.0, 0.0, 0.0)
                }
            };

            let y_color = if y_net < 0.0 { ds::CORAL } else { ds::EMERALD };
            resumo_periodo_grid(
                ui,
                "resumo_ano_grid",
                format!("Vendas ({year})"),
                y_sales,
                y_in,
                y_out,
                y_losses,
                y_net,
                y_color,
            );
            ui.add_space(12.0);

            let linhas: Vec<(String, f64, f64, f64, f64, f64)> = match lock_db(&db) {
                Some(guard) => years
                    .iter()
                    .map(|y| {
                        (
                            y.clone(),
                            guard.total_sales_for_year(y),
                            guard.total_cash_in_for_year(y),
                            guard.total_cash_out_for_year(y),
                            guard.total_losses_for_year(y),
                            guard.net_balance_for_year(y),
                        )
                    })
                    .collect(),
                None => {
                    error_banner(ui, "⚠️ Banco de dados temporariamente indisponível.");
                    Vec::new()
                }
            };
            wrap_label(ui, "Detalhamento por ano:", 12.0, ds::TITLE, true);
            ui.add_space(8.0);
            detalhamento_grid_content(ui, "det_ano_scroll", "det_ano_table", &linhas, "Ano");
        },
    );
}

fn loss_history_card(ui: &mut egui::Ui, app: &mut App) {
    report_card(
        ui,
        "🧾",
        "Histórico de Perdas",
        "Perdas registradas com busca por ID, descrição ou data",
        |ui| {
            {
                let q = app.data.loss_query.clone();
                let day_input = app.data.reports_loss_day.clone();
                match day_input_state(&day_input) {
                    DayInputState::Empty => app.data.reports_loss_day_applied.clear(),
                    DayInputState::Valid(norm) => app.data.reports_loss_day_applied = norm,
                    DayInputState::Partial | DayInputState::Invalid => {}
                }
                let applied = app.data.reports_loss_day_applied.clone();
                let db = Arc::clone(&app.db_inner);
                match lock_db(&db) {
                    Some(guard) => {
                        if applied.is_empty() {
                            app.data.losses = guard.get_losses_filtered(&q);
                        } else {
                            app.data.losses = guard.get_losses_filtered_for_day(&q, &applied);
                        }
                    }
                    None => {
                        error_banner(ui, "⚠️ Banco de dados temporariamente indisponível.");
                    }
                }
            }

            // Versões planas (sem Frame próprio): evita card-in-card dentro do `report_card`.
            search_bar_inner(ui, "Buscar por ID ou descrição:", &mut app.data.loss_query);
            ui.add_space(8.0);
            day_filter_row_inner(
                ui,
                &mut app.data.reports_loss_day,
                &mut app.data.reports_loss_day_applied,
                &mut app.data.loss_query,
            );
            if matches!(
                day_input_state(&app.data.reports_loss_day),
                DayInputState::Invalid
            ) {
                error_banner(ui, "⚠️ Data inválida. Use o formato AAAA-MM-DD.");
            }
            ui.add_space(8.0);

            loss_table_inner(
                ui,
                &app.data.losses,
                "loss_history_scroll",
                "loss_history_table",
            );
        },
    );
}

fn executar_exportacao_csv(app: &mut App) {
    let scope = app.data.report_export_scope;
    let day = app.data.export_day.clone();
    let month = app.data.export_month.clone();
    let year = app.data.export_year.clone();
    let filtro_label = translate_payment(&app.data.report_payment_filter).to_string();

    let guard = match try_lock_db(&app.db_inner) {
        Ok(guard) => guard,
        Err(e) => {
            app.data.export_feedback =
                "⚠️ Banco de dados temporariamente indisponível. Tente novamente.".to_string();
            app.data.export_is_error = true;
            log::error!("Export lock failed: {e:#}");
            return;
        }
    };
    let (escopo_display, vendas, movimentacoes, perdas_lista) = match scope {
        ReportExportScope::Diario => (
            format_day_br(&day),
            guard.get_sales_for_day(&day),
            guard.get_cash_entries_for_day(&day),
            guard.get_losses_for_day(&day),
        ),
        ReportExportScope::Mensal => (
            format_month_br(&month),
            guard.get_sales_for_month(&month),
            guard.get_cash_entries_for_month(&month),
            guard.get_losses_for_month(&month),
        ),
        ReportExportScope::Anual => (
            year.clone(),
            guard.get_sales_for_year(&year),
            guard.get_cash_entries_for_year(&year),
            guard.get_losses_for_year(&year),
        ),
        ReportExportScope::Geral => (
            "Geral".to_string(),
            guard.get_sales_filtered(""),
            guard.get_cash_entries_filtered(""),
            guard.get_losses_filtered(""),
        ),
    };
    let (
        total_vendas,
        total_recebido,
        total_entradas,
        total_saidas,
        total_perdas,
        saldo,
        num_vendas,
        ticket,
    ) = match scope {
        ReportExportScope::Diario => (
            guard.total_sales_for_day(&day),
            guard.total_received_for_day(&day),
            guard.total_cash_in_for_day(&day),
            guard.total_cash_out_for_day(&day),
            guard.total_losses_for_day(&day),
            guard.net_balance_for_day(&day),
            guard.count_sales_for_day(&day),
            guard.average_ticket_for_day(&day),
        ),
        ReportExportScope::Mensal => (
            guard.total_sales_for_month(&month),
            guard.total_received_for_month(&month),
            guard.total_cash_in_for_month(&month),
            guard.total_cash_out_for_month(&month),
            guard.total_losses_for_month(&month),
            guard.net_balance_for_month(&month),
            guard.count_sales_for_month(&month),
            guard.average_ticket_for_month(&month),
        ),
        ReportExportScope::Anual => (
            guard.total_sales_for_year(&year),
            guard.total_received_for_year(&year),
            guard.total_cash_in_for_year(&year),
            guard.total_cash_out_for_year(&year),
            guard.total_losses_for_year(&year),
            guard.net_balance_for_year(&year),
            guard.count_sales_for_year(&year),
            guard.average_ticket_for_year(&year),
        ),
        ReportExportScope::Geral => (
            guard.total_sales(),
            guard.total_received(),
            guard.total_cash_in(),
            guard.total_cash_out(),
            guard.total_losses(),
            guard.net_balance_global(),
            guard.count_sales(),
            guard.average_ticket(),
        ),
    };
    let total_devedor_escopo = (total_vendas - total_recebido).max(0.0);
    drop(guard);

    let dados = crate::export::DadosRelatorio {
        escopo_label: &escopo_display,
        filtro_pagamento: &filtro_label,
        total_vendas,
        total_recebido,
        total_devedor: total_devedor_escopo,
        total_entradas,
        total_saidas,
        total_perdas,
        saldo,
        num_vendas,
        ticket_medio: ticket,
        vendas: &vendas,
        movimentacoes: &movimentacoes,
        perdas_lista: &perdas_lista,
    };
    let bytes = match crate::export::gerar_csv_relatorio(&dados) {
        Ok(b) => b,
        Err(e) => {
            app.data.export_feedback = format!("⚠️ Não foi possível gerar o relatório: {e}");
            app.data.export_is_error = true;
            log::error!("Export error: {e}");
            return;
        }
    };

    let sugestao = crate::export::nome_arquivo_sugerido(&escopo_display);
    let caminho = rfd::FileDialog::new()
        .add_filter("CSV (Excel)", &["csv"])
        .set_directory(crate::export::default_export_dir())
        .set_file_name(&sugestao)
        .save_file();
    match caminho {
        None => {
            app.data.export_feedback = "Exportação cancelada.".to_string();
            app.data.export_is_error = false;
        }
        Some(path) => match crate::export::salvar_bytes_em_arquivo(&path, &bytes) {
            Ok(()) => {
                app.data.export_feedback =
                    format!("✅ Relatório exportado para {}", path.display());
                app.data.export_is_error = false;
            }
            Err(e) => {
                app.data.export_feedback = format!("⚠️ Não foi possível exportar: {e}");
                app.data.export_is_error = true;
                log::error!("Export save error: {e}");
            }
        },
    }
}

fn render_export_card(ui: &mut egui::Ui, app: &mut App) {
    ds::card_frame().show(ui, |ui| {
        ui.add(
            egui::Label::new(
                egui::RichText::new("⬇️ Exportar relatório (CSV para Excel)")
                    .size(14.0)
                    .strong()
                    .color(ds::TITLE),
            )
            .wrap(),
        );
        ui.add_space(4.0);
        wrap_label(
            ui,
            "Usa o período próprio abaixo, sem alterar a visualização • delimitador ; com BOM UTF-8",
            12.0,
            ds::MUTED,
            false,
        );
        ui.add_space(8.0);
        ui.horizontal_wrapped(|ui| {
            ui.spacing_mut().item_spacing = Vec2::new(8.0, 8.0);
            ui.add(
                egui::Label::new(egui::RichText::new("Período:").size(13.0).color(ds::MUTED))
                    .wrap(),
            );
            for scope in [
                ReportExportScope::Diario,
                ReportExportScope::Mensal,
                ReportExportScope::Anual,
                ReportExportScope::Geral,
            ] {
                let selected = app.data.report_export_scope == scope;
                if ui.selectable_label(selected, scope.label()).clicked() {
                    app.data.report_export_scope = scope;
                    app.data.export_feedback.clear();
                }
            }
        });
        ui.add_space(8.0);

        let db = Arc::clone(&app.db_inner);
        match app.data.report_export_scope {
            ReportExportScope::Diario => {
                let days: Vec<String> = match lock_db(&db) {
                    Some(guard) => guard.distinct_days(),
                    None => {
                        error_banner(ui, "⚠️ Banco de dados temporariamente indisponível.");
                        Vec::new()
                    }
                };
                // Sem snap para o último dia com dados (ver comentário do dia).
                ui.horizontal_wrapped(|ui| {
                    ui.spacing_mut().item_spacing = Vec2::new(8.0, 8.0);
                    ui.add(
                        egui::Label::new(egui::RichText::new("Dia:").size(13.0).color(ds::MUTED))
                            .wrap(),
                    );
                    if days.is_empty() {
                        ui.add(
                            egui::Label::new(
                                egui::RichText::new("Sem registros ainda.")
                                    .size(13.0)
                                    .color(ds::MUTED),
                            )
                            .wrap(),
                        );
                    } else {
                        let day_label = format_day_br(&app.data.export_day);
                        egui::ComboBox::from_id_salt("export_day")
                            .selected_text(day_label)
                            .show_ui(ui, |ui| {
                                for d in &days {
                                    ui.selectable_value(
                                        &mut app.data.export_day,
                                        d.clone(),
                                        format_day_br(d),
                                    );
                                }
                            });
                    }
                });
            }
            ReportExportScope::Mensal => {
                let months: Vec<String> = match lock_db(&db) {
                    Some(guard) => guard.distinct_months(),
                    None => {
                        error_banner(ui, "⚠️ Banco de dados temporariamente indisponível.");
                        Vec::new()
                    }
                };
                // Sem snap para o último mês com dados (ver comentário do dia).
                ui.horizontal_wrapped(|ui| {
                    ui.spacing_mut().item_spacing = Vec2::new(8.0, 8.0);
                    ui.add(
                        egui::Label::new(egui::RichText::new("Mês:").size(13.0).color(ds::MUTED))
                            .wrap(),
                    );
                    if months.is_empty() {
                        ui.add(
                            egui::Label::new(
                                egui::RichText::new("Sem registros ainda.")
                                    .size(13.0)
                                    .color(ds::MUTED),
                            )
                            .wrap(),
                        );
                    } else {
                        let month_label = format_month_br(&app.data.export_month);
                        egui::ComboBox::from_id_salt("export_month")
                            .selected_text(month_label)
                            .show_ui(ui, |ui| {
                                for m in &months {
                                    ui.selectable_value(
                                        &mut app.data.export_month,
                                        m.clone(),
                                        format_month_br(m),
                                    );
                                }
                            });
                    }
                });
            }
            ReportExportScope::Anual => {
                let years: Vec<String> = match lock_db(&db) {
                    Some(guard) => guard.distinct_years(),
                    None => {
                        error_banner(ui, "⚠️ Banco de dados temporariamente indisponível.");
                        Vec::new()
                    }
                };
                // Sem snap para o último ano com dados (ver comentário do dia).
                ui.horizontal_wrapped(|ui| {
                    ui.spacing_mut().item_spacing = Vec2::new(8.0, 8.0);
                    ui.add(
                        egui::Label::new(egui::RichText::new("Ano:").size(13.0).color(ds::MUTED))
                            .wrap(),
                    );
                    if years.is_empty() {
                        ui.add(
                            egui::Label::new(
                                egui::RichText::new("Sem registros ainda.")
                                    .size(13.0)
                                    .color(ds::MUTED),
                            )
                            .wrap(),
                        );
                    } else {
                        let year_label = app.data.export_year.clone();
                        egui::ComboBox::from_id_salt("export_year")
                            .selected_text(year_label)
                            .show_ui(ui, |ui| {
                                for y in &years {
                                    ui.selectable_value(
                                        &mut app.data.export_year,
                                        y.clone(),
                                        y.clone(),
                                    );
                                }
                            });
                    }
                });
            }
            ReportExportScope::Geral => {
                wrap_label(ui, "Todo o período disponível.", 13.0, ds::MUTED, false);
            }
        }

        ui.add_space(12.0);
        if ui
            .add(
                success_button("📄 Exportar CSV (Excel)")
                    .min_size(Vec2::new(ui.available_width(), 44.0)),
            )
            .clicked()
        {
            executar_exportacao_csv(app);
        }
        if !app.data.export_feedback.is_empty() {
            let msg = app.data.export_feedback.clone();
            if app.data.export_is_error {
                error_banner(ui, &msg);
            } else if msg.starts_with('✅') {
                success_banner(ui, &msg);
            } else {
                ui.add_space(8.0);
                wrap_label(ui, &msg, 12.0, ds::MUTED, false);
                ui.add_space(4.0);
            }
        }
    });
}

/// Filtro case-insensitive da lista agregada, preservando uma linha por
/// sub-modelo: buscar "iphone 12" retorna "iPhone 12", "iPhone 12 Pro" e
/// "iPhone 12 Pro Max" como linhas distintas, nunca somadas.
fn filter_item_sales(items: &[ItemSalesSummary], query: &str) -> Vec<ItemSalesSummary> {
    let q = query.trim().to_lowercase();
    if q.is_empty() {
        return items.to_vec();
    }
    items
        .iter()
        .filter(|i| i.name.to_lowercase().contains(q.as_str()))
        .cloned()
        .collect()
}

/// Aplica um preset de período à reposição: fim = hoje, início inclusivo.
/// Preset 7 => `[hoje-6, hoje]`.
fn apply_replenishment_preset(app: &mut App, days: i64) {
    let hoje = models::hoje_local();
    app.data.replenishment_start = crate::periodo_inicio(&hoje, days);
    app.data.replenishment_end = hoje;
    app.data.replenishment_preset_days = days;
    app.data.replenishment_feedback.clear();
}

fn render_report_subtabs(ui: &mut egui::Ui, app: &mut App) {
    ds::card_frame().show(ui, |ui| {
        ui.horizontal_wrapped(|ui| {
            ui.spacing_mut().item_spacing = Vec2::new(8.0, 8.0);
            for tab in [ReportSubTab::Financeiro, ReportSubTab::Reposicao] {
                let selected = app.data.report_subtab == tab;
                if ui.selectable_label(selected, tab.label()).clicked() {
                    app.data.report_subtab = tab;
                }
            }
        });
    });
}

/// Gera o CSV da lista visível e abre o diálogo de salvamento (`rfd`).
fn executar_exportacao_reposicao(
    app: &mut App,
    items: &[ItemSalesSummary],
    start: &str,
    end: &str,
) {
    let bytes = match crate::export::gerar_csv_item_sales(items, start, end) {
        Ok(b) => b,
        Err(e) => {
            app.data.replenishment_export_feedback =
                format!("⚠️ Não foi possível gerar o CSV: {e}");
            app.data.replenishment_export_is_error = true;
            log::error!("Replenishment export error: {e}");
            return;
        }
    };
    let sugestao = crate::export::nome_arquivo_reposicao(start, end);
    match rfd::FileDialog::new()
        .add_filter("CSV (Excel)", &["csv"])
        .set_directory(crate::export::default_export_dir())
        .set_file_name(&sugestao)
        .save_file()
    {
        None => {
            app.data.replenishment_export_feedback = "Exportação cancelada.".to_string();
            app.data.replenishment_export_is_error = false;
        }
        Some(path) => match crate::export::salvar_bytes_em_arquivo(&path, &bytes) {
            Ok(()) => {
                app.data.replenishment_export_feedback =
                    format!("✅ CSV exportado para {}", path.display());
                app.data.replenishment_export_is_error = false;
            }
            Err(e) => {
                app.data.replenishment_export_feedback =
                    format!("⚠️ Não foi possível exportar: {e}");
                app.data.replenishment_export_is_error = true;
                log::error!("Replenishment save error: {e}");
            }
        },
    }
}

/// Chips de categoria da reposição: Todos + 1 por tipo de atendimento.
/// Filtra via SQL (`sales.sale_type`); compõe com a busca textual.
fn render_replenishment_category_chips(ui: &mut egui::Ui, app: &mut App) {
    const OPTIONS: [(Option<models::SaleType>, &str); 4] = [
        (None, "📦 Todos"),
        (Some(models::SaleType::Aparelho), "📱 Aparelhos"),
        (Some(models::SaleType::Servico), "🔧 Serviços"),
        (Some(models::SaleType::Rapida), "🔌 Acessórios"),
    ];
    ui.horizontal_wrapped(|ui| {
        ui.spacing_mut().item_spacing = Vec2::new(8.0, 8.0);
        ui.add(
            egui::Label::new(
                egui::RichText::new("Categoria:")
                    .size(13.0)
                    .color(ds::MUTED),
            )
            .wrap(),
        );
        for (variant, label) in OPTIONS {
            let selected = app.data.replenishment_category == variant;
            if ui.selectable_label(selected, label).clicked() && !selected {
                app.data.replenishment_category = variant;
            }
        }
    });
}

fn render_replenishment_section(ui: &mut egui::Ui, app: &mut App) {
    // Defaults preguiçosos: cobrem estado vazio sem tocar em digitação.
    if app.data.replenishment_end.trim().is_empty() {
        let hoje = models::hoje_local();
        app.data.replenishment_end = hoje.clone();
        if app.data.replenishment_start.trim().is_empty() {
            let preset = app.data.replenishment_preset_days.max(1);
            app.data.replenishment_start = crate::periodo_inicio(&hoje, preset);
        }
    }
    if app.data.replenishment_start.trim().is_empty() {
        let fim = app.data.replenishment_end.clone();
        let preset = app.data.replenishment_preset_days.max(1);
        app.data.replenishment_start = crate::periodo_inicio(&fim, preset);
    }

    // Leitura única do período + categoria (não-destrutiva); o filtro 🔍
    // textual é aplicado em memória sobre o retorno.
    let start = app.data.replenishment_start.clone();
    let end = app.data.replenishment_end.clone();
    let category = app.data.replenishment_category;
    let aggregated: Vec<ItemSalesSummary> = match lock_db(&app.db_inner) {
        Some(guard) => match guard.get_aggregated_item_sales(&start, &end, category) {
            Ok(rows) => rows,
            Err(e) => {
                log::error!("Replenishment query failed: {e:#}");
                error_banner(ui, "⚠️ Não foi possível agregar os itens do período.");
                Vec::new()
            }
        },
        None => {
            error_banner(ui, "⚠️ Banco de dados temporariamente indisponível.");
            Vec::new()
        }
    };
    let query = app.data.replenishment_query.clone();
    let filtered = filter_item_sales(&aggregated, &query);
    let total_units: i64 = filtered.iter().map(|i| i.total_quantity).sum();
    let total_revenue: f64 = filtered.iter().map(|i| i.total_revenue).sum();
    let range_label = format!("{} a {}", format_day_br(&start), format_day_br(&end));
    let category_label = match category {
        None => "Todas as categorias".to_string(),
        Some(t) => t.label_ptbr().to_string(),
    };
    let table_subtitle = format!("{range_label} • {category_label}");

    report_card(
        ui,
        "📦",
        "Reposição de Estoque / Análise de Itens",
        "Agregado por modelo no período — sub-modelos (Pro, Pro Max, Plus…) nunca se misturam",
        |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.spacing_mut().item_spacing = Vec2::new(8.0, 8.0);
                ui.add(
                    egui::Label::new(egui::RichText::new("Período:").size(13.0).color(ds::MUTED))
                        .wrap(),
                );
                for days in [7, 15, 30, 40] {
                    let selected = app.data.replenishment_preset_days == days as i64;
                    if ui
                        .selectable_label(selected, format!("{days} dias"))
                        .clicked()
                    {
                        apply_replenishment_preset(app, days as i64);
                    }
                }
                if app.data.replenishment_preset_days == 0 {
                    ui.add(
                        egui::Label::new(
                            egui::RichText::new("Personalizado")
                                .size(13.0)
                                .strong()
                                .color(ds::TITLE),
                        )
                        .wrap(),
                    );
                }
            });
            ui.add_space(8.0);
            ui.horizontal_wrapped(|ui| {
                ui.spacing_mut().item_spacing = Vec2::new(8.0, 8.0);
                ui.add(
                    egui::Label::new(
                        egui::RichText::new("Início (AAAA-MM-DD):")
                            .size(13.0)
                            .color(ds::MUTED),
                    )
                    .wrap(),
                );
                let avail = ui.available_width();
                let input_w = (avail.min(140.0)).clamp(100.0, avail.max(100.0));
                ui.add(
                    styled_singleline(&mut app.data.replenishment_start, "AAAA-MM-DD…")
                        .desired_width(input_w),
                );
                ui.add(
                    egui::Label::new(egui::RichText::new("Fim:").size(13.0).color(ds::MUTED))
                        .wrap(),
                );
                ui.add(
                    styled_singleline(&mut app.data.replenishment_end, "AAAA-MM-DD…")
                        .desired_width(input_w),
                );
                if ui.add(ghost_button("✔ Aplicar", 13.0)).clicked() {
                    let s_raw = app.data.replenishment_start.clone();
                    let e_raw = app.data.replenishment_end.clone();
                    match (day_input_state(&s_raw), day_input_state(&e_raw)) {
                        (DayInputState::Valid(s), DayInputState::Valid(e)) if s <= e => {
                            app.data.replenishment_start = s;
                            app.data.replenishment_end = e;
                            app.data.replenishment_preset_days = 0;
                            app.data.replenishment_feedback.clear();
                        }
                        (DayInputState::Valid(_), DayInputState::Valid(_)) => {
                            app.data.replenishment_feedback =
                                "⚠️ Início após o fim — ajuste as datas.".to_string();
                        }
                        _ => {
                            app.data.replenishment_feedback =
                                "⚠️ Datas inválidas. Use AAAA-MM-DD (ex.: 2026-10-01).".to_string();
                        }
                    }
                }
            });
            if !app.data.replenishment_feedback.is_empty() {
                let msg = app.data.replenishment_feedback.clone();
                warning_line(ui, msg, ds::AMBER);
            }
            ui.add_space(8.0);
            render_replenishment_category_chips(ui, app);
            ui.add_space(8.0);
            ui.separator();
            ui.add_space(8.0);
            // Linha única de busca (campo + limpar lado a lado), sem
            // `Frame` próprio: zero card-in-card.
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing = Vec2::new(8.0, 8.0);
                ui.add(egui::Label::new(
                    egui::RichText::new("🔍").size(14.0).color(ds::MUTED),
                ));
                let avail = ui.available_width();
                let btn_w = 96.0;
                let input_w = (avail - btn_w - 8.0).clamp(140.0, avail.max(140.0));
                ui.add(
                    styled_singleline(
                        &mut app.data.replenishment_query,
                        "Filtrar item (ex: \"A15\", \"iPhone 12\", \"capinha\")…",
                    )
                    .desired_width(input_w),
                );
                if ui
                    .add(ghost_button("✖ Limpar", 13.0).min_size(Vec2::new(btn_w, 40.0)))
                    .clicked()
                {
                    app.data.replenishment_query.clear();
                }
            });
        },
    );

    ui.add_space(12.0);

    // KPIs no nível da página (nunca aninhados em outro card).
    let kpis = [
        ("📦 Itens Únicos", filtered.len().to_string(), ds::BLUE),
        ("📊 Unidades Vendidas", total_units.to_string(), ds::EMERALD),
        (
            "💰 Faturamento Total",
            crate::export::fmt_reais_br(total_revenue),
            ds::EMERALD,
        ),
    ];
    kpi_grid(ui, 3, &kpis);

    ui.add_space(12.0);

    report_card(ui, "🏆", "Itens mais vendidos", &table_subtitle, |ui| {
        if filtered.is_empty() {
            empty_state_flat(
                ui,
                "Nenhum item vendido no período",
                "Ajuste as datas ou o filtro — cada venda com item aparece aqui.",
            );
        } else {
            egui::ScrollArea::both()
                .id_salt("replenishment_table_scroll")
                .min_scrolled_height(380.0)
                .max_height(480.0)
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    egui_extras::TableBuilder::new(ui)
                        .id_salt("replenishment_table")
                        .striped(true)
                        .vscroll(false)
                        .column(Column::initial(220.0).clip(true))
                        .column(Column::initial(120.0).clip(true))
                        .column(Column::initial(150.0).clip(true))
                        .header(28.0, |mut h| {
                            h.col(|ui| {
                                ui.add(egui::Label::new(table_header_label("Produto / Item")));
                            });
                            h.col(|ui| {
                                ui.add(egui::Label::new(table_header_label("Qtd Vendida (Unid)")));
                            });
                            h.col(|ui| {
                                ui.add(egui::Label::new(table_header_label(
                                    "Faturamento Total (R$)",
                                )));
                            });
                        })
                        .body(|mut body| {
                            for item in &filtered {
                                body.row(28.0, |mut row| {
                                    row.col(|ui| {
                                        ui.add(egui::Label::new(table_cell(item.name.clone())));
                                    });
                                    row.col(|ui| {
                                        ui.add(egui::Label::new(table_cell(
                                            item.total_quantity.to_string(),
                                        )));
                                    });
                                    row.col(|ui| {
                                        ui.add(egui::Label::new(table_cell(
                                            crate::export::fmt_reais_br(item.total_revenue),
                                        )));
                                    });
                                });
                            }
                        });
                });
        }
        ui.add_space(8.0);
        ui.horizontal_wrapped(|ui| {
            ui.spacing_mut().item_spacing = Vec2::new(8.0, 8.0);
            if ui.add(success_button("⬇️ Exportar CSV (Excel)")).clicked() {
                let items = filtered.clone();
                let s = start.clone();
                let e = end.clone();
                executar_exportacao_reposicao(app, &items, &s, &e);
            }
            wrap_label(
                ui,
                "Exporta a lista visível (respeita o filtro) • `;` com BOM UTF-8",
                12.0,
                ds::MUTED,
                false,
            );
        });
        if !app.data.replenishment_export_feedback.is_empty() {
            let msg = app.data.replenishment_export_feedback.clone();
            let is_err = app.data.replenishment_export_is_error;
            if is_err {
                error_banner(ui, &msg);
            } else {
                success_banner(ui, &msg);
            }
        }
    });
}

fn render_reports(ui: &mut egui::Ui, app: &mut App) {
    egui::ScrollArea::vertical()
        .id_salt("reports_scroll")
        .auto_shrink([false; 2])
        .show(ui, |ui| {
            content_column(ui, |ui| {
                page_header(
                    ui,
                    "📈",
                    "Relatórios Financeiros",
                    "Consolidado geral, mensal e anual do negócio",
                );
                ui.add_space(4.0);

                render_report_subtabs(ui, app);
                ui.add_space(12.0);

                if matches!(app.data.report_subtab, ReportSubTab::Reposicao) {
                    render_replenishment_section(ui, app);
                    ui.add_space(16.0);
                    return;
                }

                let db = Arc::clone(&app.db_inner);

                ds::card_frame().show(ui, |ui| {
                    ui.horizontal_wrapped(|ui| {
                        ui.spacing_mut().item_spacing = Vec2::new(8.0, 8.0);
                        ui.add(
                            egui::Label::new(
                                egui::RichText::new("Filtrar por Pagamento:")
                                    .size(13.0)
                                    .color(ds::MUTED),
                            )
                            .wrap(),
                        );
                        let filter_label =
                            translate_payment(&app.data.report_payment_filter).to_string();
                        egui::ComboBox::from_id_salt("report_filter")
                            .selected_text(filter_label)
                            .show_ui(ui, |ui| {
                                ui.selectable_value(
                                    &mut app.data.report_payment_filter,
                                    models::PaymentMethod::Cash,
                                    "Dinheiro",
                                );
                                ui.selectable_value(
                                    &mut app.data.report_payment_filter,
                                    models::PaymentMethod::Card,
                                    "Cartão",
                                );
                                ui.selectable_value(
                                    &mut app.data.report_payment_filter,
                                    models::PaymentMethod::Pix,
                                    "Pix",
                                );
                                ui.selectable_value(
                                    &mut app.data.report_payment_filter,
                                    models::PaymentMethod::Fiado,
                                    "Fiado",
                                );
                            });
                    });
                });
                ui.add_space(12.0);

                let filter = app.data.report_payment_filter.clone();
                let (
                    total_sales_all,
                    total_sales_filtered,
                    cash_in,
                    cash_out,
                    losses,
                    balance,
                    drawer,
                    saldo_devedor,
                ) = match lock_db(&db) {
                    Some(db_guard) => {
                        let total_sales_all = db_guard.total_sales();
                        let total_sales_filtered = db_guard.total_sales_by_method(&filter);
                        let cash_in = db_guard.total_cash_in();
                        let cash_out = db_guard.total_cash_out();
                        let losses = db_guard.total_losses();
                        let balance = db_guard.net_balance_global();
                        let drawer = db_guard.cash_in_drawer();
                        let saldo_devedor = db_guard.total_receivables_global();
                        (
                            total_sales_all,
                            total_sales_filtered,
                            cash_in,
                            cash_out,
                            losses,
                            balance,
                            drawer,
                            saldo_devedor,
                        )
                    }
                    None => {
                        error_banner(ui, "⚠️ Banco de dados temporariamente indisponível.");
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0)
                    }
                };

                let balance_color = if balance < 0.0 {
                    ds::CORAL
                } else {
                    ds::EMERALD
                };
                let filter_name = translate_payment(&filter).to_string();
                // Largura real do container: abaixo de BP_WIDE (860) cada cartão
                // ocupa a largura total (empilhado); lado a lado só com folga genuína.
                if ui.available_width() < BP_WIDE {
                    ui.vertical(|ui| {
                        consolidado_geral_card(
                            ui,
                            total_sales_all,
                            total_sales_filtered,
                            &filter_name,
                            cash_in,
                            cash_out,
                            losses,
                            drawer,
                            balance,
                            balance_color,
                            saldo_devedor,
                        );
                        ui.add_space(12.0);
                        render_export_card(ui, app);
                    });
                } else {
                    // `push_id`: as colunas-filhas de `columns()` não têm salt
                    // próprio; sem namespacing, os dois blocos `columns(2)` desta
                    // rota gerariam UIs-filhas com auto-IDs ambíguos (🔥 ID 13100).
                    ui.push_id("reports_top_cols", |ui| {
                        ui.columns(2, |cols| {
                            consolidado_geral_card(
                                &mut cols[0],
                                total_sales_all,
                                total_sales_filtered,
                                &filter_name,
                                cash_in,
                                cash_out,
                                losses,
                                drawer,
                                balance,
                                balance_color,
                                saldo_devedor,
                            );
                            render_export_card(&mut cols[1], app);
                        });
                    });
                }

                ui.add_space(12.0);

                {
                    let db = Arc::clone(&app.db_inner);
                    let methods = match lock_db(&db) {
                        Some(guard) => vec![
                            (
                                "Dinheiro",
                                guard.total_sales_by_method(&models::PaymentMethod::Cash),
                            ),
                            (
                                "Cartão",
                                guard.total_sales_by_method(&models::PaymentMethod::Card),
                            ),
                            (
                                "Pix",
                                guard.total_sales_by_method(&models::PaymentMethod::Pix),
                            ),
                            (
                                "Fiado",
                                guard.total_sales_by_method(&models::PaymentMethod::Fiado),
                            ),
                        ],
                        None => {
                            error_banner(ui, "⚠️ Banco de dados temporariamente indisponível.");
                            vec![
                                ("Dinheiro", 0.0),
                                ("Cartão", 0.0),
                                ("Pix", 0.0),
                                ("Fiado", 0.0),
                            ]
                        }
                    };
                    report_card(
                        ui,
                        "💳",
                        "Vendas por Pagamento",
                        "Receita total por meio de pagamento",
                        |ui| {
                            vendas_pagamento_grid(ui, "vendas_pagamento_grid", &methods);
                        },
                    );
                }

                ui.add_space(12.0);

                // Mesma regra do bloco acima: empilha em largura total abaixo de BP_WIDE.
                if ui.available_width() < BP_WIDE {
                    ui.vertical(|ui| {
                        monthly_revenue_card(ui, app);
                        ui.add_space(12.0);
                        annual_revenue_card(ui, app);
                    });
                } else {
                    // Mesmo namespacing do bloco acima: mensal × anual lado a
                    // lado, cada um com `ScrollArea::both` + `TableBuilder`.
                    ui.push_id("reports_rev_cols", |ui| {
                        ui.columns(2, |cols| {
                            monthly_revenue_card(&mut cols[0], app);
                            annual_revenue_card(&mut cols[1], app);
                        });
                    });
                }

                ui.add_space(12.0);

                loss_history_card(ui, app);
                ui.add_space(16.0);
            });
        });
}
