#pragma once
#ifdef __cplusplus
extern "C" {
#endif

#include <stdbool.h>
#include <stddef.h>

/// Evaluates a Nix expression.
/// 
/// \param code  A null‑terminated C string containing the Nix expression.
/// \returns     A newly‑allocated C string containing the result; or NULL on error.
///              Caller must free() it via free_cstring().
char *eval_nix_expr(const char *code);

/// Frees a C string previously returned by eval_nix_expr.
/// 
/// \param s     Pointer returned by eval_nix_expr; no‑op if NULL.
void free_cstring(char *s);

#ifdef __cplusplus
}
#endif
