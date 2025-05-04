
#include <iostream>
#include <cstdlib>
#include "libtvix.h"  // Adjust include path as needed

int main() {
    const char *expr = "builtins.readFile ./LICENSE";
    std::cout << "Evaluating Nix expression: " << expr << std::endl;

    // Call the Rust C API
    char *result = eval_nix_expr(expr);
    if (result == nullptr) {
        std::cerr << "Evaluation failed or returned null." << std::endl;
        return EXIT_FAILURE;
    }

    // Print and free
    std::cout << "Result: " << result << std::endl;
    free_cstring(result);

    // Example with an invalid expression
    const char *bad_expr = "invalid nix code";
    std::cout << "\nEvaluating invalid expression: " << bad_expr << std::endl;
    char *bad_result = eval_nix_expr(bad_expr);
    if (bad_result) {
        std::cout << "Returned: " << bad_result << std::endl;
        free_cstring(bad_result);
    } else {
        std::cout << "No result (null returned)." << std::endl;
    }

    return EXIT_SUCCESS;
}
