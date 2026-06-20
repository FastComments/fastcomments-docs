---
Das FastComments Rust SDK besteht aus mehreren Modulen:

- **Client Module** - API-Client für die FastComments REST-APIs
  - Vollständige Typdefinitionen für alle API-Modelle
  - Drei API-Clients, die alle FastComments-Methoden abdecken:
    - `default_api` (**DefaultApi**) - mit API-Schlüssel authentifizierte Methoden für die serverseitige Verwendung
    - `public_api` (**PublicApi**) - öffentliche Methoden ohne API-Schlüssel, die sicher aus Browsern und mobilen Apps aufgerufen werden können
    - `moderation_api` (**ModerationApi**) - Methoden für das Moderator-Dashboard, einschließlich Kommentar-Moderation (Auflisten, Zählen, Suchen, Protokolle, Export), Moderationsaktionen (entfernen/wiederherstellen, melden, Review-/Spam-/Genehmigungsstatus setzen, Stimmen, Thread wieder öffnen/schließen), Sperren (Sperre aufgrund eines Kommentars, rückgängig machen, Vor-Sperr-Zusammenfassungen, Sperrstatus/-einstellungen, Anzahl gesperrter Nutzer) sowie Abzeichen & Vertrauen (Abzeichen vergeben/entfernen, manuelle Abzeichen, Trust-Faktor abrufen/setzen, internes Nutzerprofil). Jede Moderationsmethode akzeptiert einen `sso`-Parameter, sodass der Aufruf im Namen eines per SSO authentifizierten Moderators erfolgen kann.
  - Vollständige async/await-Unterstützung mit tokio
  - Siehe [client/README.md](https://github.com/FastComments/fastcomments-rust/blob/main/client/README.md) für detaillierte API-Dokumentation

- **SSO Module** - Serverseitige Single Sign-On-Dienstprogramme
  - Sichere Token-Erzeugung zur Benutzer-Authentifizierung
  - Unterstützung für sowohl einfache als auch sichere SSO-Modi
  - Token-Signierung basierend auf HMAC-SHA256

- **Core Types** - Gemeinsame Typdefinitionen und Hilfsfunktionen
  - Kommentar-Modelle und Metadatenstrukturen
  - Benutzer- und Mandantenkonfigurationen
  - Hilfsfunktionen für häufige Operationen
---