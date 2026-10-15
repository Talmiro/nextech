use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sale {
    pub id: i64,
    pub name: String,
    pub quantity: i64,
    pub unit_price: f64,
    pub total_price: f64,
    pub payment_method: PaymentMethod,
    pub paid_amount: f64,
    pub client_name: String,
    pub phone: String,
    pub imei: String,
    pub sale_type: SaleType,
    pub status: SaleStatus,
    pub created_at: String,
    /// Linhas da venda. Vendas legadas (gravadas antes de `sale_items`)
    /// chegam aqui com 1 item sintetizado dos campos de resumo acima.
    pub items: Vec<SaleItem>,
    /// Nº de linhas (`COUNT(sale_items)`): permite exibir o resumo
    /// multi-item nas listas sem anexar os itens (sem N+1).
    pub item_count: i64,
}

impl Sale {
    /// Valor ainda em aberto (`total - pago`), nunca negativo (tolerância de float).
    pub fn remaining(&self) -> f64 {
        let r = self.total_price - self.paid_amount;
        if r < 0.0 && r > -1e-9 {
            0.0
        } else {
            r.max(0.0)
        }
    }

    pub fn is_debt(&self) -> bool {
        !matches!(self.status, SaleStatus::Quitado)
    }
}

/// Tipo de atendimento do PDV (assistência técnica).
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq)]
pub enum SaleType {
    /// Venda rápida de balcão (acessórios). Sem cliente obrigatório.
    #[default]
    Rapida,
    /// Serviço/reparo. Exige nome do cliente (garantia).
    Servico,
    /// Venda de aparelho. Exige nome do cliente + IMEI/série (garantia).
    Aparelho,
}

impl SaleType {
    pub fn as_str(&self) -> &'static str {
        match self {
            SaleType::Rapida => "rapida",
            SaleType::Servico => "servico",
            SaleType::Aparelho => "aparelho",
        }
    }

    pub fn label_ptbr(&self) -> &'static str {
        match self {
            SaleType::Rapida => "Venda Rápida",
            SaleType::Servico => "Serviço",
            SaleType::Aparelho => "Aparelho",
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "rapida" => Some(SaleType::Rapida),
            "servico" => Some(SaleType::Servico),
            "aparelho" => Some(SaleType::Aparelho),
            _ => None,
        }
    }

    /// Nome do cliente é obrigatório (garantia) neste tipo.
    pub fn requires_client(&self) -> bool {
        matches!(self, SaleType::Servico | SaleType::Aparelho)
    }

    /// IMEI/série é obrigatório (rastreio de garantia) neste tipo.
    pub fn requires_imei(&self) -> bool {
        matches!(self, SaleType::Aparelho)
    }
}

/// Situação da venda no crediário ("fiado").
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq)]
pub enum SaleStatus {
    #[default]
    Quitado,
    Parcial,
    Pendente,
}

impl SaleStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            SaleStatus::Quitado => "quitado",
            SaleStatus::Parcial => "parcial",
            SaleStatus::Pendente => "pendente",
        }
    }

    pub fn label_ptbr(&self) -> &'static str {
        match self {
            SaleStatus::Quitado => "Quitado",
            SaleStatus::Parcial => "Parcial",
            SaleStatus::Pendente => "Pendente",
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "quitado" => Some(SaleStatus::Quitado),
            "parcial" => Some(SaleStatus::Parcial),
            "pendente" => Some(SaleStatus::Pendente),
            _ => None,
        }
    }
}

