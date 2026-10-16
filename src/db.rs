use anyhow::{Context, Result};
use rusqlite::{Connection, params};
use std::path::PathBuf;

use crate::models::{
    CashEntry, CashSource, Loss, MovimentoCaixa, NewCashEntry, NewLoss, NewSale, PaymentMethod,
    Sale, SaleItem, SaleStatus, SaleType, TipoMovimentoCaixa, agora_local,
    header_type_for, sale_requires_client, status_for, summary_name,
};

pub struct Database {
    conn: Connection,
}

/// Linha agregada do relatório de reposição de estoque.
///
/// Cada sub-modelo permanece uma linha distinta: o agrupamento SQL usa
/// `LOWER(TRIM(sale_items.name))`, que normaliza caixa e espaços das
/// bordas mas preserva modificadores literais ("Pro", "Pro Max",
/// "Plus", "FE", "Ultra", "Mini").
#[derive(Debug, Clone)]
pub struct ItemSalesSummary {
    /// Nome canônico de exibição (`MIN(TRIM(name))`, determinístico).
    pub name: String,
    /// Soma de `quantity` no período.
    pub total_quantity: i64,
    /// Soma de `total_price` no período.
    pub total_revenue: f64,
    /// Nº de vendas distintas (`COUNT(DISTINCT sales.id)`).
    pub sales_count: i64,
}

impl Default for Database {
    fn default() -> Self {
        Self::new().expect("Cannot open SQLite database")
    }
}

/// Nome da pasta do app dentro do diretório de dados local do SO
/// (`%LOCALAPPDATA%` no Windows, `~/.local/share` no Linux).
pub const APP_DATA_DIR_NAME: &str = "NX NEXTECH";
/// Nome do arquivo do banco de produção.
pub const DB_FILE_NAME: &str = "nextech.db";

/// Caminho canônico do banco de produção.
///
/// Windows: `%LOCALAPPDATA%\NX NEXTECH\nextech.db` (via
/// `dirs::data_local_dir()`); demais SOs: equivalente XDG.
/// Se o diretório de dados não for resolvível, cai para
/// `./nextech.db` (modo legado, ex.: CI sem `$HOME`).
#[must_use]
pub fn database_path() -> PathBuf {
    match dirs::data_local_dir() {
        Some(base) => base.join(APP_DATA_DIR_NAME).join(DB_FILE_NAME),
        None => PathBuf::from(DB_FILE_NAME),
    }
}

/// Migração única do banco legado (`./nextech.db`, era `Linux/CWD`):
/// se o destino AppData ainda não existe e o legado existir, copia.
/// Nunca sobrescreve um destino existente; erros de cópia viram `Err`
/// com contexto (o chamador decide: abortar ou seguir sem histórico).
fn migrate_legacy_database(dest: &std::path::Path) -> Result<()> {
    if dest.exists() {
        return Ok(());
    }
    let legacy = PathBuf::from(DB_FILE_NAME);
    if !legacy.exists() {
        return Ok(());
    }
    // Evita auto-cópia quando destino == CWD (fallback sem `dirs`).
    if let (Ok(dst), Ok(src)) = (dest.canonicalize(), legacy.canonicalize())
        && dst == src
    {
        return Ok(());
    }
    std::fs::copy(&legacy, dest).with_context(|| {
        format!(
            "falha ao migrar banco legado {} para {}",
            legacy.display(),
            dest.display()
        )
    })?;
    Ok(())
}

