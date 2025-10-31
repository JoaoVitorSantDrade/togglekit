use tracing::info;

pub fn is_enabled(flag_name: &str, context: Option<()>) -> bool {
    info!(flag_name, "Evaluating feature flag");
    // TODO:
    // 1. Receber o `Ruleset` (conjunto de regras) do cache.
    // 2. Encontrar a `flag_name` no ruleset.
    // 3. Avaliar as regras (Percentage, Segmentos) contra o `context`.
    match flag_name {
        "new-dashboard" => true,
        _ => false,
    }
}
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_flag_evaluation() {
        let context = None; // Contexto vazio por enquanto

        assert_eq!(is_enabled("new-dashboard", context), true);
        assert_eq!(is_enabled("other-flag", context), false);
    }
}
