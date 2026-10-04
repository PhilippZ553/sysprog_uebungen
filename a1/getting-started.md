# Rust Build Tools
## Rustup
- Toolchain-Installer und Versionsmanager für Rust

## Cargo Package-Manager
- cargo build = Bauen des Projekts
- cargo run = Bauen + Ausführen des Projekts
- cargo test = Sucht Tests und führt diese aus
- cargo doc = Baut automatisch eine Dokumentation
- cargo publish = veröffentlicht projekt auf crates.io
- cargo --version = zeigt aktuell installierte version von cargo an

## create new Project
- cargo new 'Projektname': erstellt Projekt nach Vorlage (.toml, .lock, ./src/)
- Cargo.toml: Konfigurationsdatei des Projekts
- Cargo.lock: Regelt die Versionen aller externen Abhängigkeiten
- src/: Verzeichnis indem der Projektcode steht

## dependencies
- gibt Abhängigkeiten für das Projekt an wie bspw. externe packages
- cargo add 'package-name': fügt eine Paketabhängigkeit zum Projekt hinzu
- werden importiert durch Schlüsselwort 'use' gefolgt von 'package-name'