/// Deriva o status a partir de total x pago (tolerância de centavo).
pub fn status_for(total: f64, paid: f64) -> SaleStatus {
    const EPS: f64 = 1e-9;
    if paid <= EPS {
        if total <= EPS {
            SaleStatus::Quitado
        } else {
            SaleStatus::Pendente
        }
    } else if paid + 1e-6 < total {
        SaleStatus::Parcial
    } else {
        SaleStatus::Quitado
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub enum PaymentMethod {
    #[default]
    Cash,
    Card,
    Pix,
    Fiado,
}

impl PaymentMethod {
    pub fn as_str(&self) -> &'static str {
        match self {
            PaymentMethod::Cash => "Cash",
            PaymentMethod::Card => "Card",
            PaymentMethod::Pix => "Pix",
            // Mantido exclusivamente para leitura de bancos legados;
            // nenhuma venda nova deve ser gravada com este meio.
            PaymentMethod::Fiado => "Fiado",
        }
    }
    /// Rótulo PT-BR para exibição e logs (`cash_entries.description`).
    pub fn label_ptbr(&self) -> &'static str {
        match self {
            PaymentMethod::Cash => "Dinheiro",
            PaymentMethod::Card => "Cartão",
            PaymentMethod::Pix => "Pix",
            PaymentMethod::Fiado => "Fiado",
        }
    }
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "Cash" => Some(PaymentMethod::Cash),
            "Card" => Some(PaymentMethod::Card),
            "Pix" => Some(PaymentMethod::Pix),
            "Fiado" => Some(PaymentMethod::Fiado),
            _ => None,
        }
    }
    pub fn all() -> Vec<PaymentMethod> {
        vec![
            PaymentMethod::Cash,
            PaymentMethod::Card,
            PaymentMethod::Pix,
            PaymentMethod::Fiado,
        ]
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub enum CashSource {
    #[default]
    Drawer,
    Bank,
}

impl CashSource {
    pub fn as_str(&self) -> &'static str {
        match self {
            CashSource::Drawer => "drawer",
            CashSource::Bank => "bank",
        }
    }
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "drawer" => Some(CashSource::Drawer),
            "bank" => Some(CashSource::Bank),
            _ => None,
        }
    }
    pub fn short_label(&self) -> &'static str {
        match self {
            CashSource::Drawer => "💵 Gaveta",
            CashSource::Bank => "🏦 Banco",
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum MovementKind {
    #[default]
    Nenhum,
    Entrada,
    Saida,
    Perda,
}

impl MovementKind {
    pub fn label(&self) -> &'static str {
        match self {
            MovementKind::Nenhum => "Nenhum",
            MovementKind::Entrada => "➕ Entrada",
            MovementKind::Saida => "➖ Saída / Despesa",
            MovementKind::Perda => "⚠️ Perda / Prejuízo",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CashEntry {
    pub id: i64,
    pub amount: f64,
    pub is_entry: bool,
    pub source: CashSource,
    pub description: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Loss {
    pub id: i64,
    pub amount: f64,
    pub source: CashSource,
    pub description: String,
    pub created_at: String,
}

/// Tipo de linha exibida no Histórico de Caixa unificado.
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq)]
pub enum TipoMovimentoCaixa {
    #[default]
    Entrada,
    Saida,
    Perda,
}

impl TipoMovimentoCaixa {
    pub fn label(&self) -> &'static str {
        match self {
            TipoMovimentoCaixa::Entrada => "Entrada",
            TipoMovimentoCaixa::Saida => "Saída",
            TipoMovimentoCaixa::Perda => "Perda",
        }
    }
}

/// Linha do Histórico de Caixa unificado (entradas, saídas e perdas).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MovimentoCaixa {
    pub id: i64,
    pub amount: f64,
    pub tipo: TipoMovimentoCaixa,
    pub source: CashSource,
    pub description: String,
    pub created_at: String,
}

/// Linha de uma venda multi-item.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaleItem {
    pub id: i64,
    pub sale_id: i64,
    pub name: String,
    pub quantity: i64,
    pub unit_price: f64,
    pub total_price: f64,
    pub imei: String,
}

/// Nome-resumo exibido em tabelas e no CSV: só o primeiro item quando a
/// venda tem 1 linha, ou `"<Primeiro> (+N itens)"` quando multi-item.
#[must_use]
pub fn summary_name(first_name: &str, extra_count: usize) -> String {
    let first = first_name.trim();
    if extra_count == 0 {
        return first.to_string();
    }
    if extra_count == 1 {
        return format!("{first} (+1 item)");
    }
    format!("{first} (+{extra_count} itens)")
}

/// Texto da coluna "Unitário": média de `unit_price` seria enganosa no
/// multi-item (ex.: 1000 + 350 → 675), então exibe `—` quando há 2+ linhas.
#[must_use]
pub fn unit_price_display(sale: &Sale) -> String {
    if sale.item_count > 1 {
        "—".to_string()
    } else {
        format!("{:.2}", sale.unit_price)
    }
}

