Das FastComments Rust SDK besteht aus mehreren Modulen:

- **Client‑Modul** – API‑Client für die FastComments REST‑APIs
  - Vollständige Typdefinitionen für alle API‑Modelle
  - Drei API‑Clients, die alle FastComments‑Methoden abdecken:
    - `default_api` (**DefaultApi**) – API‑Key‑authentifizierte Methoden für die serverseitige Nutzung
    - `public_api` (**PublicApi**) – öffentliche, keine API‑Key‑Methoden, die sicher aus Browsern und mobilen Apps aufgerufen werden können
    - `moderation_api` (**ModerationApi**) – eine umfangreiche Suite von Live‑ und schnellen Moderations‑APIs. Jede Moderations‑Methode akzeptiert einen `sso`‑Parameter und kann über SSO oder ein FastComments.com‑Session‑Cookie authentifizieren.
  - Vollständige async/await‑Unterstützung mit Tokio
  - Siehe [client/README.md](https://github.com/FastComments/fastcomments-rust/blob/main/client/README.md) für die detaillierte API‑Dokumentation

- **SSO‑Modul** – serverseitige Single Sign‑On‑Hilfsmittel
  - Sichere Token‑Erstellung für die Benutzer‑Authentifizierung
  - Unterstützung für einfache und sichere SSO‑Modi
  - Auf HMAC‑SHA256 basierende Token‑Signierung

- **Core‑Typen** – gemeinsam genutzte Typdefinitionen und Hilfsprogramme
  - Kommentarmodelle und Metadaten‑Strukturen
  - Benutzer‑ und Mandanten‑Konfigurationen
  - Hilfsfunktionen für gängige Vorgänge