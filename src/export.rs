//! Geração de relatórios CSV compatíveis com Excel PT-BR.
//!
//! Usa delimitador `;`, BOM UTF-8 e decimal com vírgula para que o
//! Excel em português abra o arquivo sem ajustes manuais.

use anyhow::{Context, Result};
use std::path::{Path, PathBuf};

use crate::db::ItemSalesSummary;
use crate::models::{CashEntry, Loss, Sale};

/// Dados necessários para gerar o CSV consolidado do período.
///
/// Relatórios são revenue-only: `Faturamento (Total Entradas)`, `Saídas`,
/// `Perdas` e `Saldo`. Sem cálculo de margem. Meio de pagamento removido.
pub struct DadosRelatorio<'a> {
    /// Rótulo do período exibido no cabeçalho (ex.: "10/2025", "2025", "10/10/2025", "Geral").
    pub escopo_label: &'a str,
    /// LEGADO ignorado na geração (mantido para a UI antiga compilar
    /// até a fase Egui; será excluído com o filtro de pagamento).
    pub filtro_pagamento: &'a str,
    pub total_vendas: f64,
    pub total_recebido: f64,
    pub total_devedor: f64,
    pub total_entradas: f64,
    pub total_saidas: f64,
    pub total_perdas: f64,
    pub saldo: f64,
    pub num_vendas: i64,
    pub ticket_medio: f64,
    pub vendas: &'a [Sale],
    pub movimentacoes: &'a [CashEntry],
    pub perdas_lista: &'a [Loss],
}

/// Formata valor monetário no padrão BR (`1234,56`).
pub fn fmt_valor_br(v: f64) -> String {
    format!("{:.2}", v).replace('.', ",")
}

/// Formata moeda no padrão BR com milhar (`R$ 11.580,00`).
/// Usado nos cartões de métricas (telas); CSV segue com `fmt_valor_br`.
#[must_use]
pub fn fmt_reais_br(v: f64) -> String {
    let base = format!("{:.2}", v);
    let (sinal, digitos) = base
        .strip_prefix('-')
        .map_or(("", base.as_str()), |r| ("-", r));
    let mut partes = digitos.splitn(2, '.');
    let inteiros = partes.next().unwrap_or("0");
    let centavos = partes.next().unwrap_or("00");
    let chars: Vec<char> = inteiros.chars().collect();
    let mut grupo = String::with_capacity(inteiros.len() + 2);
    for (i, c) in chars.iter().enumerate() {
        if i > 0 && (chars.len() - i).is_multiple_of(3) {
            grupo.push('.');
        }
        grupo.push(*c);
    }
    format!("R$ {sinal}{grupo},{centavos}")
}

/// Gera um nome de arquivo seguro a partir do rótulo do período.
///
/// # Exemplos
///
/// ```
/// assert_eq!(nextech::export::nome_arquivo_sugerido("10/2025"), "nextech-relatorio-10-2025.csv");
/// ```
#[must_use]
pub fn nome_arquivo_sugerido(escopo_label: &str) -> String {
    let mut base = String::with_capacity(escopo_label.len() + 24);
    for ch in escopo_label.chars() {
        if ch.is_ascii_alphanumeric() || ch == '-' || ch == '_' {
            base.push(ch);
        } else if ch == '/' || ch == ' ' || ch == '.' {
            base.push('-');
        }
    }
    let base = base.trim_matches('-');
    let base = if base.is_empty() { "geral" } else { base };
    format!("nextech-relatorio-{base}.csv")
}

fn situacao_ptbr(s: &crate::models::Sale) -> String {
    let restante = (s.total_price - s.paid_amount).max(0.0);
    format!(
        "{} (restante {})",
        s.status.label_ptbr(),
        fmt_valor_br(restante)
    )
}

fn origem_ptbr(s: &str) -> &str {
    match s {
        "bank" => "Banco",
        _ => "Gaveta",
    }
}

fn data_hora_br(s: &str) -> String {
    let s = s.trim();
    for fmt in ["%Y-%m-%d %H:%M:%S", "%Y-%m-%d %H:%M"] {
        if let Ok(dt) = chrono::NaiveDateTime::parse_from_str(s, fmt) {
            return dt.format("%d/%m/%Y %H:%M").to_string();
        }
    }
    s.to_string()
}

