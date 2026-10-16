//! Comprovante de venda / Termo de Garantia — recibo HTML para impressão A4.
//!
//! Documento autocontido (CSS inline, zero `<img>`, sem assets externos)
//! para abrir via `file://` no navegador padrão e caber em UMA folha A4
//! (`@page { size: A4 portrait; margin: 8mm; }`). Todo o cabeçalho é
//! tipografia + CSS puro (vetor): rasteriza na resolução da impressora
//! (300+ DPI), sem pixelar como uma imagem. Layout tinta-amigável:
//! recibo impresso, não tela — desvio intencional do tema zinc-dark do app.
//!
//! Meio de pagamento removido: sem linha "Forma de pagamento".
//! Garantia em blocos separados por categoria da linha: A (Aparelho) se
//! houver qualquer item Aparelho, B (Serviço) se houver qualquer item
//! Serviço, ambos em sequência no `Mista` completo; assinaturas ao final.

use std::path::PathBuf;

use crate::export::fmt_valor_br;
use crate::models::{Sale, SaleType};

const STORE_BRAND: &str = "NX NEXTECH";
const STORE_TAGLINE: &str = "Celulares & Acessórios";
const STORE_SUBTITLE: &str = "Mercado Novo - Centro - Correntina - BA";
const STORE_WHATSAPP: &str = "(77) 99193-4388";
const STORE_INSTAGRAM: &str = "@celulares_nextech";

/// Título do documento por tipo de atendimento.
///
/// `Mista` (carrinho multi-categoria) usa o título genérico de venda;
/// a garantia combinada abaixo cobre serviço + aparelho.
fn doc_title_for(sale_type: SaleType) -> &'static str {
    match sale_type {
        SaleType::Rapida | SaleType::Mista => "COMPROVANTE DE VENDA",
        SaleType::Servico => "TERMO DE GARANTIA E COMPROVANTE DE SERVIÇO",
        SaleType::Aparelho => "TERMO DE GARANTIA DE APARELHO",
    }
}

/// Escapa texto controlado pelo operador/cliente para HTML seguro.
#[must_use]
pub fn escape_html(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&#39;"),
            _ => out.push(c),
        }
    }
    out
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

fn dash_if_empty(s: &str) -> String {
    let t = s.trim();
    if t.is_empty() {
        "—".to_string()
    } else {
        escape_html(t)
    }
}

/// Bloco "Cliente e Aparelho" (sem box de IMEI).
///
/// Venda Rápida omite o bloco por completo (balcão, compacto, sem traços
/// vazios). Serviço, Aparelho e Mista exibem cliente + telefone; IMEIs
/// saem sempre inline nas linhas da tabela (estrutura uniforme
/// single/multi).
fn customer_block(sale: &Sale) -> String {
    if sale.sale_type == SaleType::Rapida {
        return String::new();
    }
    format!(
        "<h2>Cliente e Aparelho</h2>\n<table class=\"kv\">\n<tr><td>Cliente</td><td>{}</td></tr>\n<tr><td>Telefone</td><td>{}</td></tr>\n</table>",
        dash_if_empty(&sale.client_name),
        dash_if_empty(&sale.phone),
    )
}

/// Bloco A — garantia de aparelhos (HTML exato do termo da loja).
fn warranty_aparelho_html() -> String {
    "<div class=\"warranty-block\">\n    <h4>TERMO DE GARANTIA - APARELHOS (90 DIAS)</h4>\n    <ul>\n        <li><b>Não cobre:</b> mau uso, quedas, tela trincada, umidade/oxidação, descargas elétricas, lacre de garantia violado, bloqueios de software/contas ou intervenção de terceiros.</li>\n        <li>Troca do aparelho em até 07 (sete) dias em caso de defeito de fabricação.</li>\n        <li>Partes externas (Painéis, Telas, Teclas) devem ser testadas na loja no ato da compra.</li>\n    </ul>\n    <div style=\"border: 1px solid #333; padding: 5px; text-align: center; font-weight: bold; margin-top: 5px;\">\n        TROCA MÁXIMA DE APARELHOS EM ATÉ 07 DIAS APÓS A VENDA MEDIANTE DEFEITO DE FÁBRICA.\n    </div>\n    <div style=\"border: 1px solid #333; padding: 5px; text-align: center; margin-top: 5px; font-size: 10px;\">\n        ATENÇÃO CLIENTE: Declaro para os devidos fins que o aparelho foi testado e encontra-se em perfeitas condições. A garantia contra defeitos de fabricação é de 90 dias, e a condição para troca imediata é de até 07 dias (sem avarias físicas).\n    </div>\n</div>".to_string()
}