impl Database {
    pub fn new() -> Result<Self> {
        let path = database_path();
        if let Some(parent) = path.parent()
            && !parent.as_os_str().is_empty()
        {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("falha ao criar {}", parent.display()))?;
        }
        migrate_legacy_database(&path)?;
        let conn = Connection::open(&path)
            .with_context(|| format!("Cannot open SQLite database at {}", path.display()))?;
        Ok(Self { conn })
    }

    /// Banco em memória (testes): mesmo esquema de `new()` sem tocar o
    /// arquivo de produção.
    pub fn open_in_memory() -> Result<Self> {
        let conn = Connection::open_in_memory().context("Cannot open in-memory database")?;
        let mut db = Self { conn };
        db.init()?;
        Ok(db)
    }

    pub fn init(&mut self) -> Result<()> {
        self.conn
            .execute_batch(
                r#"
            CREATE TABLE IF NOT EXISTS sales (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                quantity INTEGER NOT NULL,
                unit_price REAL NOT NULL,
                total_price REAL NOT NULL,
                payment_method TEXT NOT NULL,
                paid_amount REAL NOT NULL DEFAULT 0.0,
                initial_paid REAL NOT NULL DEFAULT 0.0,
                client_name TEXT NOT NULL DEFAULT '',
                phone TEXT NOT NULL DEFAULT '',
                imei TEXT NOT NULL DEFAULT '',
                sale_type TEXT NOT NULL DEFAULT 'rapida',
                item_count INTEGER NOT NULL DEFAULT 1,
                status TEXT NOT NULL DEFAULT 'quitado',
                valor_recebido REAL NOT NULL DEFAULT 0.0,
                troco REAL NOT NULL DEFAULT 0.0,
                created_at TEXT NOT NULL DEFAULT (datetime('now', 'localtime'))
            );
            CREATE TABLE IF NOT EXISTS cash_entries (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                amount REAL NOT NULL,
                is_entry INTEGER NOT NULL,
                source TEXT NOT NULL DEFAULT 'drawer',
                description TEXT NOT NULL,
                created_at TEXT NOT NULL DEFAULT (datetime('now', 'localtime'))
            );
            CREATE TABLE IF NOT EXISTS losses (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                amount REAL NOT NULL,
                source TEXT NOT NULL DEFAULT 'drawer',
                description TEXT NOT NULL,
                created_at TEXT NOT NULL DEFAULT (datetime('now', 'localtime'))
            );
            CREATE TABLE IF NOT EXISTS sale_items (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                sale_id INTEGER NOT NULL,
                name TEXT NOT NULL,
                quantity INTEGER NOT NULL,
                unit_price REAL NOT NULL,
                total_price REAL NOT NULL,
                imei TEXT NOT NULL DEFAULT '',
                item_type TEXT NOT NULL DEFAULT 'rapida',
                position INTEGER NOT NULL DEFAULT 0
            );
            CREATE INDEX IF NOT EXISTS idx_sale_items_sale ON sale_items(sale_id);
            "#,
            )
            .map_err(|e| anyhow::anyhow!("Failed to create tables: {}", e))?;
        self.migrate_source_columns()?;
        self.migrate_fiado_columns()?;
        self.migrate_pos_fields()?;
        self.migrate_sale_items()?;
        self.migrate_item_type()?;
        self.migrate_item_count()?;
        self.migrate_troco_columns()?;
        Ok(())
    }

    fn table_has_column(&self, table: &str, column: &str) -> bool {
        let mut stmt = match self.conn.prepare(&format!("PRAGMA table_info({table})")) {
            Ok(s) => s,
            Err(_) => return false,
        };
        let cols: Vec<String> = stmt
            .query_map([], |r| r.get::<_, String>(1))
            .map(|rows| rows.filter_map(|r| r.ok()).collect())
            .unwrap_or_default();
        cols.iter().any(|c| c == column)
    }

    fn migrate_source_columns(&mut self) -> Result<()> {
        if !self.table_has_column("cash_entries", "source") {
            self.conn
                .execute(
                    "ALTER TABLE cash_entries ADD COLUMN source TEXT NOT NULL DEFAULT 'drawer'",
                    [],
                )
                .map_err(|e| anyhow::anyhow!("Failed to migrate cash_entries: {}", e))?;
        }
        if !self.table_has_column("losses", "source") {
            self.conn
                .execute(
                    "ALTER TABLE losses ADD COLUMN source TEXT NOT NULL DEFAULT 'drawer'",
                    [],
                )
                .map_err(|e| anyhow::anyhow!("Failed to migrate losses: {}", e))?;
        }
        Ok(())
    }

    fn migrate_fiado_columns(&mut self) -> Result<()> {
        for (col, ddl) in [
            (
                "paid_amount",
                "ALTER TABLE sales ADD COLUMN paid_amount REAL NOT NULL DEFAULT 0.0",
            ),
            (
                "initial_paid",
                "ALTER TABLE sales ADD COLUMN initial_paid REAL NOT NULL DEFAULT 0.0",
            ),
            (
                "client_name",
                "ALTER TABLE sales ADD COLUMN client_name TEXT NOT NULL DEFAULT ''",
            ),
            (
                "status",
                "ALTER TABLE sales ADD COLUMN status TEXT NOT NULL DEFAULT 'quitado'",
            ),
        ] {
            if !self.table_has_column("sales", col) {
                self.conn
                    .execute(ddl, [])
                    .map_err(|e| anyhow::anyhow!("Failed to migrate sales ({col}): {}", e))?;
            }
        }
        // Backfill: vendas legadas (sem controle de pago) viram quitadas,
        // preservando todos os relatórios e saldos existentes.
        // initial_paid congela o recebido no ato da venda; abatimentos
        // posteriores só entram via cash_entries (sem dupla contagem).
        self.conn
            .execute(
                "UPDATE sales SET paid_amount = total_price, status = 'quitado' WHERE paid_amount = 0 AND total_price > 0 AND status = 'quitado' AND client_name = ''",
                [],
            )
            .map_err(|e| anyhow::anyhow!("Failed to backfill fiado columns: {}", e))?;
        self.conn
            .execute(
                "UPDATE sales SET initial_paid = paid_amount WHERE initial_paid = 0",
                [],
            )
            .map_err(|e| anyhow::anyhow!("Failed to backfill initial_paid: {}", e))?;
        Ok(())
    }

    fn migrate_pos_fields(&mut self) -> Result<()> {
        for (col, ddl) in [
            (
                "sale_type",
                "ALTER TABLE sales ADD COLUMN sale_type TEXT NOT NULL DEFAULT 'rapida'",
            ),
            (
                "phone",
                "ALTER TABLE sales ADD COLUMN phone TEXT NOT NULL DEFAULT ''",
            ),
            (
                "imei",
                "ALTER TABLE sales ADD COLUMN imei TEXT NOT NULL DEFAULT ''",
            ),
        ] {
            if !self.table_has_column("sales", col) {
                self.conn
                    .execute(ddl, [])
                    .map_err(|e| anyhow::anyhow!("Failed to migrate sales ({col}): {}", e))?;
            }
        }
        Ok(())
    }

    /// Garante índice + backfill de `sale_items`: cada venda legada vira
    /// 1 item espelhado do cabeçalho (zero perda de histórico).
    fn migrate_sale_items(&mut self) -> Result<()> {
        // A tabela já nasce no `init` acima; aqui garantimos o índice e o
        // backfill das vendas legadas (1 linha = 1 item, zero perda).
        self.conn
            .execute(
                "CREATE INDEX IF NOT EXISTS idx_sale_items_sale ON sale_items(sale_id)",
                [],
            )
            .map_err(|e| anyhow::anyhow!("Failed to index sale_items: {}", e))?;
        // Backfill herda a categoria da venda-mãe (`sales.sale_type`).
        // Roda antes de `migrate_item_type` para bancos onde a coluna
        // ainda não existia no momento do CREATE (migração de esquema).
        if self.table_has_column("sale_items", "item_type") {
            self.conn
                .execute(
                    "INSERT INTO sale_items (sale_id, name, quantity, unit_price, total_price, imei, item_type, position) SELECT id, name, quantity, unit_price, quantity * unit_price, imei, COALESCE(NULLIF(sale_type, ''), 'rapida'), 0 FROM sales WHERE id NOT IN (SELECT sale_id FROM sale_items)",
                    [],
                )
                .map_err(|e| anyhow::anyhow!("Failed to backfill sale_items: {}", e))?;
        } else {
            self.conn
                .execute(
                    "INSERT INTO sale_items (sale_id, name, quantity, unit_price, total_price, imei, position) SELECT id, name, quantity, unit_price, quantity * unit_price, imei, 0 FROM sales WHERE id NOT IN (SELECT sale_id FROM sale_items)",
                    [],
                )
                .map_err(|e| anyhow::anyhow!("Failed to backfill sale_items: {}", e))?;
        }
        Ok(())
    }

    /// Garante a coluna `sale_items.item_type` (categoria por linha).
    ///
    /// Reuso de `SaleType` para churn mínimo (`rapida` = acessório).
    /// Linhas pré-existentes herdam `sales.sale_type` da venda-mãe.
    fn migrate_item_type(&mut self) -> Result<()> {
        if !self.table_has_column("sale_items", "item_type") {
            self.conn
                .execute(
                    "ALTER TABLE sale_items ADD COLUMN item_type TEXT NOT NULL DEFAULT 'rapida'",
                    [],
                )
                .map_err(|e| anyhow::anyhow!("Failed to migrate sale_items (item_type): {}", e))?;
        }
        self.conn
            .execute(
                "UPDATE sale_items SET item_type = (SELECT COALESCE(NULLIF(sales.sale_type, ''), 'rapida') FROM sales WHERE sales.id = sale_items.sale_id) WHERE item_type IS NULL OR item_type = ''",
                [],
            )
            .map_err(|e| anyhow::anyhow!("Failed to backfill item_type: {}", e))?;
        Ok(())
    }

    fn migrate_item_count(&mut self) -> Result<()> {
        // Contador-resumo para as listas (sem N+1): roda DEPOIS do backfill
        // de `sale_items`, com piso 1 para qualquer resíduo sem linhas.
        if !self.table_has_column("sales", "item_count") {
            self.conn
                .execute(
                    "ALTER TABLE sales ADD COLUMN item_count INTEGER NOT NULL DEFAULT 1",
                    [],
                )
                .map_err(|e| anyhow::anyhow!("Failed to migrate sales (item_count): {}", e))?;
        }
        self.conn
            .execute(
                "UPDATE sales SET item_count = MAX(1, (SELECT COUNT(*) FROM sale_items WHERE sale_items.sale_id = sales.id))",
                [],
            )
            .map_err(|e| anyhow::anyhow!("Failed to backfill item_count: {}", e))?;
        Ok(())
    }

    /// Garante as colunas `sales.valor_recebido` / `sales.troco` (Troco).
    /// Backfill legado: `recebido = paid_amount`, `troco = 0`.
    fn migrate_troco_columns(&mut self) -> Result<()> {
        for (col, ddl) in [
            (
                "valor_recebido",
                "ALTER TABLE sales ADD COLUMN valor_recebido REAL NOT NULL DEFAULT 0.0",
            ),
            (
                "troco",
                "ALTER TABLE sales ADD COLUMN troco REAL NOT NULL DEFAULT 0.0",
            ),
        ] {
            if !self.table_has_column("sales", col) {
                self.conn
                    .execute(ddl, [])
                    .map_err(|e| anyhow::anyhow!("Failed to migrate sales ({col}): {}", e))?;
            }
        }
        self.conn
            .execute(
                "UPDATE sales SET valor_recebido = paid_amount WHERE valor_recebido = 0 AND paid_amount > 0",
                [],
            )
            .map_err(|e| anyhow::anyhow!("Failed to backfill valor_recebido: {}", e))?;
        Ok(())
    }

    /// Registra a venda multi-item multi-categoria e retorna o ID gerado.
    ///
    /// Valida cada linha, soma os subtotais e grava cabeçalho + itens em
    /// uma transação (tudo ou nada). O cabeçalho guarda o resumo de
    /// exibição (`summary_name`) e o tipo agregado (`header_type_for`:
    /// tipo único ou `Mista`); o financeiro usa só o total somado.
    ///
    /// Meio de pagamento removido: `NewSale::payment_method` é ignorado e
    /// a coluna `sales.payment_method` grava `'Cash'` fixo (compat de
    /// esquema; nunca exibido). `sales.sale_type` de entrada é ignorado e
    /// recalculado das linhas.
    ///
    /// Troco (pagamento imediato): `valor_recebido` vazio = exato
    /// (troco 0); preenchido e maior que o total = troco devolvido.
    /// O caixa soma EXATAMENTE o total (`paid = initial = total` na
    /// quitada); o recebido NUNCA entra no saldo.
    /// Fiado/parcial: lógica existente (`paid_amount` = entrada real,
    /// `valor_recebido` ignorado, troco 0).
    ///
    /// # Erros
    ///
    /// Retorna erro se a lista estiver vazia, se qualquer linha for
    /// inválida, se o pago sair do intervalo [0, total], se o recebido
    /// (à vista) for menor que o total, se faltar cliente
    /// (qualquer linha Serviço/Aparelho/Mista, ou fiado parcial/pendente)
    /// ou se faltar IMEI em qualquer linha Aparelho.
    pub fn add_sale(&mut self, sale: &NewSale) -> Result<i64> {
        if sale.items.is_empty() {
            anyhow::bail!("Sale must have at least one item");
        }
        struct ParsedItem {
            name: String,
            quantity: i64,
            unit_price: f64,
            subtotal: f64,
            imei: String,
            item_type: SaleType,
        }
        let mut parsed: Vec<ParsedItem> = Vec::with_capacity(sale.items.len());
        for (idx, item) in sale.items.iter().enumerate() {
            let n = idx + 1;
            let name = item.name.trim().to_string();
            if name.is_empty() {
                anyhow::bail!("Item {n}: name is required");
            }
            let quantity: i64 = item
                .quantity
                .trim()
                .parse()
                .context(format!("Item {n}: invalid quantity"))?;
            if quantity <= 0 {
                anyhow::bail!("Item {n}: invalid quantity");
            }
            let unit_price: f64 = item
                .unit_price
                .trim()
                .replace(',', ".")
                .parse()
                .context(format!("Item {n}: invalid unit price"))?;
            if !unit_price.is_finite() || unit_price <= 0.0 {
                anyhow::bail!("Item {n}: invalid unit price");
            }
            // Rastreio de garantia por-linha: cada item Aparelho exige o
            // próprio IMEI/série; Serviço e acessório têm IMEI opcional.
            // `Mista` nunca aparece como categoria de linha.
            if item.item_type == SaleType::Mista {
                anyhow::bail!("Item {n}: invalid item category");
            }
            let imei = item.imei.trim().to_string();
            if item.item_type.requires_imei() && imei.is_empty() {
                anyhow::bail!("Item {n}: IMEI/serial is required for device items");
            }
            parsed.push(ParsedItem {
                name,
                quantity,
                unit_price,
                subtotal: (quantity as f64) * unit_price,
                imei,
                item_type: item.item_type,
            });
        }
        let total: f64 = parsed.iter().map(|i| i.subtotal).sum();
        // Pagamento removido: ignora `sale.payment_method` (compat UI antiga).
        let header_type = header_type_for(&sale.items);
        // Valor recebido no ato: vazio = quitação integral, exceto venda
        // fiado/parcial (vazio = nada recebido). A chave é `is_fiado`.
        let paid_raw = sale.paid_amount.trim().replace(',', ".");
        let mut paid = if paid_raw.is_empty() {
            if sale.is_fiado { 0.0 } else { total }
        } else {
            paid_raw.parse().context("Invalid paid amount")?
        };
        if !paid.is_finite() {
            anyhow::bail!("Invalid paid amount");
        }
        if paid < 0.0 || paid - total > 1e-6 {
            anyhow::bail!("Paid amount must be between 0 and the total");
        }
        // Clamp de float: pago essencialmente igual ao total quita.
        if (total - paid).abs() < 1e-6 {
            paid = total;
        }
        let status = status_for(total, paid);
        // Troco: só venda imediata quitada (não-fiado, pago == total).
        // Vazio = exato (troco 0). Recebido < total falha; recebido >
        // total gera troco, mas o caixa registra EXATAMENTE o total
        // (`paid/initial` já valem `total` na quitada). Parcial/pendente
        // (fiado ou entrada parcial) ignora o campo, troco sempre 0.
        let (valor_recebido, troco) = if !sale.is_fiado
            && matches!(status, SaleStatus::Quitado)
        {
            let raw = sale.valor_recebido.trim().replace(',', ".");
            if raw.is_empty() {
                (total, 0.0)
            } else {
                let recebido: f64 = raw.parse().context("Invalid received amount")?;
                if !recebido.is_finite() {
                    anyhow::bail!("Invalid received amount");
                }
                if recebido + 1e-6 < total {
                    anyhow::bail!("Received amount is less than the total");
                }
                let troco = (recebido - total).max(0.0);
                (recebido, troco)
            }
        } else {
            (paid, 0.0)
        };
        if !matches!(status, SaleStatus::Quitado) && sale.client_name.trim().is_empty() {
            anyhow::bail!("Client name is required for partial or pending sales");
        }
        // Garantia por-linha: qualquer Serviço/Aparelho/Misto exige cliente
        // mesmo quando quitado. Acessórios puros dispensam.
        if sale_requires_client(&sale.items) && sale.client_name.trim().is_empty() {
            anyhow::bail!("Client name is required for service and device items");
        }
        let total_qty: i64 = parsed.iter().map(|i| i.quantity).sum();
        let display_unit = if total_qty > 0 {
            total / (total_qty as f64)
        } else {
            0.0
        };
        let header_name = summary_name(&parsed[0].name, parsed.len() - 1);
        let first_imei = parsed[0].imei.clone();
        let now = agora_local();
        let tx = self
            .conn
            .transaction()
            .context("Failed to begin transaction")?;
        tx.execute(
            "INSERT INTO sales (name, quantity, unit_price, total_price, payment_method, paid_amount, initial_paid, client_name, phone, imei, sale_type, item_count, status, valor_recebido, troco, created_at) VALUES (?1, ?2, ?3, ?4, 'Cash', ?5, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14)",
            params![
                header_name,
                total_qty,
                display_unit,
                total,
                paid,
                sale.client_name.trim(),
                sale.phone.trim(),
                first_imei,
                header_type.as_str(),
                parsed.len() as i64,
                status.as_str(),
                valor_recebido,
                troco,
                now
            ],
        )?;
        let sale_id = tx.last_insert_rowid();
        for (pos, item) in parsed.iter().enumerate() {
            tx.execute(
                "INSERT INTO sale_items (sale_id, name, quantity, unit_price, total_price, imei, item_type, position) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
                params![
                    sale_id,
                    item.name,
                    item.quantity,
                    item.unit_price,
                    item.subtotal,
                    item.imei,
                    item.item_type.as_str(),
                    pos as i64
                ],
            )?;
        }
        tx.commit().context("Failed to commit sale")?;
        Ok(sale_id)
    }

    /// Exclui a venda e suas linhas em transação (tudo ou nada).
    ///
    /// Totais de caixa/relatórios são somas ao vivo, então se corrigem no
    /// próximo frame. Abates já lançados em `cash_entries` permanecem como
    /// trilha de auditoria do dinheiro que de fato circulou.
    ///
    /// # Erros
    ///
    /// Retorna erro se a venda não existir.
    pub fn delete_sale(&mut self, sale_id: i64) -> Result<()> {
        let tx = self
            .conn
            .transaction()
            .context("Failed to begin transaction")?;
        tx.execute("DELETE FROM sale_items WHERE sale_id = ?1", [sale_id])?;
        let removed = tx.execute("DELETE FROM sales WHERE id = ?1", [sale_id])?;
        if removed == 0 {
            anyhow::bail!("Sale not found");
        }
        tx.commit().context("Failed to commit deletion")?;
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    fn map_sale(
        id: i64,
        name: String,
        quantity: i64,
        unit_price: f64,
        total_price: f64,
        payment_method: String,
        paid_amount: f64,
        client_name: String,
        phone: String,
        imei: String,
        sale_type: String,
        item_count: i64,
        status: String,
        valor_recebido: f64,
        troco: f64,
        created_at: String,
    ) -> Sale {
        Sale {
            id,
            name,
            quantity,
            unit_price,
            total_price,
            payment_method: PaymentMethod::from_str(payment_method.as_str())
                .unwrap_or(PaymentMethod::Cash),
            paid_amount,
            // Legadas sem colunas: recebido = pago, troco 0.
            valor_recebido: if valor_recebido > 0.0 {
                valor_recebido
            } else {
                paid_amount
            },
            troco: troco.max(0.0),
            client_name,
            phone,
            imei,
            sale_type: SaleType::from_str(sale_type.as_str()).unwrap_or(SaleType::Rapida),
            status: SaleStatus::from_str(status.as_str()).unwrap_or(SaleStatus::Quitado),
            created_at,
            // Linhas anexadas pelos leitores de detalhe (`get_sale_by_id`);
            // listas usam só o resumo do cabeçalho (sem N+1).
            items: Vec::new(),
            item_count: item_count.max(1),
        }
    }

    /// Linhas da venda em ordem de lançamento. Se não houver linhas
    /// (venda anterior ao backfill), sintetiza 1 item do cabeçalho para
    /// recibo/busca funcionarem de modo uniforme.
    pub fn get_items_for_sale(&self, sale_id: i64) -> Vec<SaleItem> {
        // `item_type` pode não existir em bancos abertos por versão antiga
        // sem `init` (testes com SQL cru): tenta com a coluna e cai para o
        // SELECT legado, herdando a categoria da venda-mãe.
        let with_type = self.conn.prepare(
            "SELECT id, sale_id, name, quantity, unit_price, total_price, imei, item_type FROM sale_items WHERE sale_id = ?1 ORDER BY position, id",
        );
        if let Ok(mut stmt) = with_type {
            let rows = stmt
                .query_map([sale_id], |r| {
                    let raw: String = r.get(7).unwrap_or_else(|_| "rapida".to_string());
                    Ok(SaleItem {
                        id: r.get(0)?,
                        sale_id: r.get(1)?,
                        name: r.get(2)?,
                        quantity: r.get(3)?,
                        unit_price: r.get(4)?,
                        total_price: r.get(5)?,
                        imei: r.get(6)?,
                        item_type: SaleType::from_str(raw.as_str())
                            .unwrap_or(SaleType::Rapida),
                    })
                })
                .unwrap();
            let items: Vec<SaleItem> = rows.filter_map(|r| r.ok()).collect();
            if !items.is_empty() {
                return items;
            }
        }
        let mut fallback = match self.conn.prepare(
            "SELECT id, sale_id, name, quantity, unit_price, total_price, imei FROM sale_items WHERE sale_id = ?1 ORDER BY position, id",
        ) {
            Ok(s) => s,
            Err(_) => return Vec::new(),
        };
        // Categoria herdada da venda-mãe quando a coluna não existe.
        let parent_type: SaleType = self
            .conn
            .query_row(
                "SELECT sale_type FROM sales WHERE id = ?1",
                [sale_id],
                |r| r.get::<_, String>(0),
            )
            .ok()
            .and_then(|s| SaleType::from_str(s.as_str()))
            .unwrap_or(SaleType::Rapida);
        let rows = fallback
            .query_map([sale_id], |r| {
                Ok(SaleItem {
                    id: r.get(0)?,
                    sale_id: r.get(1)?,
                    name: r.get(2)?,
                    quantity: r.get(3)?,
                    unit_price: r.get(4)?,
                    total_price: r.get(5)?,
                    imei: r.get(6)?,
                    item_type: parent_type,
                })
            })
            .unwrap();
        rows.filter_map(|r| r.ok()).collect()
    }

    pub fn get_sales(&self) -> Vec<Sale> {
        self.get_sales_filtered("")
    }

    pub fn get_sales_filtered(&self, query: &str) -> Vec<Sale> {
        let mut stmt = self.conn.prepare("SELECT id, name, quantity, unit_price, total_price, payment_method, paid_amount, client_name, phone, imei, sale_type, item_count, status, valor_recebido, troco, created_at FROM sales WHERE (?1 = '' OR CAST(id AS TEXT) LIKE '%' || ?1 || '%' OR name LIKE '%' || ?1 || '%' COLLATE NOCASE OR client_name LIKE '%' || ?1 || '%' COLLATE NOCASE OR phone LIKE '%' || ?1 || '%' COLLATE NOCASE OR imei LIKE '%' || ?1 || '%' COLLATE NOCASE OR EXISTS (SELECT 1 FROM sale_items WHERE sale_id = sales.id AND (name LIKE '%' || ?1 || '%' COLLATE NOCASE OR imei LIKE '%' || ?1 || '%' COLLATE NOCASE))) ORDER BY id DESC LIMIT 500").unwrap();
        let rows = stmt
            .query_map([query], |r| {
                Ok(Self::map_sale(
                    r.get(0)?,
                    r.get(1)?,
                    r.get(2)?,
                    r.get(3)?,
                    r.get(4)?,
                    r.get(5)?,
                    r.get(6)?,
                    r.get(7)?,
                    r.get(8)?,
                    r.get(9)?,
                    r.get(10)?,
                    r.get(11)?,
                    r.get(12)?,
                    r.get(13).unwrap_or(0.0),
                    r.get(14).unwrap_or(0.0),
                    r.get(15)?,
                ))
            })
            .unwrap();
        rows.filter_map(|r| r.ok()).collect()
    }

    pub fn get_sale_by_id(&self, id: i64) -> Option<Sale> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, name, quantity, unit_price, total_price, payment_method, paid_amount, client_name, phone, imei, sale_type, item_count, status, valor_recebido, troco, created_at FROM sales WHERE id = ?1")
            .ok()?;
        let mut sale: Sale = stmt
            .query_row([id], |r| {
                Ok(Self::map_sale(
                    r.get(0)?,
                    r.get(1)?,
                    r.get(2)?,
                    r.get(3)?,
                    r.get(4)?,
                    r.get(5)?,
                    r.get(6)?,
                    r.get(7)?,
                    r.get(8)?,
                    r.get(9)?,
                    r.get(10)?,
                    r.get(11)?,
                    r.get(12)?,
                    r.get(13).unwrap_or(0.0),
                    r.get(14).unwrap_or(0.0),
                    r.get(15)?,
                ))
            })
            .ok()?;
        sale.items = self.get_items_for_sale(id);
        if sale.items.is_empty() {
            // Venda anterior ao backfill: sintetiza 1 linha do cabeçalho,
            // herdando a categoria da venda.
            let item_type = if sale.sale_type == SaleType::Mista {
                SaleType::Rapida
            } else {
                sale.sale_type
            };
            sale.items.push(SaleItem {
                id: 0,
                sale_id: sale.id,
                name: sale.name.clone(),
                quantity: sale.quantity,
                unit_price: sale.unit_price,
                total_price: (sale.quantity as f64) * sale.unit_price,
                imei: sale.imei.clone(),
                item_type,
            });
        }
        Some(sale)
    }

    /// Dívidas em aberto (status != 'quitado'), com busca por ID/nome/cliente
    /// e filtro opcional de status. Lista global (sem filtro temporal).
    pub fn get_pending_debts(&self, query: &str, status_filter: Option<&SaleStatus>) -> Vec<Sale> {
        let status_sql = match status_filter {
            Some(s) => s.as_str().to_string(),
            None => String::new(),
        };
        let mut stmt = self.conn.prepare("SELECT id, name, quantity, unit_price, total_price, payment_method, paid_amount, client_name, phone, imei, sale_type, item_count, status, valor_recebido, troco, created_at FROM sales WHERE status != 'quitado' AND (?1 = '' OR status = ?1) AND (?2 = '' OR CAST(id AS TEXT) LIKE '%' || ?2 || '%' OR name LIKE '%' || ?2 || '%' COLLATE NOCASE OR client_name LIKE '%' || ?2 || '%' COLLATE NOCASE OR phone LIKE '%' || ?2 || '%' COLLATE NOCASE OR imei LIKE '%' || ?2 || '%' COLLATE NOCASE OR EXISTS (SELECT 1 FROM sale_items WHERE sale_id = sales.id AND (name LIKE '%' || ?2 || '%' COLLATE NOCASE OR imei LIKE '%' || ?2 || '%' COLLATE NOCASE))) ORDER BY id DESC LIMIT 500").unwrap();
        let rows = stmt
            .query_map(params![status_sql, query], |r| {
                Ok(Self::map_sale(
                    r.get(0)?,
                    r.get(1)?,
                    r.get(2)?,
                    r.get(3)?,
                    r.get(4)?,
                    r.get(5)?,
                    r.get(6)?,
                    r.get(7)?,
                    r.get(8)?,
                    r.get(9)?,
                    r.get(10)?,
                    r.get(11)?,
                    r.get(12)?,
                    r.get(13).unwrap_or(0.0),
                    r.get(14).unwrap_or(0.0),
                    r.get(15)?,
                ))
            })
            .unwrap();
        rows.filter_map(|r| r.ok()).collect()
    }

    /// Registra um abate no fiado: atualiza `paid_amount`/`status` da venda e
    /// lança a entrada correspondente no caixa (dia atual, gaveta). Atômico.
    /// Retorna o novo saldo restante. Falha se valor <= 0 ou maior que o restante.
    ///
    /// Meio de pagamento removido: todo abate é Entrada na gaveta
    /// (`CashSource::Drawer`); `CashSource` segue apenas para lançamentos
    /// manuais de caixa. Descrição sem `(Dinheiro/Cartão/Pix)`.
    pub fn register_abate_fiado_sem_meio(
        &mut self,
        sale_id: i64,
        amount: f64,
    ) -> Result<f64> {
        if !amount.is_finite() || amount <= 0.0 {
            anyhow::bail!("Abate must be greater than zero");
        }
        let source = CashSource::Drawer;
        let tx = self
            .conn
            .transaction()
            .context("Failed to begin transaction")?;
        let (total, paid, client): (f64, f64, String) = tx
            .query_row(
                "SELECT total_price, paid_amount, client_name FROM sales WHERE id = ?1",
                [sale_id],
                |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)),
            )
            .map_err(|_| anyhow::anyhow!("Sale not found"))?;
        let remaining = (total - paid).max(0.0);
        if amount - remaining > 1e-6 {
            anyhow::bail!("Abate exceeds the remaining balance");
        }
        let mut new_paid = paid + amount;
        if (total - new_paid).abs() < 1e-6 || new_paid > total {
            new_paid = total;
        }
        let new_status = status_for(total, new_paid);
        tx.execute(
            "UPDATE sales SET paid_amount = ?1, status = ?2 WHERE id = ?3",
            params![new_paid, new_status.as_str(), sale_id],
        )?;
        let now = agora_local();
        let desc = if client.trim().is_empty() {
            format!("Abate fiado #{sale_id}")
        } else {
            format!("Abate fiado #{sale_id} {}", client.trim())
        };
        tx.execute(
            "INSERT INTO cash_entries (amount, is_entry, source, description, created_at) VALUES (?1, 1, ?2, ?3, ?4)",
            params![amount, source.as_str(), desc, now],
        )?;
        tx.commit().context("Failed to commit abatement")?;
        Ok((total - new_paid).max(0.0))
    }

    /// Shim de compatibilidade com a UI antiga (fase Egui pendente).
    /// Ignora o meio e delega ao fluxo sem meio (sempre gaveta).
    #[deprecated(note = "pagamento removido; use register_abate_fiado_sem_meio")]
    pub fn register_abate_fiado(
        &mut self,
        sale_id: i64,
        amount: f64,
        _method: &PaymentMethod,
    ) -> Result<f64> {
        self.register_abate_fiado_sem_meio(sale_id, amount)
    }

    pub fn add_cash_entry(&mut self, entry: &NewCashEntry, source: &CashSource) -> Result<()> {
        if entry.description.trim().is_empty() {
            anyhow::bail!("Description is required for cash entries");
        }
        let amount: f64 = entry.amount.parse().context("Invalid amount")?;
        let now = agora_local();
        self.conn.execute(
            "INSERT INTO cash_entries (amount, is_entry, source, description, created_at) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                amount,
                if entry.is_entry { 1 } else { 0 },
                source.as_str(),
                entry.description,
                now
            ],
        )?;
        Ok(())
    }

    pub fn get_cash_entries(&self) -> Vec<CashEntry> {
        self.get_cash_entries_filtered("")
    }

    pub fn get_cash_entries_filtered(&self, query: &str) -> Vec<CashEntry> {
        let mut stmt = self.conn.prepare("SELECT id, amount, is_entry, source, description, created_at FROM cash_entries WHERE (?1 = '' OR CAST(id AS TEXT) LIKE '%' || ?1 || '%' OR description LIKE '%' || ?1 || '%' COLLATE NOCASE) ORDER BY id DESC LIMIT 500").unwrap();
        let rows = stmt
            .query_map([query], |r| {
                Ok(CashEntry {
                    id: r.get(0)?,
                    amount: r.get(1)?,
                    is_entry: r.get::<_, i64>(2)? == 1,
                    source: CashSource::from_str(r.get::<_, String>(3)?.as_str())
                        .unwrap_or_default(),
                    description: r.get(4)?,
                    created_at: r.get(5)?,
                })
            })
            .unwrap();
        rows.filter_map(|r| r.ok()).collect()
    }

    pub fn get_cash_entries_filtered_for_day(&self, query: &str, day: &str) -> Vec<CashEntry> {
        let mut stmt = self.conn.prepare("SELECT id, amount, is_entry, source, description, created_at FROM cash_entries WHERE strftime('%Y-%m-%d', created_at) = ?1 AND (?2 = '' OR CAST(id AS TEXT) LIKE '%' || ?2 || '%' OR description LIKE '%' || ?2 || '%' COLLATE NOCASE) ORDER BY id DESC LIMIT 500").unwrap();
        let rows = stmt
            .query_map(params![day, query], |r| {
                Ok(CashEntry {
                    id: r.get(0)?,
                    amount: r.get(1)?,
                    is_entry: r.get::<_, i64>(2)? == 1,
                    source: CashSource::from_str(r.get::<_, String>(3)?.as_str())
                        .unwrap_or_default(),
                    description: r.get(4)?,
                    created_at: r.get(5)?,
                })
            })
            .unwrap();
        rows.filter_map(|r| r.ok()).collect()
    }

    fn map_movimento_caixa(
        id: i64,
        amount: f64,
        tipo: String,
        source: String,
        description: String,
        created_at: String,
    ) -> MovimentoCaixa {
        let kind = match tipo.as_str() {
            "entrada" => TipoMovimentoCaixa::Entrada,
            "saida" => TipoMovimentoCaixa::Saida,
            _ => TipoMovimentoCaixa::Perda,
        };
        MovimentoCaixa {
            id,
            amount,
            tipo: kind,
            source: CashSource::from_str(source.as_str()).unwrap_or_default(),
            description,
            created_at,
        }
    }

    /// Histórico de caixa unificado: entradas, saídas e perdas na mesma
    /// lista, ordenados por data/hora (mais recentes primeiro).
    pub fn get_historico_caixa(&self, query: &str) -> Vec<MovimentoCaixa> {
        let mut stmt = self.conn.prepare("SELECT id, amount, tipo, source, description, created_at FROM (SELECT id, amount, CASE WHEN is_entry = 1 THEN 'entrada' ELSE 'saida' END AS tipo, source, description, created_at FROM cash_entries WHERE (?1 = '' OR CAST(id AS TEXT) LIKE '%' || ?1 || '%' OR description LIKE '%' || ?1 || '%' COLLATE NOCASE) UNION ALL SELECT id, amount, 'perda' AS tipo, source, description, created_at FROM losses WHERE (?1 = '' OR CAST(id AS TEXT) LIKE '%' || ?1 || '%' OR description LIKE '%' || ?1 || '%' COLLATE NOCASE)) ORDER BY created_at DESC, id DESC LIMIT 500").unwrap();
        let rows = stmt
            .query_map([query], |r| {
                Ok(Self::map_movimento_caixa(
                    r.get(0)?,
                    r.get(1)?,
                    r.get(2)?,
                    r.get(3)?,
                    r.get(4)?,
                    r.get(5)?,
                ))
            })
            .unwrap();
        rows.filter_map(|r| r.ok()).collect()
    }

    /// Histórico de caixa unificado filtrado por dia (AAAA-MM-DD).
    pub fn get_historico_caixa_for_day(&self, query: &str, day: &str) -> Vec<MovimentoCaixa> {
        let mut stmt = self.conn.prepare("SELECT id, amount, tipo, source, description, created_at FROM (SELECT id, amount, CASE WHEN is_entry = 1 THEN 'entrada' ELSE 'saida' END AS tipo, source, description, created_at FROM cash_entries WHERE strftime('%Y-%m-%d', created_at) = ?1 AND (?2 = '' OR CAST(id AS TEXT) LIKE '%' || ?2 || '%' OR description LIKE '%' || ?2 || '%' COLLATE NOCASE) UNION ALL SELECT id, amount, 'perda' AS tipo, source, description, created_at FROM losses WHERE strftime('%Y-%m-%d', created_at) = ?1 AND (?2 = '' OR CAST(id AS TEXT) LIKE '%' || ?2 || '%' OR description LIKE '%' || ?2 || '%' COLLATE NOCASE)) ORDER BY created_at DESC, id DESC LIMIT 500").unwrap();
        let rows = stmt
            .query_map(params![day, query], |r| {
                Ok(Self::map_movimento_caixa(
                    r.get(0)?,
                    r.get(1)?,
                    r.get(2)?,
                    r.get(3)?,
                    r.get(4)?,
                    r.get(5)?,
                ))
            })
            .unwrap();
        rows.filter_map(|r| r.ok()).collect()
    }

    fn cash_entries_in_period(&self, sql: &str, period: &str) -> Vec<CashEntry> {
        let mut stmt = self.conn.prepare(sql).unwrap();
        let rows = stmt
            .query_map([period], |r| {
                Ok(CashEntry {
                    id: r.get(0)?,
                    amount: r.get(1)?,
                    is_entry: r.get::<_, i64>(2)? == 1,
                    source: CashSource::from_str(r.get::<_, String>(3)?.as_str())
                        .unwrap_or_default(),
                    description: r.get(4)?,
                    created_at: r.get(5)?,
                })
            })
            .unwrap();
        rows.filter_map(|r| r.ok()).collect()
    }

    pub fn get_cash_entries_for_day(&self, day: &str) -> Vec<CashEntry> {
        self.cash_entries_in_period(
            "SELECT id, amount, is_entry, source, description, created_at FROM cash_entries WHERE strftime('%Y-%m-%d', created_at) = ?1 ORDER BY id DESC LIMIT 500",
            day,
        )
    }

    pub fn get_cash_entries_for_month(&self, year_month: &str) -> Vec<CashEntry> {
        self.cash_entries_in_period(
            "SELECT id, amount, is_entry, source, description, created_at FROM cash_entries WHERE strftime('%Y-%m', created_at) = ?1 ORDER BY id DESC LIMIT 500",
            year_month,
        )
    }

    pub fn get_cash_entries_for_year(&self, year: &str) -> Vec<CashEntry> {
        self.cash_entries_in_period(
            "SELECT id, amount, is_entry, source, description, created_at FROM cash_entries WHERE strftime('%Y', created_at) = ?1 ORDER BY id DESC LIMIT 500",
            year,
        )
    }

    pub fn add_loss(&mut self, loss: &NewLoss, source: &CashSource) -> Result<()> {
        if loss.description.trim().is_empty() {
            anyhow::bail!("Description is required for losses");
        }
        let amount: f64 = loss.amount.parse().context("Invalid amount")?;
        let now = agora_local();
        self.conn.execute(
            "INSERT INTO losses (amount, source, description, created_at) VALUES (?1, ?2, ?3, ?4)",
            params![amount, source.as_str(), loss.description, now],
        )?;
        Ok(())
    }

    pub fn get_losses(&self) -> Vec<Loss> {
        self.get_losses_filtered("")
    }

    pub fn get_losses_filtered(&self, query: &str) -> Vec<Loss> {
        let mut stmt = self
            .conn
            .prepare(
                "SELECT id, amount, source, description, created_at FROM losses WHERE (?1 = '' OR CAST(id AS TEXT) LIKE '%' || ?1 || '%' OR description LIKE '%' || ?1 || '%' COLLATE NOCASE) ORDER BY id DESC LIMIT 500",
            )
            .unwrap();
        let rows = stmt
            .query_map([query], |r| {
                Ok(Loss {
                    id: r.get(0)?,
                    amount: r.get(1)?,
                    source: CashSource::from_str(r.get::<_, String>(2)?.as_str())
                        .unwrap_or_default(),
                    description: r.get(3)?,
                    created_at: r.get(4)?,
                })
            })
            .unwrap();
        rows.filter_map(|r| r.ok()).collect()
    }

    pub fn get_losses_filtered_for_day(&self, query: &str, day: &str) -> Vec<Loss> {
        let mut stmt = self
            .conn
            .prepare(
                "SELECT id, amount, source, description, created_at FROM losses WHERE strftime('%Y-%m-%d', created_at) = ?1 AND (?2 = '' OR CAST(id AS TEXT) LIKE '%' || ?2 || '%' OR description LIKE '%' || ?2 || '%' COLLATE NOCASE) ORDER BY id DESC LIMIT 500",
            )
            .unwrap();
        let rows = stmt
            .query_map(params![day, query], |r| {
                Ok(Loss {
                    id: r.get(0)?,
                    amount: r.get(1)?,
                    source: CashSource::from_str(r.get::<_, String>(2)?.as_str())
                        .unwrap_or_default(),
                    description: r.get(3)?,
                    created_at: r.get(4)?,
                })
            })
            .unwrap();
        rows.filter_map(|r| r.ok()).collect()
    }

    fn losses_in_period(&self, sql: &str, period: &str) -> Vec<Loss> {
        let mut stmt = self.conn.prepare(sql).unwrap();
        let rows = stmt
            .query_map([period], |r| {
                Ok(Loss {
                    id: r.get(0)?,
                    amount: r.get(1)?,
                    source: CashSource::from_str(r.get::<_, String>(2)?.as_str())
                        .unwrap_or_default(),
                    description: r.get(3)?,
                    created_at: r.get(4)?,
                })
            })
            .unwrap();
        rows.filter_map(|r| r.ok()).collect()
    }

    pub fn get_losses_for_day(&self, day: &str) -> Vec<Loss> {
        self.losses_in_period(
            "SELECT id, amount, source, description, created_at FROM losses WHERE strftime('%Y-%m-%d', created_at) = ?1 ORDER BY id DESC LIMIT 500",
            day,
        )
    }

    pub fn get_losses_for_month(&self, year_month: &str) -> Vec<Loss> {
        self.losses_in_period(
            "SELECT id, amount, source, description, created_at FROM losses WHERE strftime('%Y-%m', created_at) = ?1 ORDER BY id DESC LIMIT 500",
            year_month,
        )
    }

    pub fn get_losses_for_year(&self, year: &str) -> Vec<Loss> {
        self.losses_in_period(
            "SELECT id, amount, source, description, created_at FROM losses WHERE strftime('%Y', created_at) = ?1 ORDER BY id DESC LIMIT 500",
            year,
        )
    }

    pub fn total_sales(&self) -> f64 {
        let mut stmt = self
            .conn
            .prepare("SELECT COALESCE(SUM(total_price), 0.0) FROM sales")
            .unwrap();
        let row: rusqlite::Result<f64> = stmt.query_row([], |r| r.get(0));
        row.unwrap_or(0.0)
    }
    /// Shim legado: meio removido; mantido para a UI antiga compilar.
    /// Será excluído na fase Egui (relatórios passam a revenue-only).
    #[deprecated(note = "pagamento removido dos relatórios")]
    pub fn total_sales_by_method(&self, method: &PaymentMethod) -> f64 {
        let pm = method.as_str();
        let mut stmt = self
            .conn
            .prepare("SELECT COALESCE(SUM(total_price), 0.0) FROM sales WHERE payment_method = ?1")
            .unwrap();
        let row: rusqlite::Result<f64> = stmt.query_row([pm], |r| r.get(0));
        row.unwrap_or(0.0)
    }
    pub fn total_cash_in(&self) -> f64 {
        let mut stmt = self
            .conn
            .prepare("SELECT COALESCE(SUM(amount), 0.0) FROM cash_entries WHERE is_entry = 1")
            .unwrap();
        let row: rusqlite::Result<f64> = stmt.query_row([], |r| r.get(0));
        row.unwrap_or(0.0)
    }
    pub fn total_cash_out(&self) -> f64 {
        let mut stmt = self
            .conn
            .prepare("SELECT COALESCE(SUM(amount), 0.0) FROM cash_entries WHERE is_entry = 0")
            .unwrap();
        let row: rusqlite::Result<f64> = stmt.query_row([], |r| r.get(0));
        row.unwrap_or(0.0)
    }
    pub fn total_losses(&self) -> f64 {
        let mut stmt = self
            .conn
            .prepare("SELECT COALESCE(SUM(amount), 0.0) FROM losses")
            .unwrap();
        let row: rusqlite::Result<f64> = stmt.query_row([], |r| r.get(0));
        row.unwrap_or(0.0)
    }

    fn sum_for_period(&self, sql: &str, period: &str) -> f64 {
        let stmt = self.conn.prepare(sql);
        let mut stmt = match stmt {
            Ok(s) => s,
            Err(_) => return 0.0,
        };
        let row: rusqlite::Result<f64> = stmt.query_row([period], |r| r.get(0));
        row.unwrap_or(0.0)
    }

    fn sum_for_source(&self, sql: &str, source: &str) -> f64 {
        let stmt = self.conn.prepare(sql);
        let mut stmt = match stmt {
            Ok(s) => s,
            Err(_) => return 0.0,
        };
        let row: rusqlite::Result<f64> = stmt.query_row([source], |r| r.get(0));
        row.unwrap_or(0.0)
    }

    fn sum_for_source_period(&self, sql: &str, source: &str, period: &str) -> f64 {
        let stmt = self.conn.prepare(sql);
        let mut stmt = match stmt {
            Ok(s) => s,
            Err(_) => return 0.0,
        };
        let row: rusqlite::Result<f64> =
            stmt.query_row(rusqlite::params![source, period], |r| r.get(0));
        row.unwrap_or(0.0)
    }

    pub fn total_cash_in_by_source(&self, source: &CashSource) -> f64 {
        self.sum_for_source(
            "SELECT COALESCE(SUM(amount), 0.0) FROM cash_entries WHERE is_entry = 1 AND source = ?1",
            source.as_str(),
        )
    }

    pub fn total_cash_out_by_source(&self, source: &CashSource) -> f64 {
        self.sum_for_source(
            "SELECT COALESCE(SUM(amount), 0.0) FROM cash_entries WHERE is_entry = 0 AND source = ?1",
            source.as_str(),
        )
    }

    pub fn total_losses_by_source(&self, source: &CashSource) -> f64 {
        self.sum_for_source(
            "SELECT COALESCE(SUM(amount), 0.0) FROM losses WHERE source = ?1",
            source.as_str(),
        )
    }

    pub fn total_cash_in_by_source_for_month(&self, source: &CashSource, year_month: &str) -> f64 {
        self.sum_for_source_period(
            "SELECT COALESCE(SUM(amount), 0.0) FROM cash_entries WHERE is_entry = 1 AND source = ?1 AND strftime('%Y-%m', created_at) = ?2",
            source.as_str(),
            year_month,
        )
    }

    pub fn total_cash_out_by_source_for_month(&self, source: &CashSource, year_month: &str) -> f64 {
        self.sum_for_source_period(
            "SELECT COALESCE(SUM(amount), 0.0) FROM cash_entries WHERE is_entry = 0 AND source = ?1 AND strftime('%Y-%m', created_at) = ?2",
            source.as_str(),
            year_month,
        )
    }

    pub fn total_losses_by_source_for_month(&self, source: &CashSource, year_month: &str) -> f64 {
        self.sum_for_source_period(
            "SELECT COALESCE(SUM(amount), 0.0) FROM losses WHERE source = ?1 AND strftime('%Y-%m', created_at) = ?2",
            source.as_str(),
            year_month,
        )
    }

    pub fn total_cash_in_by_source_for_year(&self, source: &CashSource, year: &str) -> f64 {
        self.sum_for_source_period(
            "SELECT COALESCE(SUM(amount), 0.0) FROM cash_entries WHERE is_entry = 1 AND source = ?1 AND strftime('%Y', created_at) = ?2",
            source.as_str(),
            year,
        )
    }

    pub fn total_cash_out_by_source_for_year(&self, source: &CashSource, year: &str) -> f64 {
        self.sum_for_source_period(
            "SELECT COALESCE(SUM(amount), 0.0) FROM cash_entries WHERE is_entry = 0 AND source = ?1 AND strftime('%Y', created_at) = ?2",
            source.as_str(),
            year,
        )
    }

    pub fn total_losses_by_source_for_year(&self, source: &CashSource, year: &str) -> f64 {
        self.sum_for_source_period(
            "SELECT COALESCE(SUM(amount), 0.0) FROM losses WHERE source = ?1 AND strftime('%Y', created_at) = ?2",
            source.as_str(),
            year,
        )
    }

    pub fn total_cash_in_by_source_for_day(&self, source: &CashSource, day: &str) -> f64 {
        self.sum_for_source_period(
            "SELECT COALESCE(SUM(amount), 0.0) FROM cash_entries WHERE is_entry = 1 AND source = ?1 AND strftime('%Y-%m-%d', created_at) = ?2",
            source.as_str(),
            day,
        )
    }

    pub fn total_cash_out_by_source_for_day(&self, source: &CashSource, day: &str) -> f64 {
        self.sum_for_source_period(
            "SELECT COALESCE(SUM(amount), 0.0) FROM cash_entries WHERE is_entry = 0 AND source = ?1 AND strftime('%Y-%m-%d', created_at) = ?2",
            source.as_str(),
            day,
        )
    }

    pub fn total_losses_by_source_for_day(&self, source: &CashSource, day: &str) -> f64 {
        self.sum_for_source_period(
            "SELECT COALESCE(SUM(amount), 0.0) FROM losses WHERE source = ?1 AND strftime('%Y-%m-%d', created_at) = ?2",
            source.as_str(),
            day,
        )
    }

    pub fn total_sales_for_month(&self, year_month: &str) -> f64 {
        self.sum_for_period(
            "SELECT COALESCE(SUM(total_price), 0.0) FROM sales WHERE strftime('%Y-%m', created_at) = ?1",
            year_month,
        )
    }

    pub fn total_sales_for_year(&self, year: &str) -> f64 {
        self.sum_for_period(
            "SELECT COALESCE(SUM(total_price), 0.0) FROM sales WHERE strftime('%Y', created_at) = ?1",
            year,
        )
    }

    pub fn total_cash_in_for_month(&self, year_month: &str) -> f64 {
        self.sum_for_period(
            "SELECT COALESCE(SUM(amount), 0.0) FROM cash_entries WHERE is_entry = 1 AND strftime('%Y-%m', created_at) = ?1",
            year_month,
        )
    }

    pub fn total_cash_out_for_month(&self, year_month: &str) -> f64 {
        self.sum_for_period(
            "SELECT COALESCE(SUM(amount), 0.0) FROM cash_entries WHERE is_entry = 0 AND strftime('%Y-%m', created_at) = ?1",
            year_month,
        )
    }

    pub fn total_losses_for_month(&self, year_month: &str) -> f64 {
        self.sum_for_period(
            "SELECT COALESCE(SUM(amount), 0.0) FROM losses WHERE strftime('%Y-%m', created_at) = ?1",
            year_month,
        )
    }

    pub fn total_cash_in_for_year(&self, year: &str) -> f64 {
        self.sum_for_period(
            "SELECT COALESCE(SUM(amount), 0.0) FROM cash_entries WHERE is_entry = 1 AND strftime('%Y', created_at) = ?1",
            year,
        )
    }

    pub fn total_cash_out_for_year(&self, year: &str) -> f64 {
        self.sum_for_period(
            "SELECT COALESCE(SUM(amount), 0.0) FROM cash_entries WHERE is_entry = 0 AND strftime('%Y', created_at) = ?1",
            year,
        )
    }

    pub fn total_losses_for_year(&self, year: &str) -> f64 {
        self.sum_for_period(
            "SELECT COALESCE(SUM(amount), 0.0) FROM losses WHERE strftime('%Y', created_at) = ?1",
            year,
        )
    }

    pub fn net_balance_for_month(&self, year_month: &str) -> f64 {
        self.total_initial_received_for_month(year_month) + self.total_cash_in_for_month(year_month)
            - self.total_cash_out_for_month(year_month)
            - self.total_losses_for_month(year_month)
    }

    pub fn net_balance_for_year(&self, year: &str) -> f64 {
        self.total_initial_received_for_year(year) + self.total_cash_in_for_year(year)
            - self.total_cash_out_for_year(year)
            - self.total_losses_for_year(year)
    }

    pub fn sales_by_month(&self) -> Vec<(String, f64)> {
        let mut stmt = self
            .conn
            .prepare(
                "SELECT strftime('%Y-%m', created_at) AS m, COALESCE(SUM(total_price), 0.0) FROM sales WHERE created_at IS NOT NULL GROUP BY m ORDER BY m DESC",
            )
            .unwrap();
        let rows = stmt
            .query_map([], |r| Ok((r.get::<_, String>(0)?, r.get::<_, f64>(1)?)))
            .unwrap();
        rows.filter_map(|r| r.ok()).collect()
    }

    pub fn sales_by_year(&self) -> Vec<(String, f64)> {
        let mut stmt = self
            .conn
            .prepare(
                "SELECT strftime('%Y', created_at) AS y, COALESCE(SUM(total_price), 0.0) FROM sales WHERE created_at IS NOT NULL GROUP BY y ORDER BY y DESC",
            )
            .unwrap();
        let rows = stmt
            .query_map([], |r| Ok((r.get::<_, String>(0)?, r.get::<_, f64>(1)?)))
            .unwrap();
        rows.filter_map(|r| r.ok()).collect()
    }

    pub fn distinct_months(&self) -> Vec<String> {
        let mut stmt = self
            .conn
            .prepare(
                "SELECT m FROM (SELECT DISTINCT strftime('%Y-%m', created_at) AS m FROM sales UNION SELECT DISTINCT strftime('%Y-%m', created_at) FROM cash_entries UNION SELECT DISTINCT strftime('%Y-%m', created_at) FROM losses) WHERE m IS NOT NULL ORDER BY m DESC",
            )
            .unwrap();
        let rows = stmt.query_map([], |r| r.get::<_, String>(0)).unwrap();
        rows.filter_map(|r| r.ok()).collect()
    }

    pub fn distinct_years(&self) -> Vec<String> {
        let mut stmt = self
            .conn
            .prepare(
                "SELECT y FROM (SELECT DISTINCT strftime('%Y', created_at) AS y FROM sales UNION SELECT DISTINCT strftime('%Y', created_at) FROM cash_entries UNION SELECT DISTINCT strftime('%Y', created_at) FROM losses) WHERE y IS NOT NULL ORDER BY y DESC",
            )
            .unwrap();
        let rows = stmt.query_map([], |r| r.get::<_, String>(0)).unwrap();
        rows.filter_map(|r| r.ok()).collect()
    }

    /// Recebido no ato em dinheiro: meio removido, todas as vendas novas
    /// gravam `'Cash'` fixo, logo equivale a `total_initial_received()`.
    /// Mantido para o cálculo da gaveta até a limpeza final dos shims.
    pub fn total_cash_sales(&self) -> f64 {
        self.total_initial_received()
    }

    pub fn total_cash_sales_for_day(&self, day: &str) -> f64 {
        self.sum_for_period(
            "SELECT COALESCE(SUM(initial_paid), 0.0) FROM sales WHERE payment_method = 'Cash' AND strftime('%Y-%m-%d', created_at) = ?1",
            day,
        )
    }

    pub fn total_cash_sales_for_month(&self, year_month: &str) -> f64 {
        self.sum_for_period(
            "SELECT COALESCE(SUM(initial_paid), 0.0) FROM sales WHERE payment_method = 'Cash' AND strftime('%Y-%m', created_at) = ?1",
            year_month,
        )
    }

    pub fn total_cash_sales_for_year(&self, year: &str) -> f64 {
        self.sum_for_period(
            "SELECT COALESCE(SUM(initial_paid), 0.0) FROM sales WHERE payment_method = 'Cash' AND strftime('%Y', created_at) = ?1",
            year,
        )
    }

    pub fn cash_in_drawer(&self) -> f64 {
        let drawer = CashSource::Drawer;
        self.total_cash_sales() + self.total_cash_in_by_source(&drawer)
            - self.total_cash_out_by_source(&drawer)
            - self.total_losses_by_source(&drawer)
    }

    pub fn cash_in_drawer_for_day(&self, day: &str) -> f64 {
        let drawer = CashSource::Drawer;
        self.total_cash_sales_for_day(day) + self.total_cash_in_by_source_for_day(&drawer, day)
            - self.total_cash_out_by_source_for_day(&drawer, day)
            - self.total_losses_by_source_for_day(&drawer, day)
    }

    pub fn cash_in_drawer_for_month(&self, year_month: &str) -> f64 {
        let drawer = CashSource::Drawer;
        self.total_cash_sales_for_month(year_month)
            + self.total_cash_in_by_source_for_month(&drawer, year_month)
            - self.total_cash_out_by_source_for_month(&drawer, year_month)
            - self.total_losses_by_source_for_month(&drawer, year_month)
    }

    pub fn cash_in_drawer_for_year(&self, year: &str) -> f64 {
        let drawer = CashSource::Drawer;
        self.total_cash_sales_for_year(year) + self.total_cash_in_by_source_for_year(&drawer, year)
            - self.total_cash_out_by_source_for_year(&drawer, year)
            - self.total_losses_by_source_for_year(&drawer, year)
    }

    pub fn total_sales_for_day(&self, day: &str) -> f64 {
        self.sum_for_period(
            "SELECT COALESCE(SUM(total_price), 0.0) FROM sales WHERE strftime('%Y-%m-%d', created_at) = ?1",
            day,
        )
    }

    pub fn total_cash_in_for_day(&self, day: &str) -> f64 {
        self.sum_for_period(
            "SELECT COALESCE(SUM(amount), 0.0) FROM cash_entries WHERE is_entry = 1 AND strftime('%Y-%m-%d', created_at) = ?1",
            day,
        )
    }

    pub fn total_cash_out_for_day(&self, day: &str) -> f64 {
        self.sum_for_period(
            "SELECT COALESCE(SUM(amount), 0.0) FROM cash_entries WHERE is_entry = 0 AND strftime('%Y-%m-%d', created_at) = ?1",
            day,
        )
    }

    pub fn total_losses_for_day(&self, day: &str) -> f64 {
        self.sum_for_period(
            "SELECT COALESCE(SUM(amount), 0.0) FROM losses WHERE strftime('%Y-%m-%d', created_at) = ?1",
            day,
        )
    }

    pub fn net_balance_for_day(&self, day: &str) -> f64 {
        self.total_initial_received_for_day(day) + self.total_cash_in_for_day(day)
            - self.total_cash_out_for_day(day)
            - self.total_losses_for_day(day)
    }

    /// Saldo geral = recebido no ato + entradas − saídas − perdas (Opção A).
    /// Usa `initial_paid` para não contar abates duas vezes (paid + ledger).
    pub fn net_balance_global(&self) -> f64 {
        self.total_initial_received() + self.total_cash_in()
            - self.total_cash_out()
            - self.total_losses()
    }

    fn count_for_period(&self, sql: &str, period: &str) -> i64 {
        let stmt = self.conn.prepare(sql);
        let mut stmt = match stmt {
            Ok(s) => s,
            Err(_) => return 0,
        };
        let row: rusqlite::Result<i64> = stmt.query_row([period], |r| r.get(0));
        row.unwrap_or(0)
    }

    pub fn count_sales(&self) -> i64 {
        let mut stmt = self.conn.prepare("SELECT COUNT(*) FROM sales").unwrap();
        let row: rusqlite::Result<i64> = stmt.query_row([], |r| r.get(0));
        row.unwrap_or(0)
    }

    pub fn count_sales_for_day(&self, day: &str) -> i64 {
        self.count_for_period(
            "SELECT COUNT(*) FROM sales WHERE strftime('%Y-%m-%d', created_at) = ?1",
            day,
        )
    }

    pub fn count_sales_for_month(&self, year_month: &str) -> i64 {
        self.count_for_period(
            "SELECT COUNT(*) FROM sales WHERE strftime('%Y-%m', created_at) = ?1",
            year_month,
        )
    }

    pub fn count_sales_for_year(&self, year: &str) -> i64 {
        self.count_for_period(
            "SELECT COUNT(*) FROM sales WHERE strftime('%Y', created_at) = ?1",
            year,
        )
    }

    fn avg_ticket_for_sql(&self, sql: &str) -> f64 {
        let stmt = self.conn.prepare(sql);
        let mut stmt = match stmt {
            Ok(s) => s,
            Err(_) => return 0.0,
        };
        let row: rusqlite::Result<f64> = stmt.query_row([], |r| r.get(0));
        row.unwrap_or(0.0)
    }

    fn avg_ticket_for_period(&self, sql: &str, period: &str) -> f64 {
        let stmt = self.conn.prepare(sql);
        let mut stmt = match stmt {
            Ok(s) => s,
            Err(_) => return 0.0,
        };
        let row: rusqlite::Result<f64> = stmt.query_row([period], |r| r.get(0));
        row.unwrap_or(0.0)
    }

    pub fn average_ticket(&self) -> f64 {
        self.avg_ticket_for_sql("SELECT COALESCE(AVG(total_price), 0.0) FROM sales")
    }

    pub fn average_ticket_for_day(&self, day: &str) -> f64 {
        self.avg_ticket_for_period(
            "SELECT COALESCE(AVG(total_price), 0.0) FROM sales WHERE strftime('%Y-%m-%d', created_at) = ?1",
            day,
        )
    }

    pub fn average_ticket_for_month(&self, year_month: &str) -> f64 {
        self.avg_ticket_for_period(
            "SELECT COALESCE(AVG(total_price), 0.0) FROM sales WHERE strftime('%Y-%m', created_at) = ?1",
            year_month,
        )
    }

    pub fn average_ticket_for_year(&self, year: &str) -> f64 {
        self.avg_ticket_for_period(
            "SELECT COALESCE(AVG(total_price), 0.0) FROM sales WHERE strftime('%Y', created_at) = ?1",
            year,
        )
    }

    pub fn distinct_days(&self) -> Vec<String> {
        let mut stmt = self
            .conn
            .prepare(
                "SELECT d FROM (SELECT DISTINCT strftime('%Y-%m-%d', created_at) AS d FROM sales UNION SELECT DISTINCT strftime('%Y-%m-%d', created_at) FROM cash_entries UNION SELECT DISTINCT strftime('%Y-%m-%d', created_at) FROM losses) WHERE d IS NOT NULL ORDER BY d DESC",
            )
            .unwrap();
        let rows = stmt.query_map([], |r| r.get::<_, String>(0)).unwrap();
        rows.filter_map(|r| r.ok()).collect()
    }

    fn sales_in_period(&self, sql: &str, period: &str) -> Vec<Sale> {
        let mut stmt = self.conn.prepare(sql).unwrap();
        let rows = stmt
            .query_map([period], |r| {
                Ok(Self::map_sale(
                    r.get(0)?,
                    r.get(1)?,
                    r.get(2)?,
                    r.get(3)?,
                    r.get(4)?,
                    r.get(5)?,
                    r.get(6)?,
                    r.get(7)?,
                    r.get(8)?,
                    r.get(9)?,
                    r.get(10)?,
                    r.get(11)?,
                    r.get(12)?,
                    r.get(13).unwrap_or(0.0),
                    r.get(14).unwrap_or(0.0),
                    r.get(15)?,
                ))
            })
            .unwrap();
        rows.filter_map(|r| r.ok()).collect()
    }

    pub fn get_sales_for_day(&self, day: &str) -> Vec<Sale> {
        self.sales_in_period(
            "SELECT id, name, quantity, unit_price, total_price, payment_method, paid_amount, client_name, phone, imei, sale_type, item_count, status, valor_recebido, troco, created_at FROM sales WHERE strftime('%Y-%m-%d', created_at) = ?1 ORDER BY id DESC LIMIT 500",
            day,
        )
    }

    pub fn get_sales_for_month(&self, year_month: &str) -> Vec<Sale> {
        self.sales_in_period(
            "SELECT id, name, quantity, unit_price, total_price, payment_method, paid_amount, client_name, phone, imei, sale_type, item_count, status, valor_recebido, troco, created_at FROM sales WHERE strftime('%Y-%m', created_at) = ?1 ORDER BY id DESC LIMIT 500",
            year_month,
        )
    }

    pub fn get_sales_for_year(&self, year: &str) -> Vec<Sale> {
        self.sales_in_period(
            "SELECT id, name, quantity, unit_price, total_price, payment_method, paid_amount, client_name, phone, imei, sale_type, item_count, status, valor_recebido, troco, created_at FROM sales WHERE strftime('%Y', created_at) = ?1 ORDER BY id DESC LIMIT 500",
            year,
        )
    }

    /// Agregação de itens para o relatório de reposição de estoque.
    ///
    /// Agrupa `sale_items` por `LOWER(TRIM(name))` no intervalo inclusivo
    /// `[start_date, end_date]` (dias `AAAA-MM-DD` sobre `sales.created_at`),
    /// somando quantidade e faturamento. Sub-modelos distintos
    /// ("iPhone 12" vs "iPhone 12 Pro Max") nunca colapsam, pois os
    /// modificadores literais permanecem na chave de agrupamento.
    /// `sale_type` (`None` = Todos) filtra pela categoria DA LINHA
    /// (`sale_items.item_type`); `Some(Mista)` filtra vendas de cabeçalho
    /// misto (`sales.sale_type = 'mista'`); `None` agrega tudo.
    ///
    /// # Erros
    ///
    /// Retorna erro se a consulta SQL falhar.
    pub fn get_aggregated_item_sales(
        &self,
        start_date: &str,
        end_date: &str,
        sale_type: Option<SaleType>,
    ) -> Result<Vec<ItemSalesSummary>> {
        // `Mista` como filtro = vendas mistas (cabeçalho); demais tipos =
        // categoria da linha. `None` = todas.
        let (item_filter, sale_filter) = match sale_type {
            None => (String::new(), String::new()),
            Some(SaleType::Mista) => (String::new(), SaleType::Mista.as_str().to_string()),
            Some(t) => (t.as_str().to_string(), String::new()),
        };
        let mut stmt = self.conn.prepare(
            "SELECT MIN(TRIM(si.name)) AS name, SUM(si.quantity) AS total_quantity, SUM(si.total_price) AS total_revenue, COUNT(DISTINCT s.id) AS sales_count FROM sale_items si JOIN sales s ON s.id = si.sale_id WHERE strftime('%Y-%m-%d', s.created_at) BETWEEN ?1 AND ?2 AND (?3 = '' OR si.item_type = ?3) AND (?4 = '' OR s.sale_type = ?4) GROUP BY LOWER(TRIM(si.name)) ORDER BY total_quantity DESC",
        )?;
        let rows = stmt
            .query_map(params![start_date, end_date, item_filter, sale_filter], |r| {
                Ok(ItemSalesSummary {
                    name: r.get(0)?,
                    total_quantity: r.get(1)?,
                    total_revenue: r.get(2)?,
                    sales_count: r.get(3)?,
                })
            })
            .context("Failed to aggregate item sales")?;
        let mut out = Vec::new();
        for row in rows {
            out.push(row?);
        }
        Ok(out)
    }

    /// Total efetivamente recebido (soma de `paid_amount`) — por período.
    /// É a base do caixa/saldo (Opção A): crédito não recebido nunca infla a gaveta.
    pub fn total_received(&self) -> f64 {
        let mut stmt = self
            .conn
            .prepare("SELECT COALESCE(SUM(paid_amount), 0.0) FROM sales")
            .unwrap();
        let row: rusqlite::Result<f64> = stmt.query_row([], |r| r.get(0));
        row.unwrap_or(0.0)
    }

    pub fn total_received_for_day(&self, day: &str) -> f64 {
        self.sum_for_period(
            "SELECT COALESCE(SUM(paid_amount), 0.0) FROM sales WHERE strftime('%Y-%m-%d', created_at) = ?1",
            day,
        )
    }

    pub fn total_received_for_month(&self, year_month: &str) -> f64 {
        self.sum_for_period(
            "SELECT COALESCE(SUM(paid_amount), 0.0) FROM sales WHERE strftime('%Y-%m', created_at) = ?1",
            year_month,
        )
    }

    pub fn total_received_for_year(&self, year: &str) -> f64 {
        self.sum_for_period(
            "SELECT COALESCE(SUM(paid_amount), 0.0) FROM sales WHERE strftime('%Y', created_at) = ?1",
            year,
        )
    }

    /// Shim legado (pagamento removido). Será excluído na fase Egui.
    #[deprecated(note = "pagamento removido dos relatórios")]
    pub fn total_received_by_method(&self, method: &PaymentMethod) -> f64 {
        let pm = method.as_str();
        let mut stmt = self
            .conn
            .prepare("SELECT COALESCE(SUM(paid_amount), 0.0) FROM sales WHERE payment_method = ?1")
            .unwrap();
        let row: rusqlite::Result<f64> = stmt.query_row([pm], |r| r.get(0));
        row.unwrap_or(0.0)
    }

    /// Recebido no ato da venda (congela `initial_paid`): base do caixa/saldo.
    /// Abatimentos posteriores entram só via `cash_entries`.
    pub fn total_initial_received(&self) -> f64 {
        let mut stmt = self
            .conn
            .prepare("SELECT COALESCE(SUM(initial_paid), 0.0) FROM sales")
            .unwrap();
        let row: rusqlite::Result<f64> = stmt.query_row([], |r| r.get(0));
        row.unwrap_or(0.0)
    }

    pub fn total_initial_received_for_day(&self, day: &str) -> f64 {
        self.sum_for_period(
            "SELECT COALESCE(SUM(initial_paid), 0.0) FROM sales WHERE strftime('%Y-%m-%d', created_at) = ?1",
            day,
        )
    }

    pub fn total_initial_received_for_month(&self, year_month: &str) -> f64 {
        self.sum_for_period(
            "SELECT COALESCE(SUM(initial_paid), 0.0) FROM sales WHERE strftime('%Y-%m', created_at) = ?1",
            year_month,
        )
    }

    pub fn total_initial_received_for_year(&self, year: &str) -> f64 {
        self.sum_for_period(
            "SELECT COALESCE(SUM(initial_paid), 0.0) FROM sales WHERE strftime('%Y', created_at) = ?1",
            year,
        )
    }

    /// Shim legado (pagamento removido). Será excluído na fase Egui.
    #[deprecated(note = "pagamento removido dos relatórios")]
    pub fn total_initial_received_by_method(&self, method: &PaymentMethod) -> f64 {
        let pm = method.as_str();
        let mut stmt = self
            .conn
            .prepare("SELECT COALESCE(SUM(initial_paid), 0.0) FROM sales WHERE payment_method = ?1")
            .unwrap();
        let row: rusqlite::Result<f64> = stmt.query_row([pm], |r| r.get(0));
        row.unwrap_or(0.0)
    }

    /// Saldo devedor global: soma de (total - pago) das vendas em aberto.
    /// Propositalmente sem filtro temporal — KPI fixo do painel.
    pub fn total_receivables_global(&self) -> f64 {
        let mut stmt = self
            .conn
            .prepare("SELECT COALESCE(SUM(total_price - paid_amount), 0.0) FROM sales WHERE status != 'quitado'")
            .unwrap();
        let row: rusqlite::Result<f64> = stmt.query_row([], |r| r.get(0));
        row.unwrap_or(0.0).max(0.0)
    }

    pub fn total_receivables_for_client(&self, client: &str) -> f64 {
        let mut stmt = self
            .conn
            .prepare("SELECT COALESCE(SUM(total_price - paid_amount), 0.0) FROM sales WHERE status != 'quitado' AND client_name = ?1")
            .unwrap();
        let row: rusqlite::Result<f64> = stmt.query_row([client], |r| r.get(0));
        row.unwrap_or(0.0).max(0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::NewSaleItem;
    use rusqlite::params;

    fn banco_teste() -> Database {
        Database::open_in_memory().expect("banco de teste em memória")
    }

    fn venda(db: &Database, nome: &str, total: f64, metodo: &str, criado_em: &str) {
        // Venda legada quitada: pago = total, sem cliente, status quitado.
        db.conn
            .execute(
                "INSERT INTO sales (name, quantity, unit_price, total_price, payment_method, paid_amount, initial_paid, client_name, phone, imei, sale_type, status, created_at) VALUES (?1, 1, ?2, ?2, ?3, ?2, ?2, '', '', '', 'rapida', 'quitado', ?4)",
                params![nome, total, metodo, criado_em],
            )
            .expect("inserir venda de teste");
    }

    #[allow(clippy::too_many_arguments)]
    fn venda_parcial(
        db: &Database,
        nome: &str,
        total: f64,
        pago: f64,
        metodo: &str,
        cliente: &str,
        status: &str,
        criado_em: &str,
    ) {
        db.conn
            .execute(
                "INSERT INTO sales (name, quantity, unit_price, total_price, payment_method, paid_amount, initial_paid, client_name, phone, imei, sale_type, status, created_at) VALUES (?1, 1, ?2, ?2, ?3, ?4, ?4, ?5, '', '', 'rapida', ?6, ?7)",
                params![nome, total, metodo, pago, cliente, status, criado_em],
            )
            .expect("inserir venda parcial de teste");
    }

    fn nova_venda(
        nome: &str,
        total_unit: &str,
        metodo: Option<PaymentMethod>,
        pago: &str,
        cliente: &str,
        fiado: bool,
    ) -> NewSale {
        nova_venda_com_tipo(
            nome,
            total_unit,
            metodo,
            pago,
            cliente,
            "",
            "",
            SaleType::Rapida,
            fiado,
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn nova_venda_com_tipo(
        nome: &str,
        total_unit: &str,
        metodo: Option<PaymentMethod>,
        pago: &str,
        cliente: &str,
        telefone: &str,
        imei: &str,
        tipo: SaleType,
        fiado: bool,
    ) -> NewSale {
        // `tipo` agora é a categoria da linha; o cabeçalho é derivado.
        // `Mista` como categoria de linha é inválida — testes usam tipos reais.
        let item_tipo = if tipo == SaleType::Mista {
            SaleType::Rapida
        } else {
            tipo
        };
        NewSale {
            items: vec![NewSaleItem {
                name: nome.to_string(),
                quantity: "1".to_string(),
                unit_price: total_unit.to_string(),
                imei: imei.to_string(),
                item_type: item_tipo,
            }],
            payment_method: metodo,
            client_name: cliente.to_string(),
            phone: telefone.to_string(),
            sale_type: tipo,
            paid_amount: pago.to_string(),
            is_fiado: fiado,
            valor_recebido: String::new(),
        }
    }

    /// Rascunho multi-item: cada tupla é (nome, qtd, unitário, imei).
    /// Todas as linhas herdam `tipo` como categoria (testes single-tipo);
    /// para carrinho misto real, montar `NewSale` manualmente (ver teste
    /// `carrinho_misto_servico_acessorio_aparelho`).
    #[allow(clippy::too_many_arguments)]
    fn nova_venda_itens(
        itens: Vec<(&str, &str, &str, &str)>,
        metodo: Option<PaymentMethod>,
        pago: &str,
        cliente: &str,
        telefone: &str,
        tipo: SaleType,
        fiado: bool,
    ) -> NewSale {
        let item_tipo = if tipo == SaleType::Mista {
            SaleType::Rapida
        } else {
            tipo
        };
        NewSale {
            items: itens
                .into_iter()
                .map(|(nome, qtd, unit, imei)| NewSaleItem {
                    name: nome.to_string(),
                    quantity: qtd.to_string(),
                    unit_price: unit.to_string(),
                    imei: imei.to_string(),
                    item_type: item_tipo,
                })
                .collect(),
            payment_method: metodo,
            client_name: cliente.to_string(),
            phone: telefone.to_string(),
            sale_type: tipo,
            paid_amount: pago.to_string(),
            is_fiado: fiado,
            valor_recebido: String::new(),
        }
    }

    /// Rascunho multi-categoria: cada tupla é (nome, qtd, unitário, imei, tipo).
    /// O cabeçalho `sale_type` é ignorado por `add_sale` (derivado das linhas).
    #[allow(clippy::too_many_arguments)]
    fn nova_venda_mista(
        itens: Vec<(&str, &str, &str, &str, SaleType)>,
        pago: &str,
        cliente: &str,
        telefone: &str,
        fiado: bool,
    ) -> NewSale {
        NewSale {
            items: itens
                .into_iter()
                .map(|(nome, qtd, unit, imei, tipo)| NewSaleItem {
                    name: nome.to_string(),
                    quantity: qtd.to_string(),
                    unit_price: unit.to_string(),
                    imei: imei.to_string(),
                    item_type: tipo,
                })
                .collect(),
            payment_method: None,
            client_name: cliente.to_string(),
            phone: telefone.to_string(),
            sale_type: SaleType::Mista,
            paid_amount: pago.to_string(),
            is_fiado: fiado,
            valor_recebido: String::new(),
        }
    }

    fn movimento(
        db: &Database,
        valor: f64,
        entrada: i64,
        origem: &str,
        descricao: &str,
        criado_em: &str,
    ) {
        db.conn
            .execute(
                "INSERT INTO cash_entries (amount, is_entry, source, description, created_at) VALUES (?1, ?2, ?3, ?4, ?5)",
                params![valor, entrada, origem, descricao, criado_em],
            )
            .expect("inserir movimento de teste");
    }

    fn perda(db: &Database, valor: f64, origem: &str, descricao: &str, criado_em: &str) {
        db.conn
            .execute(
                "INSERT INTO losses (amount, source, description, created_at) VALUES (?1, ?2, ?3, ?4)",
                params![valor, origem, descricao, criado_em],
            )
            .expect("inserir perda de teste");
    }

    fn quase_igual(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-9
    }

    #[test]
    fn periodo_vazio_retorna_zero() {
        let db = banco_teste();
        assert!(quase_igual(db.total_sales_for_day("2026-10-09"), 0.0));
        assert!(quase_igual(db.total_sales_for_month("2026-10"), 0.0));
        assert!(quase_igual(db.total_sales_for_year("2026"), 0.0));
        assert!(quase_igual(db.total_cash_in_for_month("2026-10"), 0.0));
        assert!(quase_igual(db.total_cash_out_for_month("2026-10"), 0.0));
        assert!(quase_igual(db.total_losses_for_month("2026-10"), 0.0));
        assert!(quase_igual(db.net_balance_for_day("2026-10-09"), 0.0));
        assert!(quase_igual(db.net_balance_for_month("2026-10"), 0.0));
        assert!(quase_igual(db.net_balance_for_year("2026"), 0.0));
        assert_eq!(db.count_sales_for_day("2026-10-09"), 0);
        assert_eq!(db.count_sales_for_month("2026-10"), 0);
        assert_eq!(db.count_sales_for_year("2026"), 0);
        assert!(quase_igual(db.average_ticket_for_day("2026-10-09"), 0.0));
        assert!(quase_igual(db.average_ticket_for_month("2026-10"), 0.0));
        assert!(quase_igual(db.average_ticket_for_year("2026"), 0.0));
        assert!(quase_igual(db.cash_in_drawer(), 0.0));
        assert!(db.get_sales_for_day("2026-10-09").is_empty());
        assert!(db.distinct_days().is_empty());
        assert!(db.distinct_months().is_empty());
        assert!(db.distinct_years().is_empty());
    }

    #[test]
    fn isolamento_diario_na_fronteira_do_mes() {
        let db = banco_teste();
        venda(&db, "Serviço A", 100.0, "Cash", "2026-01-31 20:00:00");
        venda(&db, "Serviço B", 200.0, "Pix", "2026-02-01 09:00:00");
        // Totais diários isolados, sem sangramento entre 31/01 e 01/02.
        assert!(quase_igual(db.total_sales_for_day("2026-01-31"), 100.0));
        assert!(quase_igual(db.total_sales_for_day("2026-02-01"), 200.0));
        assert_eq!(db.count_sales_for_day("2026-01-31"), 1);
        // Totais mensais isolados.
        assert!(quase_igual(db.total_sales_for_month("2026-01"), 100.0));
        assert!(quase_igual(db.total_sales_for_month("2026-02"), 200.0));
        assert_eq!(db.get_sales_for_month("2026-01").len(), 1);
        assert_eq!(db.get_sales_for_month("2026-02").len(), 1);
        assert_eq!(db.get_sales_for_month("2026-01")[0].name, "Serviço A");
    }

    #[test]
    fn isolamento_mensal_na_fronteira_do_ano() {
        let db = banco_teste();
        venda(&db, "Reparo X", 150.0, "Card", "2025-12-31 23:30:00");
        venda(&db, "Reparo Y", 250.0, "Cash", "2026-01-01 00:30:00");
        assert!(quase_igual(db.total_sales_for_month("2025-12"), 150.0));
        assert!(quase_igual(db.total_sales_for_month("2026-01"), 250.0));
        assert!(quase_igual(db.total_sales_for_year("2025"), 150.0));
        assert!(quase_igual(db.total_sales_for_year("2026"), 250.0));
        assert_eq!(db.count_sales_for_year("2025"), 1);
        assert_eq!(db.count_sales_for_year("2026"), 1);
        assert_eq!(db.get_sales_for_year("2026").len(), 1);
    }

    #[test]
    fn bissexto_e_fevereiro_curto() {
        let db = banco_teste();
        venda(&db, "Bissexto", 290.0, "Pix", "2024-02-29 12:00:00");
        venda(&db, "Véspera", 100.0, "Cash", "2024-02-28 12:00:00");
        // 29/02 pertence a 2024-02, sem sangrar para março.
        assert!(quase_igual(db.total_sales_for_day("2024-02-29"), 290.0));
        assert!(quase_igual(db.total_sales_for_month("2024-02"), 390.0));
        assert!(quase_igual(db.total_sales_for_month("2024-03"), 0.0));
        assert!(quase_igual(db.total_sales_for_year("2024"), 390.0));
        assert_eq!(db.count_sales_for_month("2024-02"), 2);
        // Fevereiro de ano não bissexto continua vazio e zerado.
        assert!(quase_igual(db.total_sales_for_month("2023-02"), 0.0));
        assert!(quase_igual(db.total_sales_for_day("2023-02-29"), 0.0));
    }

    #[test]
    fn isolamento_caixa_perdas_e_origens_por_periodo() {
        let db = banco_teste();
        movimento(&db, 500.0, 1, "drawer", "Reforço", "2026-05-10 10:00:00");
        movimento(&db, 80.0, 0, "drawer", "Despesa", "2026-05-11 10:00:00");
        movimento(
            &db,
            1000.0,
            1,
            "bank",
            "Transferência",
            "2026-05-10 10:00:00",
        );
        perda(&db, 30.0, "drawer", "Quebra", "2026-05-12 10:00:00");
        perda(&db, 20.0, "drawer", "Quebra antiga", "2026-04-12 10:00:00");
        // Mês isolado: a perda de abril não contamina maio.
        assert!(quase_igual(db.total_cash_in_for_month("2026-05"), 1500.0));
        assert!(quase_igual(db.total_cash_out_for_month("2026-05"), 80.0));
        assert!(quase_igual(db.total_losses_for_month("2026-05"), 30.0));
        assert!(quase_igual(db.total_losses_for_month("2026-04"), 20.0));
        assert!(quase_igual(db.total_losses_for_month("2026-06"), 0.0));
        // Gaveta exclui o banco; banco não altera a gaveta.
        assert!(quase_igual(
            db.total_cash_in_by_source_for_month(&CashSource::Drawer, "2026-05"),
            500.0
        ));
        assert!(quase_igual(
            db.total_cash_in_by_source_for_month(&CashSource::Bank, "2026-05"),
            1000.0
        ));
        assert!(quase_igual(
            db.cash_in_drawer_for_month("2026-05"),
            500.0 - 80.0 - 30.0
        ));
        // Dia isolado dentro do mês.
        assert!(quase_igual(db.total_cash_in_for_day("2026-05-10"), 1500.0));
        assert!(quase_igual(db.total_cash_in_for_day("2026-05-11"), 0.0));
        assert!(quase_igual(db.cash_in_drawer_for_day("2026-05-10"), 500.0));
    }

    #[test]
    fn agregados_por_mes_ano_e_saldo_consistentes() {
        let db = banco_teste();
        venda(&db, "A", 100.0, "Cash", "2026-03-05 10:00:00");
        venda(&db, "B", 200.0, "Pix", "2026-03-20 10:00:00");
        venda(&db, "C", 400.0, "Cash", "2026-07-01 10:00:00");
        movimento(&db, 50.0, 1, "drawer", "Reforço", "2026-03-06 10:00:00");
        movimento(&db, 20.0, 0, "drawer", "Despesa", "2026-03-07 10:00:00");
        perda(&db, 10.0, "drawer", "Quebra", "2026-03-08 10:00:00");
        // Agregações corrigidas (sem alias no WHERE) agrupam e ordenam.
        let por_mes = db.sales_by_month();
        assert_eq!(por_mes.len(), 2);
        assert_eq!(por_mes[0].0, "2026-07");
        assert!(quase_igual(por_mes[0].1, 400.0));
        assert_eq!(por_mes[1].0, "2026-03");
        assert!(quase_igual(por_mes[1].1, 300.0));
        let por_ano = db.sales_by_year();
        assert_eq!(por_ano.len(), 1);
        assert_eq!(por_ano[0].0, "2026");
        assert!(quase_igual(por_ano[0].1, 700.0));
        // Saldo = vendas + entradas − saídas − perdas, por período.
        assert!(quase_igual(
            db.net_balance_for_month("2026-03"),
            300.0 + 50.0 - 20.0 - 10.0
        ));
        assert!(quase_igual(
            db.net_balance_for_year("2026"),
            700.0 + 50.0 - 20.0 - 10.0
        ));
        // Ticket médio por período e vazio zerado.
        assert!(quase_igual(db.average_ticket_for_month("2026-03"), 150.0));
        assert!(quase_igual(db.average_ticket_for_year("2026"), 700.0 / 3.0));
        assert!(quase_igual(db.average_ticket_for_month("2026-04"), 0.0));
        // Distintos ordenados do mais recente ao mais antigo.
        assert_eq!(db.distinct_months(), vec!["2026-07", "2026-03"]);
        assert_eq!(db.distinct_years(), vec!["2026"]);
    }

    #[test]
    fn vendas_por_pagamento_isoladas_por_periodo() {
        let db = banco_teste();
        venda(&db, "D1", 100.0, "Cash", "2026-09-01 10:00:00");
        venda(&db, "D2", 50.0, "Pix", "2026-09-02 10:00:00");
        assert!(quase_igual(db.total_cash_sales_for_month("2026-09"), 100.0));
        assert!(quase_igual(db.total_cash_sales_for_month("2026-08"), 0.0));
        assert!(quase_igual(db.total_cash_sales_for_year("2026"), 100.0));
        assert!(quase_igual(db.total_cash_sales_for_day("2026-09-02"), 0.0));
    }

    #[test]
    fn parcial_nao_infla_caixa_e_compoe_recebiveis() {
        let mut db = banco_teste();
        let v = nova_venda(
            "Troca de tela",
            "300",
            Some(PaymentMethod::Cash),
            "100",
            "Maria Silva",
            false,
        );
        db.add_sale(&v).expect("venda parcial válida");
        // Comprometido x recebido: total 300, recebido 100.
        assert!(quase_igual(db.total_sales(), 300.0));
        assert!(quase_igual(db.total_received(), 100.0));
        assert!(quase_igual(db.total_receivables_global(), 200.0));
        let debts = db.get_pending_debts("", None);
        assert_eq!(debts.len(), 1);
        assert_eq!(debts[0].status, SaleStatus::Parcial);
        assert!(quase_igual(debts[0].remaining(), 200.0));
        assert_eq!(debts[0].client_name, "Maria Silva");
        // Gaveta: só o recebido (100), nunca o total (300).
        assert!(quase_igual(db.cash_in_drawer(), 100.0));
        assert!(quase_igual(db.net_balance_global(), 100.0));
    }

    #[test]
    fn pendente_sem_entrada_tem_recebido_zero() {
        let mut db = banco_teste();
        // Pendente: meio da entrada (Dinheiro) + flag fiado, nada recebido.
        let v = nova_venda(
            "Reparo fiado",
            "150",
            Some(PaymentMethod::Cash),
            "",
            "João Souza",
            true,
        );
        db.add_sale(&v).expect("venda pendente válida");
        assert!(quase_igual(db.total_received(), 0.0));
        assert!(quase_igual(db.total_receivables_global(), 150.0));
        assert!(quase_igual(db.cash_in_drawer(), 0.0));
        let debts = db.get_pending_debts("", Some(&SaleStatus::Pendente));
        assert_eq!(debts.len(), 1);
        assert!(quase_igual(
            db.total_receivables_for_client("João Souza"),
            150.0
        ));
        assert!(quase_igual(db.total_receivables_for_client("Outro"), 0.0));
    }

    #[test]
    fn add_sale_valida_pago_e_cliente() {
        let mut db = banco_teste();
        // Pago maior que o total falha.
        let v = nova_venda(
            "Item",
            "100",
            Some(PaymentMethod::Cash),
            "150",
            "Cliente",
            false,
        );
        assert!(db.add_sale(&v).is_err());
        // Pago negativo falha.
        let v = nova_venda(
            "Item",
            "100",
            Some(PaymentMethod::Cash),
            "-10",
            "Cliente",
            false,
        );
        assert!(db.add_sale(&v).is_err());
        // Parcial sem cliente falha.
        let v = nova_venda("Item", "100", Some(PaymentMethod::Cash), "40", "", false);
        assert!(db.add_sale(&v).is_err());
        // Pendente sem cliente falha (meio da entrada + flag fiado, pago vazio).
        let v = nova_venda("Item", "100", Some(PaymentMethod::Pix), "", "", true);
        assert!(db.add_sale(&v).is_err());
        // Quitada sem cliente passa.
        let v = nova_venda("Item", "100", Some(PaymentMethod::Pix), "", "", false);
        assert!(db.add_sale(&v).is_ok());
        assert!(quase_igual(db.total_receivables_global(), 0.0));
    }

    #[test]
    fn abate_reduz_restante_e_alimenta_caixa_do_dia() {
        let mut db = banco_teste();
        let v = nova_venda(
            "Conserto",
            "300",
            Some(PaymentMethod::Cash),
            "",
            "Ana",
            true,
        );
        db.add_sale(&v).expect("pendente");
        let sale = db.get_pending_debts("Ana", None).pop().expect("dívida");
        let caixa_antes = db.total_cash_in_for_day(&crate::models::hoje_local());
        let restante = db
            .register_abate_fiado_sem_meio(sale.id, 120.0)
            .expect("abate parcial");
        assert!(quase_igual(restante, 180.0));
        assert!(quase_igual(db.total_receivables_global(), 180.0));
        assert!(quase_igual(
            db.total_cash_in_for_day(&crate::models::hoje_local()),
            caixa_antes + 120.0
        ));
        // Abate final quita e zera o global.
        let restante = db
            .register_abate_fiado_sem_meio(sale.id, 180.0)
            .expect("quitação");
        assert!(quase_igual(restante, 0.0));
        assert!(quase_igual(db.total_receivables_global(), 0.0));
        let quitada = db.get_sale_by_id(sale.id).expect("venda");
        assert_eq!(quitada.status, SaleStatus::Quitado);
        assert!(db.get_pending_debts("", None).is_empty());
    }

    #[test]
    fn abate_acima_do_restante_falha_sem_efeito_colateral() {
        let mut db = banco_teste();
        let v = nova_venda("Peça", "200", Some(PaymentMethod::Cash), "50", "Bia", false);
        db.add_sale(&v).expect("parcial");
        let sale = db.get_pending_debts("Bia", None).pop().expect("dívida");
        assert!(
            db.register_abate_fiado_sem_meio(sale.id, 151.0)
                .is_err()
        );
        assert!(
            db.register_abate_fiado_sem_meio(sale.id, 0.0)
                .is_err()
        );
        assert!(
            db.register_abate_fiado_sem_meio(99999, 10.0)
                .is_err()
        );
        // Nada mudou: pago continua 50, restante 150, sem entrada no caixa.
        let kept = db.get_sale_by_id(sale.id).expect("venda");
        assert!(quase_igual(kept.paid_amount, 50.0));
        assert!(quase_igual(db.total_receivables_global(), 150.0));
        assert!(quase_igual(db.total_cash_in(), 0.0));
    }

    #[test]
    fn abate_pix_vai_para_banco_nao_para_gaveta() {
        // Legado renomeado: meio removido — todo abate é Entrada na gaveta.
        let mut db = banco_teste();
        let v = nova_venda(
            "Manutenção",
            "300",
            Some(PaymentMethod::Pix),
            "",
            "Rui",
            true,
        );
        db.add_sale(&v).expect("pendente");
        let sale = db.get_pending_debts("Rui", None).pop().expect("dívida");
        let hoje = crate::models::hoje_local();
        let restante = db
            .register_abate_fiado_sem_meio(sale.id, 120.0)
            .expect("abate");
        assert!(quase_igual(restante, 180.0));
        assert!(quase_igual(db.total_receivables_global(), 180.0));
        // Roteamento novo: gaveta +120, banco zerado, sem rótulo de meio.
        assert!(quase_igual(
            db.total_cash_in_by_source_for_day(&CashSource::Drawer, &hoje),
            120.0
        ));
        assert!(quase_igual(
            db.total_cash_in_by_source_for_day(&CashSource::Bank, &hoje),
            0.0
        ));
        assert!(quase_igual(db.cash_in_drawer_for_day(&hoje), 120.0));
        let entradas = db.get_cash_entries_for_day(&hoje);
        assert_eq!(entradas.len(), 1);
        assert!(!entradas[0].description.contains("(Pix)"));
        assert!(entradas[0].description.contains(&sale.id.to_string()));
        // Segundo abate soma na gaveta.
        db.register_abate_fiado_sem_meio(sale.id, 80.0)
            .expect("segundo abate");
        assert!(quase_igual(
            db.total_cash_in_by_source_for_day(&CashSource::Drawer, &hoje),
            200.0
        ));
        assert!(quase_igual(db.total_receivables_global(), 100.0));
    }

    #[test]
    fn legado_fiado_ainda_le() {
        let db = banco_teste();
        // Linha gravada pelo formato antigo (meio 'Fiado' direto no banco).
        db.conn
            .execute(
                "INSERT INTO sales (name, quantity, unit_price, total_price, payment_method, paid_amount, client_name, status, created_at) VALUES ('Legada', 1, 200, 200, 'Fiado', 0, 'Zé', 'pendente', '2026-01-05 10:00:00')",
                [],
            )
            .expect("inserir venda legada");
        let sale = db.get_sale_by_id(1).expect("venda legada");
        assert_eq!(sale.payment_method, PaymentMethod::Fiado);
        assert_eq!(sale.status, SaleStatus::Pendente);
        assert_eq!(db.get_pending_debts("Zé", None).len(), 1);
        assert!(quase_igual(db.total_receivables_global(), 200.0));
    }

    #[test]
    fn recebiveis_globais_ignoram_filtro_temporal() {
        let db = banco_teste();
        venda_parcial(
            &db,
            "Antiga",
            100.0,
            20.0,
            "Fiado",
            "C1",
            "parcial",
            "2024-01-05 10:00:00",
        );
        venda_parcial(
            &db,
            "Recente",
            200.0,
            0.0,
            "Fiado",
            "C2",
            "pendente",
            "2026-09-05 10:00:00",
        );
        // Global soma tudo: 80 + 200 = 280, independente do período.
        assert!(quase_igual(db.total_receivables_global(), 280.0));
        // Recebido respeita o período.
        assert!(quase_igual(db.total_received_for_month("2024-01"), 20.0));
        assert!(quase_igual(db.total_received_for_month("2026-09"), 0.0));
        assert!(quase_igual(db.total_received_for_day("2024-01-05"), 20.0));
        // Filtros por cliente/status.
        assert_eq!(db.get_pending_debts("c1", None).len(), 1);
        assert_eq!(
            db.get_pending_debts("", Some(&SaleStatus::Pendente)).len(),
            1
        );
        assert_eq!(
            db.get_pending_debts("", Some(&SaleStatus::Parcial)).len(),
            1
        );
        assert_eq!(
            db.get_pending_debts("", Some(&SaleStatus::Quitado)).len(),
            0
        );
        assert!(db.get_pending_debts("", None).len() == 2);
    }

    #[test]
    fn abate_nao_infla_vendas_em_dinheiro() {
        let mut db = banco_teste();
        // Fiado puro em dinheiro: nada recebido no ato.
        let v = nova_venda(
            "Fiado puro",
            "200",
            Some(PaymentMethod::Cash),
            "",
            "Ana",
            true,
        );
        db.add_sale(&v).expect("pendente");
        let sale = db.get_pending_debts("Ana", None).pop().expect("dívida");
        db.register_abate_fiado_sem_meio(sale.id, 50.0)
            .expect("abate 50 em dinheiro");
        // Vendas em dinheiro: só o ato (0), nunca o abate.
        assert!(quase_igual(db.total_cash_sales(), 0.0));
        assert!(quase_igual(
            db.total_cash_sales_for_day(&crate::models::hoje_local()),
            0.0
        ));
        // Gaveta: só o ledger do abate (50), sem duplicar.
        assert!(quase_igual(db.cash_in_drawer(), 50.0));
        assert!(quase_igual(db.net_balance_global(), 50.0));
        assert!(quase_igual(db.total_receivables_global(), 150.0));
        // Parcial com entrada inicial 100 + abate 50: vendas=100, gaveta=150.
        let v2 = nova_venda(
            "Parcial",
            "300",
            Some(PaymentMethod::Cash),
            "100",
            "Bia",
            false,
        );
        db.add_sale(&v2).expect("parcial");
        let sale2 = db.get_pending_debts("Bia", None).pop().expect("dívida");
        db.register_abate_fiado_sem_meio(sale2.id, 50.0)
            .expect("abate parcial");
        assert!(quase_igual(db.total_cash_sales(), 100.0));
        assert!(quase_igual(db.cash_in_drawer(), 200.0));
        assert!(quase_igual(db.net_balance_global(), 200.0));
    }

    #[test]
    fn servico_exige_cliente_mesmo_quitado() {
        let mut db = banco_teste();
        // Serviço quitado sem cliente falha (garantia).
        let v = nova_venda_com_tipo(
            "Troca de tela",
            "200",
            Some(PaymentMethod::Pix),
            "",
            "",
            "",
            "",
            SaleType::Servico,
            false,
        );
        assert!(db.add_sale(&v).is_err());
        // Com cliente passa.
        let v = nova_venda_com_tipo(
            "Troca de tela",
            "200",
            Some(PaymentMethod::Pix),
            "",
            "Carlos",
            "11999990000",
            "",
            SaleType::Servico,
            false,
        );
        assert!(db.add_sale(&v).is_ok());
        let sales = db.get_sales_filtered("Carlos");
        assert_eq!(sales.len(), 1);
        assert_eq!(sales[0].sale_type, SaleType::Servico);
        assert_eq!(sales[0].phone, "11999990000");
    }

    #[test]
    fn aparelho_exige_imei_e_cliente() {
        let mut db = banco_teste();
        // Aparelho sem IMEI falha.
        let v = nova_venda_com_tipo(
            "iPhone 12",
            "2500",
            Some(PaymentMethod::Card),
            "",
            "Ana",
            "",
            "",
            SaleType::Aparelho,
            false,
        );
        assert!(db.add_sale(&v).is_err());
        // Aparelho sem cliente falha.
        let v = nova_venda_com_tipo(
            "iPhone 12",
            "2500",
            Some(PaymentMethod::Card),
            "",
            "",
            "",
            "355123456789012",
            SaleType::Aparelho,
            false,
        );
        assert!(db.add_sale(&v).is_err());
        // Completo passa.
        let v = nova_venda_com_tipo(
            "iPhone 12",
            "2500",
            Some(PaymentMethod::Card),
            "",
            "Ana",
            "11988887777",
            "355123456789012",
            SaleType::Aparelho,
            false,
        );
        assert!(db.add_sale(&v).is_ok());
        let sales = db.get_sales_filtered("355123456789012");
        assert_eq!(sales.len(), 1);
        assert_eq!(sales[0].sale_type, SaleType::Aparelho);
        assert_eq!(sales[0].imei, "355123456789012");
    }

    #[test]
    fn migracao_legada_preenche_defaults_pos() {
        let db = banco_teste();
        // Linha no formato antigo (sem sale_type/phone/imei): defaults assumem.
        db.conn
            .execute(
                "INSERT INTO sales (name, quantity, unit_price, total_price, payment_method, paid_amount, initial_paid, client_name, status, created_at) VALUES ('Antiga', 1, 100, 100, 'Cash', 100, 100, '', 'quitado', '2026-01-05 10:00:00')",
                [],
            )
            .expect("inserir venda legada");
        let sale = db.get_sale_by_id(1).expect("venda legada");
        assert_eq!(sale.sale_type, SaleType::Rapida);
        assert_eq!(sale.phone, "");
        assert_eq!(sale.imei, "");
    }

    #[test]
    fn summary_name_resume_primeiro_item() {
        assert_eq!(summary_name("Película", 0), "Película");
        assert_eq!(summary_name("iPhone 12", 1), "iPhone 12 (+1 item)");
        assert_eq!(summary_name("Troca de Tela", 2), "Troca de Tela (+2 itens)");
    }

    #[test]
    fn multi_item_aparelho_soma_totais_e_resumo() {
        let mut db = banco_teste();
        // Aparelho + acessório multi-categoria: IMEI só no item Aparelho.
        let v = nova_venda_mista(
            vec![
                ("iPhone 12", "1", "2500", "355123456789012", SaleType::Aparelho),
                ("Capinha - Silicone", "2", "40", "", SaleType::Rapida),
            ],
            "",
            "Ana",
            "11988887777",
            false,
        );
        let id = db.add_sale(&v).expect("multi-item válido");
        let sale = db.get_sale_by_id(id).expect("venda");
        assert_eq!(sale.sale_type, SaleType::Mista);
        assert!(quase_igual(sale.total_price, 2580.0));
        assert_eq!(sale.name, "iPhone 12 (+1 item)");
        assert_eq!(sale.quantity, 3);
        assert_eq!(sale.item_count, 2);
        assert_eq!(sale.imei, "355123456789012");
        assert_eq!(sale.items.len(), 2);
        assert_eq!(sale.items[0].name, "iPhone 12");
        assert_eq!(sale.items[0].quantity, 1);
        assert!(quase_igual(sale.items[0].total_price, 2500.0));
        assert_eq!(sale.items[0].imei, "355123456789012");
        assert_eq!(sale.items[0].item_type, SaleType::Aparelho);
        assert_eq!(sale.items[1].name, "Capinha - Silicone");
        assert_eq!(sale.items[1].quantity, 2);
        assert!(quase_igual(sale.items[1].total_price, 80.0));
        assert_eq!(sale.items[1].imei, "");
        assert_eq!(sale.items[1].item_type, SaleType::Rapida);
        assert!(quase_igual(db.total_sales(), 2580.0));
    }

    #[test]
    fn multi_item_busca_alcanca_segundo_item() {
        let mut db = banco_teste();
        let v = nova_venda_mista(
            vec![
                ("iPhone 12", "1", "2500", "355123456789012", SaleType::Aparelho),
                ("Película - 3D", "1", "50", "", SaleType::Rapida),
            ],
            "",
            "Bia",
            "",
            false,
        );
        db.add_sale(&v).expect("multi-item");
        // Segundo item pelo nome e o primeiro pelo resumo.
        assert_eq!(db.get_sales_filtered("película").len(), 1);
        assert_eq!(db.get_sales_filtered("iphone").len(), 1);
        assert_eq!(db.get_sales_filtered("355123456789012").len(), 1);
        assert!(db.get_sales_filtered("zzz-inexistente").is_empty());
    }

    #[test]
    fn aparelho_exige_imei_por_linha() {
        let mut db = banco_teste();
        // Aparelho sem IMEI falha, mesmo com acessório preenchido.
        let v = nova_venda_mista(
            vec![
                ("iPhone 12", "1", "2500", "", SaleType::Aparelho),
                ("Capinha", "1", "40", "", SaleType::Rapida),
            ],
            "",
            "Ana",
            "",
            false,
        );
        assert!(db.add_sale(&v).is_err());
        // Aparelho com IMEI + acessório sem IMEI passa.
        let v = nova_venda_mista(
            vec![
                ("iPhone 12", "1", "2500", "355123456789012", SaleType::Aparelho),
                ("Capinha", "1", "40", "", SaleType::Rapida),
            ],
            "",
            "Ana",
            "",
            false,
        );
        assert!(db.add_sale(&v).is_ok());
        // Segundo aparelho sem IMEI também falha (regra por-linha).
        let v = nova_venda_mista(
            vec![
                ("iPhone 12", "1", "2500", "355123456789012", SaleType::Aparelho),
                ("iPhone 13", "1", "3000", "", SaleType::Aparelho),
            ],
            "",
            "Ana",
            "",
            false,
        );
        assert!(db.add_sale(&v).is_err());
    }

    #[test]
    fn carrinho_misto_servico_acessorio_aparelho() {
        let mut db = banco_teste();
        // Serviço + acessório + aparelho no mesmo carrinho.
        let v = nova_venda_mista(
            vec![
                ("Troca de tela", "1", "300", "", SaleType::Servico),
                ("Capinha", "1", "40", "", SaleType::Rapida),
                ("iPhone 12", "1", "2500", "355123456789012", SaleType::Aparelho),
            ],
            "",
            "Carlos",
            "11999990000",
            false,
        );
        let id = db.add_sale(&v).expect("carrinho misto válido");
        let sale = db.get_sale_by_id(id).expect("venda");
        assert_eq!(sale.sale_type, SaleType::Mista);
        assert!(quase_igual(sale.total_price, 2840.0));
        assert_eq!(sale.item_count, 3);
        assert_eq!(sale.items.len(), 3);
        assert_eq!(sale.items[0].item_type, SaleType::Servico);
        assert_eq!(sale.items[1].item_type, SaleType::Rapida);
        assert_eq!(sale.items[2].item_type, SaleType::Aparelho);
        // Misto exige cliente (garantia): sem cliente falha.
        let sem_cliente = nova_venda_mista(
            vec![
                ("Troca de tela", "1", "300", "", SaleType::Servico),
                ("Capinha", "1", "40", "", SaleType::Rapida),
            ],
            "",
            "",
            "",
            false,
        );
        assert!(db.add_sale(&sem_cliente).is_err());
    }

    #[test]
    fn troco_caixa_soma_total_nao_recebido() {
        let mut db = banco_teste();
        // Venda 560, recebido 600 → troco 40, caixa soma 560.
        let mut v = nova_venda("Peça", "560", None, "", "", false);
        v.valor_recebido = "600".to_string();
        let id = db.add_sale(&v).expect("venda com troco");
        let sale = db.get_sale_by_id(id).expect("venda");
        assert!(quase_igual(sale.total_price, 560.0));
        assert!(quase_igual(sale.paid_amount, 560.0));
        assert!(quase_igual(sale.valor_recebido, 600.0));
        assert!(quase_igual(sale.troco, 40.0));
        assert_eq!(sale.status, SaleStatus::Quitado);
        // Caixa: exatamente o total, nunca o recebido.
        assert!(quase_igual(db.total_initial_received(), 560.0));
        assert!(quase_igual(db.total_sales(), 560.0));
        assert!(quase_igual(db.net_balance_global(), 560.0));
        assert!(quase_igual(db.cash_in_drawer(), 560.0));
    }

    #[test]
    fn troco_zero_quando_recebido_vazio_ou_exato() {
        let mut db = banco_teste();
        // Vazio = exato.
        let v = nova_venda("A", "100", None, "", "", false);
        let id = db.add_sale(&v).expect("venda exata");
        let sale = db.get_sale_by_id(id).expect("venda");
        assert!(quase_igual(sale.valor_recebido, 100.0));
        assert!(quase_igual(sale.troco, 0.0));
        // Exato explícito.
        let mut v2 = nova_venda("B", "200", None, "", "", false);
        v2.valor_recebido = "200".to_string();
        let id2 = db.add_sale(&v2).expect("venda exata explícita");
        let sale2 = db.get_sale_by_id(id2).expect("venda");
        assert!(quase_igual(sale2.troco, 0.0));
        assert!(quase_igual(db.total_initial_received(), 300.0));
    }

    #[test]
    fn recebido_menor_que_total_falha() {
        let mut db = banco_teste();
        let mut v = nova_venda("Peça", "560", None, "", "", false);
        v.valor_recebido = "500".to_string();
        assert!(db.add_sale(&v).is_err());
        assert_eq!(db.count_sales(), 0);
    }

    #[test]
    fn fiado_ignora_valor_recebido() {
        let mut db = banco_teste();
        // Fiado pendente: recebido vai para paid_amount vazio (=0); o campo
        // display-only valor_recebido é ignorado, troco sempre 0.
        let mut v = nova_venda("Fiado", "300", None, "", "Rui", true);
        v.valor_recebido = "500".to_string();
        let id = db.add_sale(&v).expect("fiado");
        let sale = db.get_sale_by_id(id).expect("venda");
        assert_eq!(sale.status, SaleStatus::Pendente);
        assert!(quase_igual(sale.paid_amount, 0.0));
        assert!(quase_igual(sale.troco, 0.0));
        assert!(quase_igual(db.total_initial_received(), 0.0));
    }

    #[test]
    fn linha_invalida_aborta_transacao() {
        let mut db = banco_teste();
        let v = nova_venda_itens(
            vec![
                ("Troca de Tela", "1", "300", ""),
                ("Película", "0", "50", ""),
            ],
            Some(PaymentMethod::Cash),
            "",
            "Carlos",
            "",
            SaleType::Servico,
            false,
        );
        assert!(db.add_sale(&v).is_err());
        // Nada persistido: nem cabeçalho, nem linhas órfãs.
        assert_eq!(db.count_sales(), 0);
        let n: i64 = db
            .conn
            .query_row("SELECT COUNT(*) FROM sale_items", [], |r| r.get(0))
            .expect("contar itens");
        assert_eq!(n, 0);
        // Lista vazia e item sem nome também falham.
        let vazio = nova_venda_itens(
            vec![],
            Some(PaymentMethod::Cash),
            "",
            "",
            "",
            SaleType::Rapida,
            false,
        );
        assert!(db.add_sale(&vazio).is_err());
    }

    #[test]
    fn rapida_multi_sem_cliente_ok() {
        let mut db = banco_teste();
        let v = nova_venda_itens(
            vec![("Capinha", "1", "40", ""), ("Película", "2", "25", "")],
            Some(PaymentMethod::Cash),
            "",
            "",
            "",
            SaleType::Rapida,
            false,
        );
        let id = db.add_sale(&v).expect("balcão multi-item");
        let sale = db.get_sale_by_id(id).expect("venda");
        assert!(quase_igual(sale.total_price, 90.0));
        assert_eq!(sale.name, "Capinha (+1 item)");
        assert_eq!(sale.item_count, 2);
        assert_eq!(sale.status, SaleStatus::Quitado);
    }

    #[test]
    fn servico_multi_sem_cliente_falha() {
        let mut db = banco_teste();
        let v = nova_venda_itens(
            vec![
                ("Troca de Tela", "1", "300", ""),
                ("Película", "1", "50", ""),
            ],
            Some(PaymentMethod::Cash),
            "",
            "",
            "",
            SaleType::Servico,
            false,
        );
        assert!(db.add_sale(&v).is_err());
        assert_eq!(db.count_sales(), 0);
    }

    #[test]
    fn multi_item_fiado_soma_restante() {
        let mut db = banco_teste();
        let v = nova_venda_itens(
            vec![
                ("Troca de Tela", "1", "300", ""),
                ("Bateria", "1", "120", ""),
            ],
            Some(PaymentMethod::Cash),
            "100",
            "Maria Silva",
            "",
            SaleType::Servico,
            false,
        );
        db.add_sale(&v).expect("parcial multi-item");
        assert!(quase_igual(db.total_sales(), 420.0));
        assert!(quase_igual(db.total_received(), 100.0));
        assert!(quase_igual(db.total_receivables_global(), 320.0));
    }

    #[test]
    fn backfill_legado_cria_itens() {
        let mut db = banco_teste();
        // Linha gravada direto (sem itens): o migrate preenche 1 linha.
        db.conn
            .execute(
                "INSERT INTO sales (name, quantity, unit_price, total_price, payment_method, paid_amount, initial_paid, client_name, phone, imei, sale_type, status, created_at) VALUES ('Troca de Tela', 1, 300, 300, 'Pix', 300, 300, 'Rui', '', 'ABC123', 'servico', 'quitado', '2026-02-01 10:00:00')",
                [],
            )
            .expect("venda sem itens");
        db.migrate_sale_items().expect("backfill");
        let itens = db.get_items_for_sale(1);
        assert_eq!(itens.len(), 1);
        assert_eq!(itens[0].name, "Troca de Tela");
        assert_eq!(itens[0].quantity, 1);
        assert!(quase_igual(itens[0].total_price, 300.0));
        assert_eq!(itens[0].imei, "ABC123");
        // Idempotente: rodar de novo não duplica.
        db.migrate_sale_items().expect("backfill 2");
        assert_eq!(db.get_items_for_sale(1).len(), 1);
    }

    #[test]
    fn migrate_item_count_conta_linhas() {
        let mut db = banco_teste();
        // Linha legada (sem itens no momento do insert).
        db.conn
            .execute(
                "INSERT INTO sales (name, quantity, unit_price, total_price, payment_method, paid_amount, initial_paid, client_name, phone, imei, sale_type, status, created_at) VALUES ('Troca de Tela', 1, 300, 300, 'Pix', 300, 300, 'Rui', '', '', 'servico', 'quitado', '2026-02-01 10:00:00')",
                [],
            )
            .expect("venda legada");
        // Venda nova com 3 linhas via API.
        let v = nova_venda_itens(
            vec![
                ("A", "1", "10", ""),
                ("B", "1", "20", ""),
                ("C", "1", "30", ""),
            ],
            Some(PaymentMethod::Cash),
            "",
            "",
            "",
            SaleType::Rapida,
            false,
        );
        db.add_sale(&v).expect("multi-item");
        // Reconciliação conta as linhas reais (1 legada + 3 novas).
        db.migrate_item_count().expect("contagem");
        let legada = db.get_sale_by_id(1).expect("legada");
        assert_eq!(legada.item_count, 1);
        let nova = db.get_sale_by_id(2).expect("nova");
        assert_eq!(nova.item_count, 3);
        assert_eq!(nova.name, "A (+2 itens)");
        // Listas também carregam o contador (tabelas usam o resumo).
        let todas = db.get_sales_filtered("");
        assert_eq!(todas.len(), 2);
        assert!(todas.iter().any(|s| s.item_count == 3));
    }

    #[test]
    fn fallback_sintetiza_item_sem_linhas() {
        let db = banco_teste();
        db.conn
            .execute(
                "INSERT INTO sales (name, quantity, unit_price, total_price, payment_method, paid_amount, initial_paid, client_name, phone, imei, sale_type, status, created_at) VALUES ('Peça', 2, 50, 100, 'Cash', 100, 100, '', '', 'ZZZ', 'rapida', 'quitado', '2026-03-01 10:00:00')",
                [],
            )
            .expect("venda sem itens");
        db.conn
            .execute("DELETE FROM sale_items WHERE sale_id = 1", [])
            .expect("remover linhas");
        let sale = db.get_sale_by_id(1).expect("venda");
        assert_eq!(sale.items.len(), 1);
        assert_eq!(sale.items[0].name, "Peça");
        assert_eq!(sale.items[0].quantity, 2);
        assert_eq!(sale.items[0].imei, "ZZZ");
    }

    #[test]
    fn excluir_venda_remove_linhas_e_abate_metricas() {
        let mut db = banco_teste();
        let v = nova_venda_mista(
            vec![
                ("iPhone 12", "1", "2500", "IMEI1", SaleType::Aparelho),
                ("Capinha", "1", "40", "", SaleType::Rapida),
            ],
            "",
            "Ana",
            "",
            false,
        );
        let id = db.add_sale(&v).expect("multi-item");
        assert!(quase_igual(db.total_sales(), 2540.0));
        db.delete_sale(id).expect("exclusão");
        assert!(db.get_sale_by_id(id).is_none());
        assert!(db.get_items_for_sale(id).is_empty());
        assert_eq!(db.count_sales(), 0);
        assert!(quase_igual(db.total_sales(), 0.0));
        assert!(quase_igual(db.net_balance_global(), 0.0));
        // Segunda exclusão do mesmo ID falha (idempotência de erro).
        assert!(db.delete_sale(id).is_err());
        assert!(db.delete_sale(99999).is_err());
    }

    #[test]
    fn excluir_fiado_remove_divida_mas_mantem_abates() {
        let mut db = banco_teste();
        let v = nova_venda(
            "Conserto",
            "300",
            Some(PaymentMethod::Cash),
            "",
            "Rui",
            true,
        );
        db.add_sale(&v).expect("pendente");
        let sale = db.get_pending_debts("Rui", None).pop().expect("dívida");
        db.register_abate_fiado_sem_meio(sale.id, 120.0)
            .expect("abate");
        assert!(quase_igual(db.total_receivables_global(), 180.0));
        let caixa_antes = db.total_cash_in();
        assert!(caixa_antes > 0.0);
        db.delete_sale(sale.id).expect("exclusão do fiado");
        // Dívida some; o dinheiro do abate permanece no caixa (auditoria).
        assert!(quase_igual(db.total_receivables_global(), 0.0));
        assert!(quase_igual(db.total_cash_in(), caixa_antes));
        assert!(db.get_pending_debts("", None).is_empty());
    }

    fn venda_item_direta(
        db: &Database,
        sale_nome: &str,
        item_nome: &str,
        qtd: i64,
        unit: f64,
        tipo: SaleType,
        criado_em: &str,
    ) {
        // Insere cabeçalho + 1 linha de item com `created_at` e `sale_type`
        // controlados, sem passar pelas validações de balcão (IMEI/cliente).
        db.conn
            .execute(
                "INSERT INTO sales (name, quantity, unit_price, total_price, payment_method, paid_amount, initial_paid, client_name, phone, imei, sale_type, status, created_at) VALUES (?1, ?2, ?3, ?4, 'Cash', ?4, ?4, '', '', '', ?5, 'quitado', ?6)",
                params![
                    sale_nome,
                    qtd,
                    unit,
                    qtd as f64 * unit,
                    tipo.as_str(),
                    criado_em
                ],
            )
            .expect("inserir venda de teste");
        let sale_id: i64 = db
            .conn
            .query_row("SELECT last_insert_rowid()", [], |r| r.get(0))
            .expect("rowid");
        db.conn
            .execute(
                "DELETE FROM sale_items WHERE sale_id = ?1",
                params![sale_id],
            )
            .expect("limpar backfill");
        db.conn
            .execute(
                "INSERT INTO sale_items (sale_id, name, quantity, unit_price, total_price, imei, item_type, position) VALUES (?1, ?2, ?3, ?4, ?5, '', ?6, 0)",
                params![sale_id, item_nome, qtd, unit, qtd as f64 * unit, tipo.as_str()],
            )
            .expect("inserir item de teste");
    }

    fn resumo_por_nome(resumos: &[ItemSalesSummary], nome: &str) -> ItemSalesSummary {
        resumos
            .iter()
            .find(|r| r.name == nome)
            .unwrap_or_else(|| panic!("resumo ausente para {nome}"))
            .clone()
    }

    #[test]
    fn agregacao_mantem_submodelos_distintos() {
        let db = banco_teste();
        venda_item_direta(
            &db,
            "S1",
            "iPhone 12",
            2,
            2500.0,
            SaleType::Aparelho,
            "2026-10-05 10:00:00",
        );
        venda_item_direta(
            &db,
            "S2",
            "iPhone 12 Pro",
            3,
            3000.0,
            SaleType::Aparelho,
            "2026-10-06 10:00:00",
        );
        venda_item_direta(
            &db,
            "S3",
            "iPhone 12 Pro Max",
            1,
            3500.0,
            SaleType::Aparelho,
            "2026-10-07 10:00:00",
        );
        venda_item_direta(
            &db,
            "S4",
            "iPhone 14",
            4,
            4000.0,
            SaleType::Aparelho,
            "2026-10-08 10:00:00",
        );
        venda_item_direta(
            &db,
            "S5",
            "iPhone 14 Plus",
            2,
            4200.0,
            SaleType::Aparelho,
            "2026-10-09 10:00:00",
        );
        let res = db
            .get_aggregated_item_sales("2026-10-01", "2026-10-31", None)
            .expect("agregação");
        // 5 sub-modelos, jamais colapsados.
        assert_eq!(res.len(), 5);
        assert_eq!(resumo_por_nome(&res, "iPhone 12").total_quantity, 2);
        assert_eq!(resumo_por_nome(&res, "iPhone 12 Pro").total_quantity, 3);
        assert_eq!(resumo_por_nome(&res, "iPhone 12 Pro Max").total_quantity, 1);
        assert_eq!(resumo_por_nome(&res, "iPhone 14").total_quantity, 4);
        assert_eq!(resumo_por_nome(&res, "iPhone 14 Plus").total_quantity, 2);
        assert!(quase_igual(
            resumo_por_nome(&res, "iPhone 12 Pro").total_revenue,
            9000.0
        ));
        assert_eq!(resumo_por_nome(&res, "iPhone 14").sales_count, 1);
        // Ordenado por quantidade DESC.
        let qtds: Vec<i64> = res.iter().map(|r| r.total_quantity).collect();
        let mut ordenado = qtds.clone();
        ordenado.sort_unstable_by(|a, b| b.cmp(a));
        assert_eq!(qtds, ordenado);
    }

    #[test]
    fn agregacao_normaliza_caixa_e_espacos_das_bordas() {
        let db = banco_teste();
        venda_item_direta(
            &db,
            "S1",
            "  iPhone 12 ",
            1,
            2500.0,
            SaleType::Aparelho,
            "2026-10-05 10:00:00",
        );
        venda_item_direta(
            &db,
            "S2",
            "IPHONE 12",
            2,
            2500.0,
            SaleType::Aparelho,
            "2026-10-06 10:00:00",
        );
        venda_item_direta(
            &db,
            "S3",
            "iphone 12",
            3,
            2500.0,
            SaleType::Aparelho,
            "2026-10-07 10:00:00",
        );
        venda_item_direta(
            &db,
            "S4",
            "iPhone 12 Pro Max",
            1,
            3500.0,
            SaleType::Aparelho,
            "2026-10-08 10:00:00",
        );
        let res = db
            .get_aggregated_item_sales("2026-10-01", "2026-10-31", None)
            .expect("agregação");
        // Variações de caixa/borda colapsam; "Pro Max" segue separado.
        assert_eq!(res.len(), 2);
        let base = res
            .iter()
            .find(|r| r.name.to_lowercase() == "iphone 12")
            .expect("linha base");
        assert_eq!(base.total_quantity, 6);
        assert!(quase_igual(base.total_revenue, 15000.0));
        assert_eq!(base.sales_count, 3);
        assert_eq!(resumo_por_nome(&res, "iPhone 12 Pro Max").total_quantity, 1);
    }

    #[test]
    fn agregacao_respeita_janela_de_datas_inclusiva() {
        let db = banco_teste();
        venda_item_direta(
            &db,
            "Antes",
            "Capinha",
            5,
            40.0,
            SaleType::Rapida,
            "2026-09-30 23:59:59",
        );
        venda_item_direta(
            &db,
            "Inicio",
            "Capinha",
            2,
            40.0,
            SaleType::Rapida,
            "2026-10-01 00:00:01",
        );
        venda_item_direta(
            &db,
            "Fim",
            "Capinha",
            3,
            40.0,
            SaleType::Rapida,
            "2026-10-31 23:00:00",
        );
        venda_item_direta(
            &db,
            "Depois",
            "Capinha",
            7,
            40.0,
            SaleType::Rapida,
            "2026-11-01 00:00:01",
        );
        let res = db
            .get_aggregated_item_sales("2026-10-01", "2026-10-31", None)
            .expect("agregação");
        assert_eq!(res.len(), 1);
        assert_eq!(res[0].total_quantity, 5);
        assert_eq!(res[0].sales_count, 2);
        // Janela vazia retorna vetor vazio, sem erro.
        let vazio = db
            .get_aggregated_item_sales("2025-01-01", "2025-01-31", None)
            .expect("janela vazia");
        assert!(vazio.is_empty());
    }

    #[test]
    fn agregacao_filtra_por_categoria_sem_misturar() {
        let db = banco_teste();
        venda_item_direta(
            &db,
            "A1",
            "iPhone 12",
            2,
            2500.0,
            SaleType::Aparelho,
            "2026-10-05 10:00:00",
        );
        venda_item_direta(
            &db,
            "A2",
            "iPhone 12 Pro Max",
            1,
            3500.0,
            SaleType::Aparelho,
            "2026-10-06 10:00:00",
        );
        venda_item_direta(
            &db,
            "S1",
            "Troca de tela",
            3,
            300.0,
            SaleType::Servico,
            "2026-10-07 10:00:00",
        );
        venda_item_direta(
            &db,
            "R1",
            "Capinha",
            5,
            40.0,
            SaleType::Rapida,
            "2026-10-08 10:00:00",
        );
        venda_item_direta(
            &db,
            "R2",
            "Capinha",
            2,
            40.0,
            SaleType::Rapida,
            "2026-10-09 10:00:00",
        );
        // Todos agrega as 3 categorias.
        let todos = db
            .get_aggregated_item_sales("2026-10-01", "2026-10-31", None)
            .expect("todos");
        assert_eq!(todos.len(), 4);
        assert_eq!(resumo_por_nome(&todos, "Capinha").total_quantity, 7);
        // Aparelhos: só os 2 sub-modelos, ainda distintos.
        let ap = db
            .get_aggregated_item_sales("2026-10-01", "2026-10-31", Some(SaleType::Aparelho))
            .expect("aparelhos");
        assert_eq!(ap.len(), 2);
        assert_eq!(resumo_por_nome(&ap, "iPhone 12").total_quantity, 2);
        assert_eq!(resumo_por_nome(&ap, "iPhone 12 Pro Max").total_quantity, 1);
        // Serviços isolados.
        let sv = db
            .get_aggregated_item_sales("2026-10-01", "2026-10-31", Some(SaleType::Servico))
            .expect("serviços");
        assert_eq!(sv.len(), 1);
        assert_eq!(resumo_por_nome(&sv, "Troca de tela").total_quantity, 3);
        // Acessórios isolados, com as 2 vendas somadas.
        let rp = db
            .get_aggregated_item_sales("2026-10-01", "2026-10-31", Some(SaleType::Rapida))
            .expect("acessórios");
        assert_eq!(rp.len(), 1);
        assert_eq!(resumo_por_nome(&rp, "Capinha").total_quantity, 7);
        assert_eq!(resumo_por_nome(&rp, "Capinha").sales_count, 2);
    }

    #[test]
    fn agregacao_todos_mescla_mesmo_nome_entre_categorias() {
        // Mesmo item vendido no balcão e em bundle de aparelho: Todos soma,
        // cada categoria vê só a sua parte.
        let db = banco_teste();
        venda_item_direta(
            &db,
            "R1",
            "Capinha",
            4,
            40.0,
            SaleType::Rapida,
            "2026-10-05 10:00:00",
        );
        venda_item_direta(
            &db,
            "A1",
            "Capinha",
            1,
            40.0,
            SaleType::Aparelho,
            "2026-10-06 10:00:00",
        );
        let todos = db
            .get_aggregated_item_sales("2026-10-01", "2026-10-31", None)
            .expect("todos");
        assert_eq!(todos.len(), 1);
        assert_eq!(todos[0].total_quantity, 5);
        assert_eq!(todos[0].sales_count, 2);
        let rp = db
            .get_aggregated_item_sales("2026-10-01", "2026-10-31", Some(SaleType::Rapida))
            .expect("rápida");
        assert_eq!(resumo_por_nome(&rp, "Capinha").total_quantity, 4);
        let ap = db
            .get_aggregated_item_sales("2026-10-01", "2026-10-31", Some(SaleType::Aparelho))
            .expect("aparelho");
        assert_eq!(resumo_por_nome(&ap, "Capinha").total_quantity, 1);
        // Categoria sem vendas no período retorna vazio, sem erro.
        let sv = db
            .get_aggregated_item_sales("2026-10-01", "2026-10-31", Some(SaleType::Servico))
            .expect("serviço vazio");
        assert!(sv.is_empty());
    }

    #[test]
    fn acessorio_em_venda_mista_agrega_por_linha() {
        // Cenário real do carrinho misto: cabeçalho `Mista` com Aparelho +
        // Acessório + Serviço, mais uma segunda venda onde "Capinha" sai
        // como linha de Aparelho (bundle). O filtro por categoria isola
        // pela LINHA (`si.item_type`); "Todos" soma tudo.
        let mut db = banco_teste();
        let mista = nova_venda_mista(
            vec![
                ("iPhone 12", "1", "2500", "IMEI1", SaleType::Aparelho),
                ("Capinha", "2", "40", "", SaleType::Rapida),
                ("Troca de tela", "1", "300", "", SaleType::Servico),
            ],
            "",
            "Carlos",
            "",
            false,
        );
        let id = db.add_sale(&mista).expect("mista válida");
        assert_eq!(
            db.get_sale_by_id(id).expect("venda").sale_type,
            SaleType::Mista
        );
        // Move a mista para dentro da janela de outubro (add_sale carimba hoje).
        db.conn
            .execute(
                "UPDATE sales SET created_at = '2026-10-05 10:00:00' WHERE id = ?1",
                params![id],
            )
            .expect("retroagir venda");
        let bundle = nova_venda_mista(
            vec![("Capinha", "1", "40", "IMEI2", SaleType::Aparelho)],
            "",
            "Ana",
            "",
            false,
        );
        let id2 = db.add_sale(&bundle).expect("bundle válido");
        db.conn
            .execute(
                "UPDATE sales SET created_at = '2026-10-06 10:00:00' WHERE id = ?1",
                params![id2],
            )
            .expect("retroagir venda");
        // Acessórios: só a linha Rapida da mista (qtd 2, 1 venda).
        let rp = db
            .get_aggregated_item_sales("2026-10-01", "2026-10-31", Some(SaleType::Rapida))
            .expect("acessórios");
        assert_eq!(resumo_por_nome(&rp, "Capinha").total_quantity, 2);
        assert!(quase_igual(
            resumo_por_nome(&rp, "Capinha").total_revenue,
            80.0
        ));
        assert_eq!(resumo_por_nome(&rp, "Capinha").sales_count, 1);
        // Aparelhos: iPhone + Capinha do bundle, ainda distintos.
        let ap = db
            .get_aggregated_item_sales("2026-10-01", "2026-10-31", Some(SaleType::Aparelho))
            .expect("aparelhos");
        assert_eq!(ap.len(), 2);
        assert_eq!(resumo_por_nome(&ap, "iPhone 12").total_quantity, 1);
        assert_eq!(resumo_por_nome(&ap, "Capinha").total_quantity, 1);
        // Serviços: só a troca de tela da mista.
        let sv = db
            .get_aggregated_item_sales("2026-10-01", "2026-10-31", Some(SaleType::Servico))
            .expect("serviços");
        assert_eq!(sv.len(), 1);
        assert_eq!(
            resumo_por_nome(&sv, "Troca de tela").total_quantity,
            1
        );
        // Todos: mesmo nome em 2 categorias soma (2 + 1 = 3, 2 vendas).
        let todos = db
            .get_aggregated_item_sales("2026-10-01", "2026-10-31", None)
            .expect("todos");
        assert_eq!(resumo_por_nome(&todos, "Capinha").total_quantity, 3);
        assert!(quase_igual(
            resumo_por_nome(&todos, "Capinha").total_revenue,
            120.0
        ));
        assert_eq!(resumo_por_nome(&todos, "Capinha").sales_count, 2);
    }
}
