# Developer Guide

Instructions for contributing to forensic-db development.

## Development Setup

### Clone Repository

```bash
git clone https://github.com/0o-de-lally/forensic-db.git
cd forensic-db
```

### Build

```bash
cargo build
cargo build --release
```

### Testing

```bash
cargo test
cargo test --release
```

Run integration tests with Neo4j:

```bash
# Set up Neo4j credentials
export LIBRA_GRAPH_DB_URI='neo4j://localhost'
export LIBRA_GRAPH_DB_USER='neo4j'
export LIBRA_GRAPH_DB_PASS='your-password'

cargo test --test integration
```

## Project Structure

```
forensic-db/
├── src/
│   ├── lib.rs           # Library entry point
│   ├── main.rs          # CLI entry point
│   ├── warehouse_cli.rs # CLI command definitions
│   ├── extract_*.rs     # Extraction modules
│   ├── load_*.rs        # Loading modules
│   ├── enrich_*.rs      # Enrichment modules
│   ├── schema_*.rs      # Data structure definitions
│   └── analytics/       # Analytics modules
├── docs/
│   ├── product/         # User-facing guides
│   ├── technical/       # Architecture & specs
│   ├── development/     # Developer resources
│   └── specs/           # Feature specifications
├── tests/
│   ├── fixtures/        # Test data
│   └── support/         # Test helpers
└── Cargo.toml
```

## Adding New Features

1. Create module in appropriate directory
2. Add `pub mod` declaration to `src/lib.rs`
3. Add CLI subcommand to `src/warehouse_cli.rs` if needed
4. Write tests
5. Update documentation

## Code Style

- Follow Rust idioms and conventions
- Use meaningful variable and function names
- Add comments for complex logic
- Run `cargo fmt` before committing

## Documentation

All substantial documentation goes in `docs/` subdirectories. Use kebab-case for filenames.

See [documentation rules](documentation-rules.md) for details.

## Pull Requests

1. Fork the repository
2. Create feature branch
3. Make changes
4. Add tests
5. Update documentation
6. Submit PR with clear description