/// Gera o conteúdo CSV completo (com BOM UTF-8) do relatório.
///
/// # Erros
///
/// Retorna erro se a serialização CSV falhar.
pub fn gerar_csv_relatorio(dados: &DadosRelatorio) -> Result<Vec<u8>> {
    let mut wtr = csv::WriterBuilder::new()
        .delimiter(b';')
        .flexible(true)
        .from_writer(Vec::new());

    wtr.write_record(["Relatório Financeiro NexTech", dados.escopo_label])
        .context("falha ao escrever cabeçalho do relatório")?;
    // Filtro de pagamento removido (campo LEGADO ignorado).
    let _ = dados.filtro_pagamento;
    wtr.write_record([""; 2])
        .context("falha ao escrever separador")?;

    wtr.write_record(["Resumo", "Valor (R$)"])
        .context("falha ao escrever resumo")?;
    let resumo: [(&str, f64); 9] = [
        ("Total em vendas", dados.total_vendas),
        ("Total recebido", dados.total_recebido),
        ("Saldo devedor (fiado)", dados.total_devedor),
        ("Entradas", dados.total_entradas),
        ("Saídas", dados.total_saidas),
        ("Perdas", dados.total_perdas),
        ("Saldo atual", dados.saldo),
        ("Ticket médio", dados.ticket_medio),
        ("Qtd. vendas", dados.num_vendas as f64),
    ];
    for (rotulo, valor) in resumo {
        let campo = if rotulo == "Qtd. vendas" {
            dados.num_vendas.to_string()
        } else {
            fmt_valor_br(valor)
        };
        wtr.write_record([rotulo, campo.as_str()])
            .context("falha ao escrever linha do resumo")?;
    }
    wtr.write_record([""; 2])
        .context("falha ao escrever separador")?;

    wtr.write_record([
        "ID",
        "Tipo",
        "Produto/Serviço",
        "Cliente",
        "Telefone",
        "IMEI/Série",
        "Qtd",
        "Unitário (R$)",
        "Total (R$)",
        "Recebido (R$)",
        "Restante (R$)",
        "Situação",
        "Data/Hora",
    ])
    .context("falha ao escrever cabeçalho de vendas")?;
    for v in dados.vendas {
        wtr.write_record([
            v.id.to_string(),
            v.sale_type.label_ptbr().to_string(),
            v.name.clone(),
            v.client_name.clone(),
            v.phone.clone(),
            v.imei.clone(),
            v.quantity.to_string(),
            // Mesma regra da tabela: média seria enganosa no multi-item.
            if v.item_count > 1 {
                "—".to_string()
            } else {
                fmt_valor_br(v.unit_price)
            },
            fmt_valor_br(v.total_price),
            fmt_valor_br(v.paid_amount),
            fmt_valor_br((v.total_price - v.paid_amount).max(0.0)),
            situacao_ptbr(v),
            data_hora_br(&v.created_at),
        ])
        .context("falha ao escrever venda")?;
    }
    wtr.write_record([""; 2])
        .context("falha ao escrever separador")?;

    wtr.write_record([
        "ID",
        "Valor (R$)",
        "Tipo",
        "Descrição",
        "Origem",
        "Data/Hora",
    ])
    .context("falha ao escrever cabeçalho de caixa")?;
    for m in dados.movimentacoes {
        let tipo = if m.is_entry { "Entrada" } else { "Saída" };
        wtr.write_record([
            m.id.to_string(),
            fmt_valor_br(m.amount),
            tipo.to_string(),
            m.description.clone(),
            origem_ptbr(m.source.as_str()).to_string(),
            data_hora_br(&m.created_at),
        ])
        .context("falha ao escrever movimentação")?;
    }
    wtr.write_record([""; 2])
        .context("falha ao escrever separador")?;

    wtr.write_record(["ID", "Valor (R$)", "Descrição", "Origem", "Data/Hora"])
        .context("falha ao escrever cabeçalho de perdas")?;
    for p in dados.perdas_lista {
        wtr.write_record([
            p.id.to_string(),
            fmt_valor_br(p.amount),
            p.description.clone(),
            origem_ptbr(p.source.as_str()).to_string(),
            data_hora_br(&p.created_at),
        ])
        .context("falha ao escrever perda")?;
    }

    let inner = wtr.into_inner().context("falha ao finalizar CSV")?;
    let mut out = Vec::with_capacity(inner.len() + 3);
    out.extend_from_slice(&[0xEF, 0xBB, 0xBF]);
    out.extend_from_slice(&inner);
    Ok(out)
}

