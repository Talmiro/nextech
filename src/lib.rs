use eframe::egui;
use std::sync::{Arc, Mutex};

pub mod db;
pub mod export;
pub mod models;
pub mod receipt;
pub mod ui;

pub struct App {
    pub db_inner: Arc<Mutex<db::Database>>,
    pub current_route: ui::AppRoute,
    pub data: AppData,
    pub toasts: egui_toast::Toasts,
    last_seen_day: String,
}

pub struct AppData {
    pub sales: Vec<models::Sale>,
    pub cash_entries: Vec<models::MovimentoCaixa>,
    pub losses: Vec<models::Loss>,
    pub new_sale: models::NewSale,
    pub new_cash_entry: models::NewCashEntry,
    pub new_loss: models::NewLoss,
    pub sales_query: String,
    pub cash_query: String,
    pub loss_query: String,
    pub selected_month: String,
    pub selected_year: String,
    pub selected_day: String,
    pub dashboard_period: ui::DashboardPeriod,
    pub movement_kind: models::MovementKind,
    pub movement_error: String,
    pub sale_error: String,
    pub pos_day: String,
    pub cash_day: String,
    pub pos_day_applied: String,
    pub cash_day_applied: String,
    pub reports_loss_day: String,
    pub reports_loss_day_applied: String,
    pub report_export_scope: ui::ReportExportScope,
    pub report_subtab: ui::ReportSubTab,
    pub replenishment_preset_days: i64,
    pub replenishment_start: String,
    pub replenishment_end: String,
    pub replenishment_query: String,
    pub replenishment_category: Option<models::SaleType>,
    pub replenishment_feedback: String,
    pub replenishment_export_feedback: String,
    pub replenishment_export_is_error: bool,
    pub export_day: String,
    pub export_month: String,
    pub export_year: String,
    pub export_feedback: String,
    pub export_is_error: bool,
    pub fiado_query: String,
    pub fiado_status_idx: usize,
    pub fiado_selected: Option<i64>,
    pub fiado_abate: String,
    pub fiado_feedback: String,
    pub fiado_is_error: bool,
    pub last_sale: Option<models::Sale>,
    pub last_sale_fp: String,
    pub pending_delete: Option<ui::PendingDelete>,
}

impl AppData {
    pub fn new() -> Self {
        Self {
            sales: Vec::new(),
            cash_entries: Vec::new(),
            losses: Vec::new(),
            new_sale: models::NewSale::default(),
            new_cash_entry: models::NewCashEntry::default(),
            new_loss: models::NewLoss::default(),
            sales_query: String::new(),
            cash_query: String::new(),
            loss_query: String::new(),
            selected_month: models::hoje_local()[..7].to_string(),
            selected_year: models::hoje_local()[..4].to_string(),
            selected_day: models::hoje_local(),
            dashboard_period: ui::DashboardPeriod::default(),
            movement_kind: models::MovementKind::default(),
            movement_error: String::new(),
            sale_error: String::new(),
            pos_day: models::hoje_local(),
            cash_day: models::hoje_local(),
            pos_day_applied: models::hoje_local(),
            cash_day_applied: models::hoje_local(),
            reports_loss_day: models::hoje_local(),
            reports_loss_day_applied: models::hoje_local(),
            report_export_scope: ui::ReportExportScope::default(),
            report_subtab: ui::ReportSubTab::default(),
            replenishment_preset_days: 30,
            replenishment_start: {
                let hoje = models::hoje_local();
                periodo_inicio(&hoje, 30)
            },
            replenishment_end: models::hoje_local(),
            replenishment_query: String::new(),
            replenishment_category: None,
            replenishment_feedback: String::new(),
            replenishment_export_feedback: String::new(),
            replenishment_export_is_error: false,
            export_day: models::hoje_local(),
            export_month: models::hoje_local()[..7].to_string(),
            export_year: models::hoje_local()[..4].to_string(),
            export_feedback: String::new(),
            export_is_error: false,
            fiado_query: String::new(),
            fiado_status_idx: 0,
            fiado_selected: None,
            fiado_abate: String::new(),
            fiado_feedback: String::new(),
            fiado_is_error: false,
            last_sale: None,
            last_sale_fp: String::new(),
            pending_delete: None,
        }
    }

