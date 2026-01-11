Die FastComments Swift SDK besteht aus mehreren Modulen:

- **Client-Modul** - Auto-generierter API-Client für die FastComments REST-APIs
  - Vollständige Typdefinitionen für alle API-Modelle
  - Sowohl authentifizierte (`DefaultAPI`) als auch öffentliche (`PublicAPI`) Endpunkte
  - Volle async/await-Unterstützung
  - Siehe [client/README.md](https://github.com/FastComments/fastcomments-swift/blob/main/client/README.md) für detaillierte API-Dokumentation

- **SSO-Modul** - Serverseitige Single Sign-On-Dienstprogramme
  - Sichere Token-Erzeugung für die Benutzerauthentifizierung
  - Unterstützung für sowohl einfache als auch sichere SSO-Modi
  - Auf HMAC-SHA256 basierende Token-Signierung mit CryptoKit