/// Salva os bytes do relatório no caminho escolhido pelo operador.
///
/// # Erros
///
/// Retorna erro se a escrita no disco falhar.
pub fn salvar_bytes_em_arquivo(caminho: &Path, bytes: &[u8]) -> Result<()> {
    std::fs::write(caminho, bytes)
        .with_context(|| format!("falha ao salvar em {}", caminho.display()))?;
    Ok(())
}

/// Nome de arquivo sugerido para o CSV de reposição de estoque.
#[must_use]
pub fn nome_arquivo_reposicao(start_date: &str, end_date: &str) -> String {
    let limpa = |s: &str| {
        let mut base = String::with_capacity(s.len());
        for ch in s.chars() {
            if ch.is_ascii_alphanumeric() || ch == '-' || ch == '_' {
                base.push(ch);
            } else if ch == '/' || ch == ' ' || ch == '.' {
                base.push('-');
            }
        }
        let t = base.trim_matches('-').to_string();
        if t.is_empty() { "geral".to_string() } else { t }
    };
    format!(
        "nextech-reposicao-{}_a_{}.csv",
        limpa(start_date),
        limpa(end_date)
    )
}

/// Gera o conteúdo CSV (com BOM UTF-8) da agregação de itens.
///
/// Colunas exatas: `Produto;Quantidade_Vendida;Faturamento_Total`, com
/// valores monetários em `fmt_valor_br`. A ordem das linhas é preservada
/// (o chamador passa já ordenado por quantidade DESC).
///
/// # Erros
///
/// Retorna erro se a serialização CSV falhar.
pub fn gerar_csv_item_sales(
    items: &[ItemSalesSummary],
    start_date: &str,
    end_date: &str,
) -> Result<Vec<u8>> {
    let mut wtr = csv::WriterBuilder::new()
        .delimiter(b';')
        .flexible(true)
        .from_writer(Vec::new());

    wtr.write_record([
        "Relatório Reposição de Estoque",
        &format!("{start_date} a {end_date}"),
    ])
    .context("falha ao escrever cabeçalho da reposição")?;
    wtr.write_record([""; 2])
        .context("falha ao escrever separador")?;
    wtr.write_record(["Produto", "Quantidade_Vendida", "Faturamento_Total"])
        .context("falha ao escrever cabeçalho de itens")?;
    for item in items {
        wtr.write_record([
            item.name.clone(),
            item.total_quantity.to_string(),
            fmt_valor_br(item.total_revenue),
        ])
        .context("falha ao escrever item")?;
    }

    let inner = wtr.into_inner().context("falha ao finalizar CSV")?;
    let mut out = Vec::with_capacity(inner.len() + 3);
    out.extend_from_slice(&[0xEF, 0xBB, 0xBF]);
    out.extend_from_slice(&inner);
    Ok(out)
}

/// Diretório inicial dos diálogos de salvamento CSV.
///
/// Windows: `Documentos` (`dirs::document_dir()`); fallback: `Imagens`,
/// diretório de dados local ou temporário do SO. Nunca retorna vazio —
/// o `rfd` ignora diretório inexistente, então o melhor-esforço aqui só
/// melhora a UX sem falhar a exportação.
#[must_use]
pub fn default_export_dir() -> PathBuf {
    dirs::document_dir()
        .or_else(dirs::picture_dir)
        .or_else(dirs::data_local_dir)
        .unwrap_or_else(std::env::temp_dir)
}

