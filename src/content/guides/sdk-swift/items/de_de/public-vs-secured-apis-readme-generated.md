Das FastComments SDK bietet zwei Arten von API-Endpunkten:

### PublicAPI - Client-sichere Endpunkte

Die `PublicAPI` enthält Endpunkte, die sicher von clientseitigem Code (iOS/macOS-Apps) aufgerufen werden können. Diese Endpunkte:
- Benötigen keinen API key
- Können SSO-Tokens für die Authentifizierung verwenden
- Unterliegen einer Ratenbegrenzung pro Benutzer/Gerät
- Eignen sich für Endanwender-Anwendungen

**Beispielanwendung**: Abrufen und Erstellen von Kommentaren in Ihrer iOS-App

### DefaultAPI - Serverseitige Endpunkte

Die `DefaultAPI` enthält authentifizierte Endpunkte, die einen API key erfordern. Diese Endpunkte:
- Erfordern Ihren FastComments API key
- Sollten AUSSCHLIESSLICH von serverseitigem Code aufgerufen werden
- Bieten vollen Zugriff auf Ihre FastComments-Daten
- Unterliegen einer Ratenbegrenzung pro Mandant

**Beispielanwendung**: Administrative Vorgänge, Massenexport von Daten, Moderationstools

**WICHTIG**: Geben Sie Ihren API key niemals in clientseitigem Code preis. API keys sollten nur serverseitig verwendet werden.