/// Rótulo contextual do campo/coluna de item por tipo de atendimento.
#[must_use]
pub fn item_label(sale_type: SaleType) -> &'static str {
    match sale_type {
        SaleType::Rapida => "Produto / Acessório",
        SaleType::Servico => "Serviço Executado",
        SaleType::Aparelho => "Modelo do Aparelho",
    }
}

/// Dica (placeholder) contextual do campo de item por tipo.
#[must_use]
pub fn item_hint(sale_type: SaleType) -> &'static str {
    match sale_type {
        SaleType::Rapida => "Ex.: Capinha, Película...",
        SaleType::Servico => "Ex.: Troca de tela, Bateria...",
        SaleType::Aparelho => "Ex.: Samsung A15 128GB, iPhone 11...",
    }
}

#[derive(Debug, Clone, Default)]
pub struct NewSale {
    pub items: Vec<NewSaleItem>,
    pub payment_method: Option<PaymentMethod>,
    pub client_name: String,
    pub phone: String,
    pub sale_type: SaleType,
    pub paid_amount: String,
    pub is_fiado: bool,
}

/// Linha do rascunho em `render_pos` (strings de formulário).
#[derive(Debug, Clone, Default)]
pub struct NewSaleItem {
    pub name: String,
    pub quantity: String,
    pub unit_price: String,
    pub imei: String,
}

#[derive(Debug, Clone, Default)]
pub struct NewCashEntry {
    pub amount: String,
    pub description: String,
    pub is_entry: bool,
}

#[derive(Debug, Clone, Default)]
pub struct NewLoss {
    pub amount: String,
    pub description: String,
}

/// Data/hora de referência do aplicativo: sempre o relógio LOCAL da máquina.
///
/// Centraliza todos os pontos de geração (inicialização de filtros, carimbos
/// de persistência, botão "Hoje") para que o dia padrão nunca reflita UTC.
/// Usa `date_naive()` para que apenas o dia civil local importe.
pub fn hoje_local() -> String {
    chrono::Local::now()
        .date_naive()
        .format("%Y-%m-%d")
        .to_string()
}

/// Carimbo local de persistência (`AAAA-MM-DD HH:MM:SS`) para `created_at`.
pub fn agora_local() -> String {
    chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn venda_resumo(unit: f64, count: i64) -> Sale {
        Sale {
            id: 1,
            name: "Resumo".to_string(),
            quantity: 1,
            unit_price: unit,
            total_price: unit,
            payment_method: PaymentMethod::Cash,
            paid_amount: unit,
            client_name: String::new(),
            phone: String::new(),
            imei: String::new(),
            sale_type: SaleType::Rapida,
            status: SaleStatus::Quitado,
            created_at: "2026-10-14 10:30:00".to_string(),
            items: Vec::new(),
            item_count: count,
        }
    }

    #[test]
    fn unitario_exato_no_item_unico() {
        assert_eq!(unit_price_display(&venda_resumo(149.9, 1)), "149.90");
        assert_eq!(unit_price_display(&venda_resumo(250.0, 0)), "250.00");
    }

    #[test]
    fn unitario_vira_traco_no_multi_item() {
        assert_eq!(unit_price_display(&venda_resumo(675.0, 2)), "—");
        assert_eq!(unit_price_display(&venda_resumo(100.0, 5)), "—");
    }

    #[test]
    fn rotulo_e_dica_por_tipo() {
        assert_eq!(item_label(SaleType::Rapida), "Produto / Acessório");
        assert_eq!(item_label(SaleType::Servico), "Serviço Executado");
        assert_eq!(item_label(SaleType::Aparelho), "Modelo do Aparelho");
        assert_eq!(item_hint(SaleType::Rapida), "Ex.: Capinha, Película...");
        assert_eq!(
            item_hint(SaleType::Servico),
            "Ex.: Troca de tela, Bateria..."
        );
        assert_eq!(
            item_hint(SaleType::Aparelho),
            "Ex.: Samsung A15 128GB, iPhone 11..."
        );
    }
}