    /// Rollover à meia-noite: se o dia civil local virou enquanto o app
    /// estava aberto, atualiza apenas os filtros que ainda contêm o valor
    /// padrão anterior — datas digitadas pelo usuário nunca são alteradas.
    pub fn refresh_for_new_day(&mut self, previous_today: &str) {
        let today = models::hoje_local();
        if today == previous_today {
            return;
        }
        let month_now = today[..7].to_string();
        let month_prev = previous_today.get(..7).unwrap_or_default().to_string();
        let year_now = today[..4].to_string();
        let year_prev = previous_today.get(..4).unwrap_or_default().to_string();
        for field in [
            &mut self.selected_day,
            &mut self.pos_day,
            &mut self.cash_day,
            &mut self.pos_day_applied,
            &mut self.cash_day_applied,
            &mut self.reports_loss_day,
            &mut self.reports_loss_day_applied,
            &mut self.export_day,
        ] {
            if field.as_str() == previous_today {
                *field = today.clone();
            }
        }
        for field in [&mut self.selected_month, &mut self.export_month] {
            if field.as_str() == month_prev {
                *field = month_now.clone();
            }
        }
        for field in [&mut self.selected_year, &mut self.export_year] {
            if field.as_str() == year_prev {
                *field = year_now.clone();
            }
        }
        // Reposição: se o fim ainda era "ontem", avança para hoje; com
        // preset ativo (7/15/30/40) o início acompanha, no custom o
        // usuário digitou e nada muda.
        if self.replenishment_end.as_str() == previous_today {
            self.replenishment_end = today.clone();
            if self.replenishment_preset_days > 0 {
                self.replenishment_start = periodo_inicio(&today, self.replenishment_preset_days);
            }
        }
    }
}

/// Início inclusivo de uma janela de `days` terminando em `end`
/// (`AAAA-MM-DD`). Preset 7 => `[hoje-6, hoje]`. Fora do padrão,
/// devolve o próprio `end` sem pânico.
pub(crate) fn periodo_inicio(end: &str, days: i64) -> String {
    let span = days.max(1) - 1;
    match chrono::NaiveDate::parse_from_str(end, "%Y-%m-%d") {
        Ok(d) => (d - chrono::Days::new(span.max(0) as u64))
            .format("%Y-%m-%d")
            .to_string(),
        Err(_) => end.to_string(),
    }
}

impl Default for AppData {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let database = match db::Database::new().and_then(|mut db| {
            db.init()?;
            Ok(db)
        }) {
            Ok(db) => db,
            Err(e) => {
                log::error!(
                    "Fatal database error at {}: {e:#}",
                    db::database_path().display()
                );
                // Degradação graciosa: abre em memória para a UI ao menos
                // exibir o erro em vez de fechar com panic (sem console no
                // Windows release o operador não veria nada).
                let mut fallback =
                    db::Database::open_in_memory().expect("in-memory fallback must open");
                fallback.init().expect("in-memory fallback must initialize");
                fallback
            }
        };
        Self {
            db_inner: Arc::new(Mutex::new(database)),
            current_route: ui::AppRoute::Dashboard,
            data: AppData::new(),
            toasts: egui_toast::Toasts::new()
                .anchor(egui::Align2::RIGHT_BOTTOM, (-16.0, -16.0))
                .direction(egui::Direction::BottomUp),
            last_seen_day: models::hoje_local(),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Rollover à meia-noite sem tocar em datas digitadas pelo usuário.
        let today = models::hoje_local();
        if today != self.last_seen_day {
            let previous = std::mem::replace(&mut self.last_seen_day, today);
            self.data.refresh_for_new_day(&previous);
        }
        let sales_q = self.data.sales_query.clone();
        let cash_q = self.data.cash_query.clone();
        let loss_q = self.data.loss_query.clone();
        self.data.sales = {
            let db = self.db_inner.lock().unwrap();
            db.get_sales_filtered(&sales_q)
        };
        self.data.cash_entries = {
            let db = self.db_inner.lock().unwrap();
            db.get_historico_caixa(&cash_q)
        };
        self.data.losses = {
            let db = self.db_inner.lock().unwrap();
            db.get_losses_filtered(&loss_q)
        };
        ui::render_app(ctx, self);
    }
}