/// Bloco B — garantia de serviços (HTML exato do termo da loja).
fn warranty_servico_html() -> String {
    "<div class=\"warranty-block\">\n    <h4>TERMO DE GARANTIA - SERVIÇOS (90 DIAS)</h4>\n    <ul>\n        <li>A garantia cobre <b>EXCLUSIVAMENTE</b> o serviço executado ou a peça aplicada constantes nesta nota pelo prazo de 90 (noventa) dias a contar da data acima.</li>\n        <li><b>Não cobre:</b> danos por água/umidade, quedas ou tela trincada após a entrega, lacre violado ou intervenção de terceiros.</li>\n        <li>A garantia só é válida mediante apresentação deste comprovante.</li>\n    </ul>\n</div>".to_string()
}

/// Blocos de garantia por conteúdo do carrinho (não só cabeçalho).
///
/// - Tem item `Aparelho` → Bloco A; tem item `Serviço` → Bloco B;
///   `Mista` completa → A seguido de B; só acessórios → nada.
/// - Vendas legadas sem linhas: fallback pelo cabeçalho.
/// - Assinatura à direita condicional: tem item `Serviço` →
///   `Técnico Responsável`; sem `Serviço` (Aparelho puro, acessórios)
///   → `Vendedor / Atendente`. Uma vez ao final, se houver qualquer
///   bloco — proteção legal da loja.
fn warranty_blocks_for_sale(sale: &Sale) -> String {
    let (mut tem_aparelho, mut tem_servico) = (false, false);
    if sale.items.is_empty() {
        match sale.sale_type {
            SaleType::Aparelho => tem_aparelho = true,
            SaleType::Servico => tem_servico = true,
            SaleType::Mista => {
                tem_aparelho = true;
                tem_servico = true;
            }
            SaleType::Rapida => {}
        }
    } else {
        for i in &sale.items {
            match i.item_type {
                SaleType::Aparelho => tem_aparelho = true,
                SaleType::Servico => tem_servico = true,
                SaleType::Rapida | SaleType::Mista => {}
            }
        }
    }
    if !tem_aparelho && !tem_servico {
        return String::new();
    }
    let mut out = String::new();
    if tem_aparelho {
        out.push_str(&warranty_aparelho_html());
    }
    if tem_servico {
        out.push_str(&warranty_servico_html());
    }
    let signer = if tem_servico {
        "Técnico Responsável"
    } else {
        "Vendedor / Atendente"
    };
    out.push_str(&format!("<div class=\"sign signature-block\"><div>Assinatura do Cliente</div><div>{signer}</div></div>"));
    out
}

