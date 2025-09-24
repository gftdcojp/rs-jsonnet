{
  // Process Network Graph for rs-jsonnet project
  // Merkle DAG defining the topological dependencies between components

  // Core Components (Leaf nodes)
  error: {
    type: "core",
    dependencies: [],
    description: "Error types and Result definitions for Jsonnet operations"
  },

  value: {
    type: "core",
    dependencies: ["error"],
    description: "JsonnetValue type and core value operations"
  },

  ast: {
    type: "core",
    dependencies: ["value"],
    description: "Abstract Syntax Tree definitions"
  },

  lexer: {
    type: "parser",
    dependencies: ["ast", "error"],
    description: "Lexical analyzer for tokenizing Jsonnet source"
  },

  parser: {
    type: "parser",
    dependencies: ["ast", "lexer", "error"],
    description: "Parser for converting tokens to AST"
  },

  // Evaluation Components
  eval_context: {
    type: "eval",
    dependencies: ["value", "error"],
    description: "Evaluation context and variable bindings"
  },

  eval_handlers: {
    type: "eval",
    dependencies: ["eval_context", "ast"],
    description: "Handlers for evaluating different AST node types"
  },

  pure_evaluator: {
    type: "eval",
    dependencies: ["eval_context", "eval_handlers", "parser", "error"],
    description: "Pure functional evaluator without side effects"
  },

  stdlib: {
    type: "eval",
    dependencies: ["pure_evaluator", "value", "error"],
    description: "Standard library functions and built-ins"
  },

  // Runtime Components
  runtime_manager: {
    type: "runtime",
    dependencies: ["pure_evaluator", "error"],
    description: "Runtime management for external functions and I/O"
  },

  runtime_db: {
    type: "runtime",
    dependencies: ["runtime_manager"],
    description: "Database interface for runtime state persistence"
  },

  runtime_external: {
    type: "runtime",
    dependencies: ["runtime_manager"],
    description: "External function execution and FFI"
  },

  // Main Evaluator
  evaluator: {
    type: "main",
    dependencies: ["pure_evaluator", "stdlib", "runtime_manager", "parser", "error"],
    description: "Main evaluator combining pure evaluation with runtime capabilities"
  },

  // Library Interface
  lib: {
    type: "interface",
    dependencies: ["evaluator", "error"],
    description: "Public library interface and exports"
  },

  // Build and Test Process
  build: {
    type: "process",
    dependencies: ["lib"],
    description: "Cargo build process"
  },

  test: {
    type: "process",
    dependencies: ["build"],
    description: "Cargo test execution"
  }
}
