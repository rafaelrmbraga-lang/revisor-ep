//! Lógica pura do motor de atualização da base de regras.
//! Sem dependência de Tauri — testável isoladamente (ver testes ao final).

use serde_json::Value;
use sha2::{Digest, Sha256};

/// Schema que este binário sabe interpretar. Bases com schema maior são recusadas
/// (o usuário é avisado de que precisa atualizar o aplicativo).
pub const SCHEMA_SUPORTADO: u64 = 1;

/// Compara versões no formato "AAAA.MM.N". Retorna true se `nova` > `atual`.
pub fn versao_mais_nova(nova: &str, atual: &str) -> bool {
    let t = |v: &str| -> Vec<u64> {
        v.trim_start_matches('v')
            .split('.')
            .map(|p| p.trim().parse::<u64>().unwrap_or(0))
            .collect()
    };
    t(nova) > t(atual)
}

/// Valida uma base candidata: JSON íntegro, meta.versao presente, schema suportado.
/// Devolve (versao, schema_version).
pub fn validar_base(conteudo: &str) -> Result<(String, u64), String> {
    let v: Value =
        serde_json::from_str(conteudo).map_err(|e| format!("base corrompida (JSON inválido): {e}"))?;
    let versao = v["meta"]["versao"]
        .as_str()
        .ok_or("base sem meta.versao")?
        .to_string();
    let schema = v["meta"]["schema_version"]
        .as_u64()
        .ok_or("base sem meta.schema_version")?;
    if schema > SCHEMA_SUPORTADO {
        return Err(format!(
            "schema {schema} é mais novo que o suportado ({SCHEMA_SUPORTADO}) — atualize o aplicativo"
        ));
    }
    Ok((versao, schema))
}

pub fn sha256_hex(bytes: &[u8]) -> String {
    let mut h = Sha256::new();
    h.update(bytes);
    h.finalize().iter().map(|b| format!("{b:02x}")).collect()
}

/// Interpreta o manifest e decide. Ok(None) = já está em dia.
/// Ok(Some((url, sha256, versao))) = há base nova compatível para baixar.
pub fn decidir(manifest: &str, versao_atual: &str) -> Result<Option<(String, String, String)>, String> {
    let m: Value =
        serde_json::from_str(manifest).map_err(|e| format!("manifest inválido: {e}"))?;
    let versao = m["base_versao"].as_str().ok_or("manifest sem base_versao")?;
    let schema = m["schema_version"].as_u64().ok_or("manifest sem schema_version")?;
    if !versao_mais_nova(versao, versao_atual) {
        return Ok(None);
    }
    if schema > SCHEMA_SUPORTADO {
        return Err(format!(
            "há base nova (v{versao}), mas ela exige aplicativo mais recente (schema {schema})"
        ));
    }
    let url = m["url"].as_str().ok_or("manifest sem url")?.to_string();
    let sha = m["sha256"].as_str().ok_or("manifest sem sha256")?.to_lowercase();
    Ok(Some((url, sha, versao.to_string())))
}

#[cfg(test)]
mod testes {
    use super::*;

    #[test]
    fn compara_versoes() {
        assert!(versao_mais_nova("2026.09.0", "2026.08.2"));
        assert!(versao_mais_nova("2026.08.10", "2026.08.2")); // numérico, não lexicográfico
        assert!(!versao_mais_nova("2026.08.2", "2026.08.2"));
        assert!(!versao_mais_nova("2025.12.9", "2026.01.0"));
        assert!(versao_mais_nova("v2027.01.0", "2026.08.2"));
    }

    #[test]
    fn valida_base_ok_e_erros() {
        let ok = r#"{"meta":{"versao":"2026.08.2","schema_version":1}}"#;
        assert_eq!(validar_base(ok).unwrap(), ("2026.08.2".into(), 1));
        assert!(validar_base("{quebrado").is_err());
        assert!(validar_base(r#"{"meta":{"schema_version":1}}"#).is_err());
        let futuro = r#"{"meta":{"versao":"2027.01.0","schema_version":9}}"#;
        assert!(validar_base(futuro).unwrap_err().contains("atualize o aplicativo"));
    }

    #[test]
    fn sha256_conhecido() {
        // sha256("teste") — vetor conhecido
        assert_eq!(
            sha256_hex(b"teste"),
            "46070d4bf934fb0d4b06d9e2c46e346944e322444900a435d7d9a95e6d7435f5"
        );
    }

    #[test]
    fn decide_atualizar_em_dia_e_incompativel() {
        let m_nova = r#"{"base_versao":"2026.09.0","schema_version":1,"url":"https://x/base.json","sha256":"AB12"}"#;
        let d = decidir(m_nova, "2026.08.2").unwrap().unwrap();
        assert_eq!(d.2, "2026.09.0");
        assert_eq!(d.1, "ab12"); // normaliza p/ minúsculas
        let m_igual = r#"{"base_versao":"2026.08.2","schema_version":1,"url":"u","sha256":"s"}"#;
        assert!(decidir(m_igual, "2026.08.2").unwrap().is_none());
        let m_fut = r#"{"base_versao":"2026.09.0","schema_version":9,"url":"u","sha256":"s"}"#;
        assert!(decidir(m_fut, "2026.08.2").unwrap_err().contains("aplicativo mais recente"));
    }
}