/// Linhas da tabela de itens: 1 `<tr>` por item, com o IMEI como
/// sub-rótulo compacto sob o nome quando presente. Vendas legadas
/// (`items` vazio) caem no fallback de 1 linha do cabeçalho.
fn item_rows(sale: &Sale) -> String {
    let lines: Vec<(i64, &str, f64, f64, &str)> = if sale.items.is_empty() {
        vec![(
            sale.quantity,
            sale.name.as_str(),
            sale.unit_price,
            (sale.quantity as f64) * sale.unit_price,
            sale.imei.as_str(),
        )]
    } else {
        sale.items
            .iter()
            .map(|i| {
                (
                    i.quantity,
                    i.name.as_str(),
                    i.unit_price,
                    i.total_price,
                    i.imei.as_str(),
                )
            })
            .collect()
    };
    lines
        .into_iter()
        .map(|(qty, name, unit, sub, imei)| {
            let imei_line = if imei.trim().is_empty() {
                String::new()
            } else {
                format!(
                    "<div class=\"item-imei\">IMEI: {}</div>",
                    escape_html(imei.trim())
                )
            };
            format!(
                "<tr><td>{qty}</td><td>{}{imei_line}</td><td class=\"r\">{}</td><td class=\"r strong\">{}</td></tr>",
                escape_html(name),
                fmt_valor_br(unit),
                fmt_valor_br(sub),
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// Gera o HTML completo do comprovante/garantia para a venda.
///
/// Tabela Pagamento: Total (= caixa), Valor recebido, Troco (se > 0) e
/// Valor em aberto quando fiado. Garantia: blocos por categoria da linha.
#[must_use]
pub fn generate_warranty_html(sale: &Sale) -> String {
    let remaining = sale.remaining();
    let debt_block = if sale.is_debt() {
        format!(
            "<tr><td>Valor em aberto (fiado)</td><td class=\"r strong\">R$ {}</td></tr>",
            fmt_valor_br(remaining)
        )
    } else {
        String::new()
    };
    let troco_block = if sale.troco > 0.005 {
        format!(
            "<tr class=\"troco\"><td>Troco</td><td class=\"r strong\">R$ {}</td></tr>",
            fmt_valor_br(sale.troco)
        )
    } else {
        String::new()
    };
    format!(
        r#"<!DOCTYPE html>
<html lang="pt-BR">
<head>
<meta charset="utf-8">
<title>{doc_title} — Venda #{id}</title>
<style>
@page {{ size: A4 portrait; margin: 8mm; }}
* {{ box-sizing: border-box; -webkit-print-color-adjust: exact; print-color-adjust: exact; }}
html, body {{ width: auto; }}
body {{ font-family: Arial, Helvetica, sans-serif; color: #111; margin: 0; padding: 0; font-size: 11.5px; line-height: 1.4; }}
.brand {{ text-align: center; border-bottom: 3px double #065f46; padding-bottom: 8px; margin-bottom: 10px; }}
.brand .wordmark {{ font-size: 24px; font-weight: 800; letter-spacing: 6px; color: #065f46; margin: 0; }}
.brand .tagline {{ font-size: 10px; font-weight: bold; letter-spacing: 3px; text-transform: uppercase; color: #6b7280; margin-top: 2px; }}
.brand .locale {{ font-size: 10px; color: #333; margin-top: 2px; }}
.brand .badges {{ margin-top: 6px; }}
.brand .badge {{ display: inline-block; border: 1px solid #065f46; border-radius: 20px; padding: 2px 10px; font-size: 10px; color: #065f46; margin: 0 4px; }}
.brand .badge .lbl {{ font-weight: bold; }}
.doc-title {{ font-size: 13px; font-weight: 800; color: #065f46; margin-top: 8px; letter-spacing: 1px; }}
.meta {{ display: flex; justify-content: space-between; margin: 8px 0; font-size: 11.5px; }}
h2 {{ font-size: 12px; margin: 10px 0 4px; border-bottom: 1px solid #999; padding-bottom: 2px; }}
h4 {{ font-size: 12px; margin: 10px 0 4px; }}
table {{ width: 100%; border-collapse: collapse; margin-top: 4px; }}
th, td {{ border: 1px solid #666; padding: 4px 6px; text-align: left; font-size: 11.5px; }}
th {{ background: #eee; }}
td.r, th.r {{ text-align: right; }}
.strong {{ font-weight: bold; }}
.kv td:first-child {{ width: 38%; font-weight: bold; background: #f6f6f6; }}
.kv tr.recv td {{ border-bottom: none; padding-bottom: 1px; }}
.kv tr.troco td {{ border-top: none; padding-top: 1px; }}
.item-imei {{ font-size: 10px; font-weight: bold; margin-top: 2px; }}
ul.clauses {{ margin: 4px 0; padding-left: 18px; }}
ul.clauses li {{ margin-bottom: 3px; }}
.notice {{ margin-top: 6px; padding: 6px 8px; border: 2px solid #111; font-weight: bold; text-align: center; font-size: 11.5px; }}
.warranty-block, .signature-block, table, tr {{ page-break-inside: avoid; }}
.compact th, .compact td {{ padding: 3px 4px; font-size: 10.5px; }}
.compact .item-imei {{ font-size: 10px; }}
.compact h2 {{ margin: 8px 0 3px; }}
.compact .brand .wordmark {{ font-size: 20px; }}
.sign {{ display: flex; justify-content: space-between; margin-top: 24px; }}
.sign div {{ width: 45%; border-top: 1px solid #111; padding-top: 4px; text-align: center; font-size: 10px; }}
.print-btn {{ display: block; margin: 0 auto 12px; padding: 10px 28px; font-size: 14px; cursor: pointer; }}
@media print {{ .no-print {{ display: none; }} body {{ padding: 0; }} }}
</style>
</head>
<body{body_class}>
<button class="print-btn no-print" onclick="window.print()">🖨️ Imprimir</button>
<div class="brand">
<div class="wordmark">{store}</div>
<div class="tagline">{tagline}</div>
<div class="locale">{subtitle}</div>
<div class="badges"><span class="badge"><span class="lbl">WhatsApp:</span> {whatsapp}</span><span class="badge"><span class="lbl">Instagram:</span> {instagram}</span></div>
<div class="doc-title">{doc_title}</div>
</div>
<div class="meta"><span><strong>Venda Nº:</strong> #{id}</span><span><strong>Data/Hora:</strong> {when}</span></div>
<div class="meta"><span><strong>Tipo:</strong> {kind}</span></div>
{customer}
<h2>Item</h2>
<table>
<tr><th>Qtd</th><th>{item_col}</th><th class="r">Unitário (R$)</th><th class="r">Total (R$)</th></tr>
{rows}
</table>
<h2>Pagamento</h2>
<table class="kv">
<tr><td>Total</td><td>R$ {total2}</td></tr>
<tr class="recv"><td>Valor recebido</td><td>R$ {paid}</td></tr>
{troco}
{debt}
</table>
{warranty}
</body>
</html>"#,
        doc_title = doc_title_for(sale.sale_type),
        id = sale.id,
        item_col = if sale.sale_type == SaleType::Mista {
            "Item"
        } else {
            crate::models::item_label(sale.sale_type)
        },
        body_class = if sale.items.len() > 1 {
            " class=\"compact\""
        } else {
            ""
        },
        store = escape_html(STORE_BRAND),
        tagline = escape_html(STORE_TAGLINE),
        subtitle = escape_html(STORE_SUBTITLE),
        whatsapp = escape_html(STORE_WHATSAPP),
        instagram = escape_html(STORE_INSTAGRAM),
        when = format_datetime_br(&sale.created_at),
        kind = sale.sale_type.label_ptbr(),
        customer = customer_block(sale),
        rows = item_rows(sale),
        total2 = fmt_valor_br(sale.total_price),
        paid = fmt_valor_br(sale.valor_recebido),
        troco = troco_block,
        debt = debt_block,
        warranty = warranty_blocks_for_sale(sale),
    )
}

/// Caminho do recibo no diretório temporário do SO (um arquivo por venda).
#[must_use]
pub fn receipt_path(sale_id: i64) -> PathBuf {
    std::env::temp_dir().join(format!("nextech_recibo_{sale_id}.html"))
}

/// Salva o recibo em arquivo temporário e abre no navegador padrão.
///
/// # Erros
///
/// Retorna mensagem PT-BR se a escrita ou a abertura falhar.
pub fn open_receipt_in_browser(sale: &Sale) -> Result<(), String> {
    let path = receipt_path(sale.id);
    std::fs::write(&path, generate_warranty_html(sale))
        .map_err(|e| format!("⚠️ Não foi possível salvar o recibo: {e}"))?;
    open_path(&path)
}

fn open_path(path: &std::path::Path) -> Result<(), String> {
    open::that(path).map_err(|e| format!("⚠️ Não foi possível abrir o recibo: {e}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{PaymentMethod, SaleStatus};

    fn venda_exemplo() -> Sale {
        Sale {
            id: 42,
            name: "Troca de Tela - Galaxy A12".to_string(),
            quantity: 1,
            unit_price: 250.0,
            total_price: 250.0,
            payment_method: PaymentMethod::Cash,
            paid_amount: 250.0,
            valor_recebido: 250.0,
            troco: 0.0,
            client_name: "Maria Silva".to_string(),
            phone: "(77) 99999-0000".to_string(),
            imei: String::new(),
            sale_type: SaleType::Servico,
            status: SaleStatus::Quitado,
            created_at: "2026-10-14 10:30:00".to_string(),
            item_count: 1,
            items: vec![crate::models::SaleItem {
                id: 1,
                sale_id: 42,
                name: "Troca de Tela - Galaxy A12".to_string(),
                quantity: 1,
                unit_price: 250.0,
                total_price: 250.0,
                imei: String::new(),
                item_type: SaleType::Servico,
            }],
        }
    }

    fn venda_multi_exemplo() -> Sale {
        let mut v = venda_exemplo();
        v.id = 43;
        v.name = "iPhone 12 (+1 item)".to_string();
        v.quantity = 2;
        v.unit_price = 1270.0;
        v.total_price = 2540.0;
        v.paid_amount = 2540.0;
        v.valor_recebido = 2540.0;
        v.troco = 0.0;
        v.imei = "355123456789012".to_string();
        v.sale_type = SaleType::Mista;
        v.item_count = 2;
        v.items = vec![
            crate::models::SaleItem {
                id: 2,
                sale_id: 43,
                name: "iPhone 12".to_string(),
                quantity: 1,
                unit_price: 2500.0,
                total_price: 2500.0,
                imei: "355123456789012".to_string(),
                item_type: SaleType::Aparelho,
            },
            crate::models::SaleItem {
                id: 3,
                sale_id: 43,
                name: "Capinha - Silicone".to_string(),
                quantity: 1,
                unit_price: 40.0,
                total_price: 40.0,
                imei: String::new(),
                item_type: SaleType::Rapida,
            },
        ];
        v
    }

    #[test]
    fn contem_marca_titulo_id_e_css_a4() {
        let html = generate_warranty_html(&venda_exemplo());
        assert!(html.contains("NEXTECH"));
        assert!(html.contains("TERMO DE GARANTIA E COMPROVANTE DE SERVIÇO"));
        assert!(html.contains("Venda Nº:</strong> #42"));
        assert!(html.contains("@page"));
        assert!(html.contains("size: A4 portrait"));
        assert!(html.contains("margin: 8mm"));
        assert!(!html.contains("size: A5 portrait"));
        assert!(html.contains("NX NEXTECH"));
        assert!(html.contains("Mercado Novo - Centro - Correntina - BA"));
        assert!(html.contains("WhatsApp:"));
        assert!(html.contains("(77) 99193-4388"));
        assert!(html.contains("Instagram:"));
        assert!(html.contains("@celulares_nextech"));
        assert!(html.contains("14/10/2026 10:30"));
    }

    #[test]
    fn header_vetorial_sem_imagem() {
        let html = generate_warranty_html(&venda_exemplo());
        assert!(!html.contains("<img"));
        assert!(html.contains("wordmark"));
        assert!(html.contains("#065f46"));
        assert!(html.contains("print-color-adjust"));
    }

    #[test]
    fn cliente_telefone_e_totais_formatados() {
        let html = generate_warranty_html(&venda_exemplo());
        assert!(html.contains("Cliente e Aparelho"));
        assert!(html.contains("Maria Silva"));
        assert!(html.contains("(77) 99999-0000"));
        assert!(html.contains("250,00"));
        // Meio de pagamento removido do recibo.
        assert!(!html.contains("Forma de pagamento"));
        assert!(!html.contains(">Pix<"));
        assert!(html.contains("Serviço"));
    }

    #[test]
    fn parcial_exibe_recebido_e_restante() {
        let mut v = venda_exemplo();
        v.paid_amount = 100.0;
        v.valor_recebido = 100.0;
        v.status = SaleStatus::Parcial;
        let html = generate_warranty_html(&v);
        assert!(html.contains("100,00"));
        assert!(html.contains("Valor em aberto (fiado)"));
        assert!(html.contains("150,00"));
    }

    #[test]
    fn quitada_nao_exibe_bloco_fiado() {
        let html = generate_warranty_html(&venda_exemplo());
        assert!(!html.contains("Valor em aberto (fiado)"));
    }

    #[test]
    fn imei_em_destaque_para_aparelho() {
        let mut v = venda_exemplo();
        v.sale_type = SaleType::Aparelho;
        v.imei = "355123456789012".to_string();
        v.items[0].imei = "355123456789012".to_string();
        v.items[0].item_type = SaleType::Aparelho;
        let html = generate_warranty_html(&v);
        assert!(html.contains("TERMO DE GARANTIA - APARELHOS (90 DIAS)"));
        assert!(html.contains("<div class=\"item-imei\">IMEI: 355123456789012</div>"));
        assert!(html.contains("Aparelho"));
    }

    #[test]
    fn aparelho_sem_imei_sem_marcador() {
        let mut v = venda_exemplo();
        v.sale_type = SaleType::Aparelho;
        v.imei = String::new();
        v.items[0].imei = String::new();
        v.items[0].item_type = SaleType::Aparelho;
        let html = generate_warranty_html(&v);
        // Sem box standalone e sem marcador: bloco de cliente intacto.
        assert!(!html.contains("NÃO REGISTRADO"));
        assert!(!html.contains("<div class=\"imei\""));
        assert!(!html.contains("<div class=\"item-imei\">"));
        assert!(html.contains("Cliente e Aparelho"));
    }

    #[test]
    fn aparelho_bloco_a_completo() {
        let mut v = venda_exemplo();
        v.sale_type = SaleType::Aparelho;
        v.imei = "355123456789012".to_string();
        v.items[0].item_type = SaleType::Aparelho;
        let html = generate_warranty_html(&v);
        assert!(html.contains("TERMO DE GARANTIA - APARELHOS (90 DIAS)"));
        assert!(html.contains("descargas elétricas"));
        assert!(html.contains("TROCA MÁXIMA DE APARELHOS EM ATÉ 07 DIAS"));
        assert!(html.contains("Assinatura do Cliente"));
        // Aparelho puro (sem Serviço): atendente, nunca técnico.
        assert!(html.contains("Vendedor / Atendente"));
        assert!(!html.contains("Técnico Responsável"));
        // Bloco B (serviço) ausente no aparelho puro.
        assert!(!html.contains("TERMO DE GARANTIA - SERVIÇOS (90 DIAS)"));
    }

    #[test]
    fn aparelho_declara_aceite_e_teste() {
        let mut v = venda_exemplo();
        v.sale_type = SaleType::Aparelho;
        v.imei = "355123456789012".to_string();
        v.items[0].item_type = SaleType::Aparelho;
        let html = generate_warranty_html(&v);
        assert!(html.contains("ATENÇÃO CLIENTE:"));
        assert!(html.contains("perfeitas condições."));
        assert!(html.contains("até 07 dias (sem avarias físicas)"));
        // Serviço não tem caixa de aceite de aparelho.
        let servico = generate_warranty_html(&venda_exemplo());
        assert!(!servico.contains("ATENÇÃO CLIENTE:"));
    }

    #[test]
    fn servico_bloco_b_sem_aviso_aparelho() {
        const AVISO_A: &str = "TROCA MÁXIMA DE APARELHOS EM ATÉ 07 DIAS APÓS A VENDA MEDIANTE DEFEITO DE FÁBRICA.";
        let html = generate_warranty_html(&venda_exemplo());
        assert!(html.contains("TERMO DE GARANTIA - SERVIÇOS (90 DIAS)"));
        assert!(html.contains("EXCLUSIVAMENTE</b> o serviço executado ou a peça aplicada constantes nesta nota"));
        assert!(html.contains("tela trincada após a entrega"));
        assert!(html.contains("intervenção de terceiros"));
        assert!(html.contains("Assinatura do Cliente"));
        // Com Serviço: técnico responsável.
        assert!(html.contains("Técnico Responsável"));
        assert!(!html.contains("Vendedor / Atendente"));
        assert!(!html.contains(AVISO_A));
    }

    #[test]
    fn blocos_garantia_e_assinatura_nao_quebram() {
        let html = generate_warranty_html(&venda_exemplo());
        assert!(html.contains("class=\"warranty-block\""));
        assert!(html.contains("sign signature-block"));
        assert!(html.contains("page-break-inside: avoid"));
    }

    #[test]
    fn compact_apenas_no_multi_item() {
        let unico = generate_warranty_html(&venda_exemplo());
        assert!(!unico.contains("<body class=\"compact\">"));
        let multi = generate_warranty_html(&venda_multi_exemplo());
        assert!(multi.contains("<body class=\"compact\">"));
        assert!(multi.contains(".compact th"));
    }

    #[test]
    fn venda_rapida_compacta_sem_cliente_nem_garantia() {
        let mut v = venda_exemplo();
        v.sale_type = SaleType::Rapida;
        v.client_name = String::new();
        v.phone = String::new();
        v.items[0].item_type = SaleType::Rapida;
        let html = generate_warranty_html(&v);
        assert!(html.contains("COMPROVANTE DE VENDA"));
        assert!(!html.contains("Cliente e Aparelho"));
        assert!(!html.contains("<td>—</td>"));
        assert!(!html.contains("NÃO REGISTRADO"));
        assert!(!html.contains("90 (noventa) dias"));
        assert!(!html.contains("Assinatura do Cliente"));
    }

    #[test]
    fn escapa_html_nos_campos_do_usuario() {
        let mut v = venda_exemplo();
        v.items[0].name = "<b>\"Tela\"</b> & Cia".to_string();
        v.client_name = "A < B".to_string();
        let html = generate_warranty_html(&v);
        assert!(html.contains("&lt;b&gt;&quot;Tela&quot;&lt;/b&gt; &amp; Cia"));
        assert!(html.contains("A &lt; B"));
        assert!(!html.contains("<b>\"Tela\"</b>"));
    }

    #[test]
    fn multi_item_renderiza_linhas_com_imei_inline() {
        let html = generate_warranty_html(&venda_multi_exemplo());
        // Mista usa título genérico + garantia combinada.
        assert!(html.contains("COMPROVANTE DE VENDA"));
        assert!(html.contains("Venda Mista"));
        assert!(html.contains("iPhone 12"));
        assert!(html.contains("Capinha - Silicone"));
        assert!(html.contains("<div class=\"item-imei\">IMEI: 355123456789012</div>"));
        assert!(html.contains("2500,00"));
        assert!(html.contains("40,00"));
        // Sem box standalone (vai inline nas linhas, single ou multi).
        assert!(!html.contains("<div class=\"imei\""));
        assert!(!html.contains("imei missing"));
    }

    #[test]
    fn item_unico_imei_somente_inline() {
        let mut v = venda_exemplo();
        v.sale_type = SaleType::Aparelho;
        v.imei = "355123456789012".to_string();
        v.items[0].imei = "355123456789012".to_string();
        v.items[0].item_type = SaleType::Aparelho;
        let html = generate_warranty_html(&v);
        // Estrutura uniforme: só a sub-linha inline, sem box standalone.
        assert!(html.contains("<div class=\"item-imei\">IMEI: 355123456789012</div>"));
        assert!(!html.contains("<div class=\"imei\""));
        assert!(!html.contains("imei missing"));
        assert_eq!(html.matches("355123456789012").count(), 1);
    }

    #[test]
    fn coluna_item_dinamica_por_tipo() {
        let servico = generate_warranty_html(&venda_exemplo());
        assert!(servico.contains("<th>Serviço Executado</th>"));
        // Mista usa coluna genérica "Item" (linhas têm categorias próprias).
        let mista = generate_warranty_html(&venda_multi_exemplo());
        assert!(mista.contains("<th>Item</th>"));
        let mut aparelho = venda_exemplo();
        aparelho.sale_type = SaleType::Aparelho;
        let aparelho_html = generate_warranty_html(&aparelho);
        assert!(aparelho_html.contains("<th>Modelo do Aparelho</th>"));
        let mut rapida = venda_exemplo();
        rapida.sale_type = SaleType::Rapida;
        rapida.client_name = String::new();
        rapida.phone = String::new();
        let rapida_html = generate_warranty_html(&rapida);
        assert!(rapida_html.contains("<th>Produto / Acessório</th>"));
        for html in [&servico, &mista, &aparelho_html, &rapida_html] {
            assert!(!html.contains("<th>Produto / Serviço</th>"));
        }
    }

    #[test]
    fn exclusoes_bloco_a() {
        // Vocabulário do termo de Aparelho (Bloco A).
        let mut v = venda_exemplo();
        v.sale_type = SaleType::Aparelho;
        v.imei = "355123456789012".to_string();
        v.items[0].item_type = SaleType::Aparelho;
        let html = generate_warranty_html(&v);
        for termo in [
            "mau uso",
            "quedas",
            "tela trincada",
            "umidade/oxidação",
            "descargas elétricas",
            "lacre de garantia violado",
            "bloqueios de software/contas",
        ] {
            assert!(html.contains(termo), "faltando exclusão: {termo}");
        }
        // Vocabulário antigo aposentado.
        assert!(!html.contains("selo de garantia violado"));
        assert!(!html.contains("conta Google"));
    }

    #[test]
    fn aviso_troca_07_dias_no_aparelho_e_mista() {
        const AVISO_A: &str = "TROCA MÁXIMA DE APARELHOS EM ATÉ 07 DIAS APÓS A VENDA MEDIANTE DEFEITO DE FÁBRICA.";
        let servico = generate_warranty_html(&venda_exemplo());
        assert!(!servico.contains(AVISO_A));
        // Aparelho single tem aviso; mista com aparelho também tem.
        let mut aparelho_single = venda_exemplo();
        aparelho_single.sale_type = SaleType::Aparelho;
        aparelho_single.items[0].item_type = SaleType::Aparelho;
        let aparelho = generate_warranty_html(&aparelho_single);
        assert!(aparelho.contains(AVISO_A));
        let mista = generate_warranty_html(&venda_multi_exemplo());
        assert!(mista.contains(AVISO_A));
        assert!(mista.contains("ATENÇÃO CLIENTE:"));
        // Rápida segue compacta: sem garantia, sem aviso.
        let mut rapida = venda_exemplo();
        rapida.sale_type = SaleType::Rapida;
        rapida.client_name = String::new();
        rapida.phone = String::new();
        rapida.items[0].item_type = SaleType::Rapida;
        let rapida_html = generate_warranty_html(&rapida);
        assert!(!rapida_html.contains(AVISO_A));
        assert!(!rapida_html.contains("ATENÇÃO CLIENTE:"));
    }

    #[test]
    fn mista_completa_anexa_a_depois_b() {
        // Serviço + Aparelho: ambos os blocos, A antes de B, + assinaturas.
        let mut v = venda_exemplo();
        v.id = 44;
        v.sale_type = SaleType::Mista;
        v.items = vec![
            crate::models::SaleItem {
                id: 10,
                sale_id: 44,
                name: "Troca de tela".to_string(),
                quantity: 1,
                unit_price: 300.0,
                total_price: 300.0,
                imei: String::new(),
                item_type: SaleType::Servico,
            },
            crate::models::SaleItem {
                id: 11,
                sale_id: 44,
                name: "iPhone 12".to_string(),
                quantity: 1,
                unit_price: 2500.0,
                total_price: 2500.0,
                imei: "355123456789012".to_string(),
                item_type: SaleType::Aparelho,
            },
        ];
        let html = generate_warranty_html(&v);
        let pos_a = html
            .find("TERMO DE GARANTIA - APARELHOS (90 DIAS)")
            .expect("bloco A");
        let pos_b = html
            .find("TERMO DE GARANTIA - SERVIÇOS (90 DIAS)")
            .expect("bloco B");
        assert!(pos_a < pos_b);
        assert!(html.contains("Assinatura do Cliente"));
        // Mista com Serviço: técnico responsável.
        assert!(html.contains("Técnico Responsável"));
        assert!(!html.contains("Vendedor / Atendente"));
    }

    #[test]
    fn mista_sem_servico_assina_atendente() {
        // Aparelho + Acessório (sem Serviço): atendente, nunca técnico.
        let mut v = venda_exemplo();
        v.id = 45;
        v.sale_type = SaleType::Mista;
        v.items = vec![
            crate::models::SaleItem {
                id: 12,
                sale_id: 45,
                name: "iPhone 12".to_string(),
                quantity: 1,
                unit_price: 2500.0,
                total_price: 2500.0,
                imei: "355123456789012".to_string(),
                item_type: SaleType::Aparelho,
            },
            crate::models::SaleItem {
                id: 13,
                sale_id: 45,
                name: "Capinha".to_string(),
                quantity: 1,
                unit_price: 40.0,
                total_price: 40.0,
                imei: String::new(),
                item_type: SaleType::Rapida,
            },
        ];
        let html = generate_warranty_html(&v);
        assert!(html.contains("TERMO DE GARANTIA - APARELHOS (90 DIAS)"));
        assert!(!html.contains("TERMO DE GARANTIA - SERVIÇOS (90 DIAS)"));
        assert!(html.contains("Vendedor / Atendente"));
        assert!(!html.contains("Técnico Responsável"));
    }

    #[test]
    fn troco_exibido_quando_maior_que_zero() {
        let mut v = venda_exemplo();
        v.total_price = 560.0;
        v.paid_amount = 560.0;
        v.valor_recebido = 600.0;
        v.troco = 40.0;
        let html = generate_warranty_html(&v);
        assert!(html.contains("Valor recebido"));
        assert!(html.contains("600,00"));
        assert!(html.contains("<td>Troco</td>"));
        assert!(html.contains("40,00"));
        // Ordem estrita no bloco Pagamento: Total -> Recebido -> Troco,
        // Recebido e Troco visualmente conectados (classes recv/troco).
        let pos_total = html.find("<td>Total</td>").expect("linha Total");
        let pos_recebido = html.find("Valor recebido").expect("linha Recebido");
        let pos_troco = html.find("<td>Troco</td>").expect("linha Troco");
        assert!(pos_total < pos_recebido);
        assert!(pos_recebido < pos_troco);
        assert!(html.contains("<tr class=\"recv\">"));
        assert!(html.contains("<tr class=\"troco\">"));
        // Sem troco: linha omitida.
        let sem_troco = generate_warranty_html(&venda_exemplo());
        assert!(!sem_troco.contains("<td>Troco</td>"));
    }

    #[test]
    fn caminho_do_recibo_por_venda() {
        let p = receipt_path(42);
        assert!(p.to_string_lossy().contains("nextech_recibo_42.html"));
    }
}
