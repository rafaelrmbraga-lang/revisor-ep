// Revisor de Execução Penal — núcleo Tauri
// Princípios: leitura 100% local de PDFs; nada de dados de assistidos gravados;
// base de regras embarcada, com atualização silenciosa, verificação SHA-256 e rollback.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod motor;
use std::fs;
use std::path::PathBuf;
use tauri::Manager;

/// Base validada embutida no binário (fallback eterno: offline nunca quebra).
const BASE_EMBUTIDA: &str = include_str!("../base.json");

/// Manifesto da base no GitHub (preencher OWNER/REPO na publicação — ver ATUALIZACAO.md).
const MANIFEST_URL: &str =
    "https://github.com/rafaelrmbraga-lang/revisor-ep/releases/latest/download/manifest.json";

fn dir_dados(app: &tauri::AppHandle) -> Result<PathBuf, String> {
    let d = app.path().app_data_dir().map_err(|e| e.to_string())?;
    fs::create_dir_all(&d).map_err(|e| e.to_string())?;
    Ok(d)
}

/// Devolve a melhor base disponível: local válida > embutida.
/// Base local corrompida é posta de lado (quarentena) e o app segue com a embutida.
fn ler_base(app: &tauri::AppHandle) -> (String, String) {
    if let Ok(dir) = dir_dados(app) {
        let p = dir.join("base.json");
        if let Ok(txt) = fs::read_to_string(&p) {
            match motor::validar_base(&txt) {
                Ok((v, _)) => return (txt, format!("local v{v}")),
                Err(_) => {
                    let _ = fs::rename(&p, dir.join("base-corrompida.json"));
                }
            }
        }
    }
    let v = motor::validar_base(BASE_EMBUTIDA)
        .map(|(v, _)| v)
        .unwrap_or_else(|_| "?".into());
    (BASE_EMBUTIDA.to_string(), format!("embutida v{v}"))
}

#[tauri::command]
fn get_base(app: tauri::AppHandle) -> String {
    ler_base(&app).0
}

#[tauri::command]
fn get_base_info(app: tauri::AppHandle) -> String {
    ler_base(&app).1
}

/// Verifica o manifesto e, havendo base nova compatível, baixa, confere o SHA-256,
/// valida o conteúdo, guarda a anterior (rollback) e troca de forma atômica.
/// Retornos: "atualizada:vX" | "em_dia:vX" | Err(mensagem amigável).
#[tauri::command]
fn atualizar_base(app: tauri::AppHandle) -> Result<String, String> {
    let (atual_txt, _) = ler_base(&app);
    let (versao_atual, _) = motor::validar_base(&atual_txt)?;

    let cli = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(15))
        .user_agent("revisor-ep")
        .build()
        .map_err(|e| e.to_string())?;

    let manifest = cli
        .get(MANIFEST_URL)
        .send()
        .and_then(|r| r.error_for_status())
        .and_then(|r| r.text())
        .map_err(|_| "sem conexão ou manifesto indisponível (o aplicativo segue com a base atual)".to_string())?;

    let alvo = match motor::decidir(&manifest, &versao_atual)? {
        None => return Ok(format!("em_dia:v{versao_atual}")),
        Some(a) => a,
    };
    let (url, sha_esperado, versao_nova) = alvo;

    let bytes = cli
        .get(&url)
        .send()
        .and_then(|r| r.error_for_status())
        .and_then(|r| r.bytes())
        .map_err(|e| format!("falha ao baixar a base nova: {e}"))?;

    if motor::sha256_hex(&bytes) != sha_esperado {
        return Err("verificação de integridade falhou (SHA-256 divergente) — base recusada".into());
    }
    let texto = String::from_utf8(bytes.to_vec()).map_err(|_| "base nova ilegível".to_string())?;
    motor::validar_base(&texto)?;

    let dir = dir_dados(&app)?;
    let destino = dir.join("base.json");
    if destino.exists() {
        let _ = fs::copy(&destino, dir.join("base-anterior.json")); // rollback manual possível
    }
    let tmp = dir.join("base.json.tmp");
    fs::write(&tmp, &texto).map_err(|e| e.to_string())?;
    fs::rename(&tmp, &destino).map_err(|e| e.to_string())?; // troca atômica

    Ok(format!("atualizada:v{versao_nova}"))
}

#[tauri::command]
fn parse_pdf(path: String) -> Result<String, String> {
    let bytes = std::fs::read(&path).map_err(|e| format!("Não consegui abrir o arquivo: {e}"))?;
    pdf_extract::extract_text_from_mem(&bytes).map_err(|e| {
        format!("PDF sem camada de texto ou ilegível ({e}). Gere o RSPE novamente no SEEU (não escaneado).")
    })
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_base,
            get_base_info,
            atualizar_base,
            parse_pdf
        ])
        .run(tauri::generate_context!())
        .expect("erro ao iniciar o Revisor");
}
