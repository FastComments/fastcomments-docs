Das FastComments SDK stellt drei API-Clients bereit:

### PublicAPI – Client-sichere Methoden

Die `PublicAPI` enthält Methoden, die sicher vom clientseitigen Code (iOS/macOS‑Apps) aufgerufen werden können. Diese Methoden:
- Erfordern keinen API‑Schlüssel
- Können SSO‑Token zur Authentifizierung verwenden
- Sind pro Benutzer/Gerät rate‑limitiert
- Sind für End‑User‑Anwendungen geeignet

**Beispielanwendung**: Abrufen und Erstellen von Kommentaren in Ihrer iOS‑App

### DefaultAPI – Serverseitige Methoden

Die `DefaultAPI` enthält authentifizierte Methoden, die einen API‑Schlüssel benötigen. Diese Methoden:
- Benötigen Ihren FastComments‑API‑Schlüssel
- Sollten NUR aus serverseitigem Code aufgerufen werden
- Bieten vollen Zugriff auf Ihre FastComments‑Daten
- Sind pro Mandant rate‑limitiert

**Beispielanwendung**: Administrative Vorgänge, Massendatenexport, Benutzermanagement

### ModerationAPI – Methoden für das Moderator‑Dashboard

Die `ModerationAPI` bietet eine umfangreiche Sammlung von Live‑ und schnellen Moderations‑APIs. Jede `ModerationAPI`‑Methode akzeptiert einen `sso`‑Parameter und kann sich über SSO oder ein FastComments.com‑Session‑Cookie authentifizieren.

**Beispielanwendung**: Erstellung einer Moderations‑Erfahrung für Moderatoren Ihrer Community

**WICHTIG**: Setzen Sie Ihren API‑Schlüssel niemals im clientseitigen Code ein. API‑Schlüssel sollten nur serverseitig verwendet werden.