---
Das FastComments Rust SDK besteht aus mehreren Modulen:

- **Client Module** - Automatisch generierter API-Client für FastComments REST APIs
  - Vollständige Typdefinitionen für alle API-Modelle
  - Sowohl authentifizierte (`DefaultApi`) als auch öffentliche (`PublicApi`) Endpunkte
  - Volle async/await-Unterstützung mit `tokio`
  - Siehe [client/README.md](https://github.com/FastComments/fastcomments-rust/blob/main/client/README.md) für detaillierte API-Dokumentation

- **SSO Module** - Serverseitige Single Sign-On-Dienstprogramme
  - Sichere Token-Generierung zur Benutzer-Authentifizierung
  - Unterstützung sowohl für einfache als auch für sichere SSO-Modi
  - HMAC-SHA256-basierte Token-Signierung

- **Core Types** - Gemeinsame Typdefinitionen und Dienstprogramme
  - Kommentar-Modelle und Metadatenstrukturen
  - Benutzer- und Mandantenkonfigurationen
  - Hilfsfunktionen für gängige Operationen
---