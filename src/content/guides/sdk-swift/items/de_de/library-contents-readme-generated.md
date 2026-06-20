Das FastComments Swift SDK besteht aus mehreren Modulen:

- **Client Module** - API-Client für FastComments REST APIs
  - Vollständige Typdefinitionen für alle API-Modelle
  - Authentifizierte (`DefaultAPI`), öffentliche (`PublicAPI`) und Moderations- (`ModerationAPI`) Methoden
  - Vollständige Unterstützung für async/await
  - Siehe [client/README.md](https://github.com/FastComments/fastcomments-swift/blob/main/client/README.md) für detaillierte API-Dokumentation

- **SSO Module** - Serverseitige Single Sign-On-Dienstprogramme
  - Sichere Token-Generierung für die Benutzerauthentifizierung
  - Unterstützung für sowohl einfache als auch sichere SSO-Modi
  - Token-Signatur auf HMAC-SHA256-Basis unter Verwendung von CryptoKit