/// Gera o CSV de reposição e abre o diálogo de salvamento (`rfd`).
///
/// Retorna o caminho escolhido. A UI exibe o feedback; o cancelamento
/// pelo operador é tratado como erro recuperável.
///
/// # Erros
///
/// Retorna erro se a geração falhar, se o operador cancelar o diálogo
/// ou se a escrita no disco falhar.
pub fn export_item_sales_csv(
    items: &[ItemSalesSummary],
    start_date: &str,
    end_date: &str,
) -> Result<PathBuf> {
    let bytes = gerar_csv_item_sales(items, start_date, end_date)?;
    let sugestao = nome_arquivo_reposicao(start_date, end_date);
    let caminho = rfd::FileDialog::new()
        .add_filter("CSV (Excel)", &["csv"])
        .set_directory(default_export_dir())
        .set_file_name(&sugestao)
        .save_file()
        .context("exportação cancelada pelo operador")?;
    salvar_bytes_em_arquivo(&caminho, &bytes)?;
    Ok(caminho)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{CashSource, PaymentMethod};

    fn venda_exemplo() -> Sale {
        Sale {
            id: 1,
            name: "Troca de tela".to_string(),
            quantity: 1,
            unit_price: 149.9,
            total_price: 149.9,
            payment_method: PaymentMethod::Cash,
            paid_amount: 149.9,
            valor_recebido: 149.9,
            troco: 0.0,
            client_name: String::new(),
            phone: String::new(),
            imei: String::new(),
            sale_type: crate::models::SaleType::Rapida,
            status: crate::models::SaleStatus::Quitado,
            created_at: "2025-10-07 10:30:00".to_string(),
            item_count: 1,
            items: vec![crate::models::SaleItem {
                id: 1,
                sale_id: 1,
                name: "Troca de tela".to_string(),
                quantity: 1,
                unit_price: 149.9,
                total_price: 149.9,
                imei: String::new(),
                item_type: crate::models::SaleType::Rapida,
            }],
        }
    }

    #[test]
    fn formata_valor_no_padrao_br() {
        assert_eq!(fmt_valor_br(149.9), "149,90");
        assert_eq!(fmt_valor_br(0.0), "0,00");
    }

    #[test]
    fn formata_reais_com_milhar() {
        assert_eq!(fmt_reais_br(11580.0), "R$ 11.580,00");
        assert_eq!(fmt_reais_br(1750.0), "R$ 1.750,00");
        assert_eq!(fmt_reais_br(999.99), "R$ 999,99");
        assert_eq!(fmt_reais_br(0.0), "R$ 0,00");
        assert_eq!(fmt_reais_br(1234567.89), "R$ 1.234.567,89");
    }

    #[test]
    fn gera_nome_de_arquivo_seguro() {
        assert_eq!(
            nome_arquivo_sugerido("10/2025"),
            "nextech-relatorio-10-2025.csv"
        );
        assert_eq!(
            nome_arquivo_sugerido("Geral"),
            "nextech-relatorio-Geral.csv"
        );
    }

    #[test]
    fn csv_tem_bom_e_delimitador_ptbr() {
        let vendas = vec![venda_exemplo()];
        let dados = DadosRelatorio {
            escopo_label: "10/2025",
            filtro_pagamento: "Pix",
            total_vendas: 149.9,
            total_recebido: 149.9,
            total_devedor: 0.0,
            total_entradas: 50.0,
            total_saidas: 10.0,
            total_perdas: 0.0,
            saldo: 189.9,
            num_vendas: 1,
            ticket_medio: 149.9,
            vendas: &vendas,
            movimentacoes: &[],
            perdas_lista: &[],
        };
        let bytes = gerar_csv_relatorio(&dados).expect("csv deve gerar");
        assert!(bytes.starts_with(&[0xEF, 0xBB, 0xBF]));
        let texto = String::from_utf8(bytes).expect("csv deve ser utf-8");
        assert!(texto.contains("Relatório Financeiro NexTech;10/2025"));
        assert!(texto.contains("149,90"));
        assert!(texto.contains("07/10/2025 10:30"));
    }

    #[test]
    fn csv_escapa_ponto_e_virgula_na_descricao() {
        let entrada = CashEntry {
            id: 2,
            amount: 20.0,
            is_entry: true,
            source: CashSource::Drawer,
            description: "Reforço; caixa".to_string(),
            created_at: "2025-10-07 11:00:00".to_string(),
        };
        let dados = DadosRelatorio {
            escopo_label: "Geral",
            filtro_pagamento: "Dinheiro",
            total_vendas: 0.0,
            total_recebido: 0.0,
            total_devedor: 0.0,
            total_entradas: 20.0,
            total_saidas: 0.0,
            total_perdas: 0.0,
            saldo: 20.0,
            num_vendas: 0,
            ticket_medio: 0.0,
            vendas: &[],
            movimentacoes: &[entrada],
            perdas_lista: &[],
        };
        let bytes = gerar_csv_relatorio(&dados).expect("csv deve gerar");
        let texto = String::from_utf8(bytes).expect("csv deve ser utf-8");
        assert!(texto.contains("\"Reforço; caixa\""));
    }

    #[test]
    fn csv_multi_item_mostra_traco_no_unitario() {
        let mut venda = venda_exemplo();
        venda.name = "iPhone 12 (+1 item)".to_string();
        venda.item_count = 2;
        venda.total_price = 2540.0;
        venda.paid_amount = 2540.0;
        let vendas = vec![venda];
        let dados = DadosRelatorio {
            escopo_label: "Geral",
            filtro_pagamento: "Todos",
            total_vendas: 2540.0,
            total_recebido: 2540.0,
            total_devedor: 0.0,
            total_entradas: 0.0,
            total_saidas: 0.0,
            total_perdas: 0.0,
            saldo: 2540.0,
            num_vendas: 1,
            ticket_medio: 2540.0,
            vendas: &vendas,
            movimentacoes: &[],
            perdas_lista: &[],
        };
        let bytes = gerar_csv_relatorio(&dados).expect("csv deve gerar");
        let texto = String::from_utf8(bytes).expect("csv deve ser utf-8");
        assert!(texto.contains("iPhone 12 (+1 item)"));
        // Unitário vira traço isolado entre `;` (média seria enganosa).
        assert!(texto.contains(";—;"));
        assert!(!texto.contains("149,90"));
    }

    fn resumo(nome: &str, qtd: i64, receita: f64, vendas: i64) -> ItemSalesSummary {
        ItemSalesSummary {
            name: nome.to_string(),
            total_quantity: qtd,
            total_revenue: receita,
            sales_count: vendas,
        }
    }

    #[test]
    fn csv_reposicao_tem_bom_e_header_exato() {
        let itens = vec![
            resumo("iPhone 12", 5, 12500.0, 3),
            resumo("iPhone 12 Pro Max", 2, 7000.0, 2),
        ];
        let bytes =
            gerar_csv_item_sales(&itens, "2026-10-01", "2026-10-31").expect("csv deve gerar");
        assert!(bytes.starts_with(&[0xEF, 0xBB, 0xBF]));
        let texto = String::from_utf8(bytes).expect("csv deve ser utf-8");
        assert!(texto.contains("Produto;Quantidade_Vendida;Faturamento_Total"));
        assert!(texto.contains("iPhone 12;5;12500,00"));
        assert!(texto.contains("iPhone 12 Pro Max;2;7000,00"));
        // Sub-modelos seguem em linhas distintas, jamais somadas.
        assert_eq!(texto.matches("iPhone 12").count(), 2);
    }

    #[test]
    fn csv_reposicao_escapa_ponto_e_virgula() {
        let itens = vec![resumo("Capinha; silicone", 3, 120.0, 2)];
        let bytes =
            gerar_csv_item_sales(&itens, "2026-10-01", "2026-10-31").expect("csv deve gerar");
        let texto = String::from_utf8(bytes).expect("csv deve ser utf-8");
        assert!(texto.contains("\"Capinha; silicone\""));
    }

    #[test]
    fn csv_reposicao_preserva_ordem_e_nome_de_arquivo() {
        let itens = vec![
            resumo("iPhone 14", 4, 16000.0, 2),
            resumo("Capinha", 10, 400.0, 5),
        ];
        let bytes =
            gerar_csv_item_sales(&itens, "2026-10-01", "2026-10-31").expect("csv deve gerar");
        let texto = String::from_utf8(bytes).expect("csv deve ser utf-8");
        let pos_14 = texto.find("iPhone 14").expect("linha 14");
        let pos_cap = texto.find("Capinha").expect("linha capinha");
        assert!(pos_14 < pos_cap);
        assert_eq!(
            nome_arquivo_reposicao("2026-10-01", "2026-10-31"),
            "nextech-reposicao-2026-10-01_a_2026-10-31.csv"
        );
    }

    #[test]
    fn csv_reposicao_lista_vazia_tem_so_cabecalho() {
        // Categoria sem vendas no período: CSV válido, só com cabeçalho.
        let bytes = gerar_csv_item_sales(&[], "2026-10-01", "2026-10-31").expect("csv deve gerar");
        assert!(bytes.starts_with(&[0xEF, 0xBB, 0xBF]));
        let texto = String::from_utf8(bytes).expect("csv deve ser utf-8");
        assert!(texto.contains("Produto;Quantidade_Vendida;Faturamento_Total"));
    }
}
