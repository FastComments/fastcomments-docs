Das FastComments SDK stellt drei API-Clients bereit:

### PublicAPI - Client-sichere Methoden

Die `PublicAPI` enthält Methoden, die sicher aus clientseitigem Code (iOS-/macOS-Apps) aufgerufen werden können. Diese Methoden:
- Benötigen keinen API-Schlüssel
- Können SSO-Tokens zur Authentifizierung verwenden
- Unterliegen pro Benutzer/Gerät einer Ratenbegrenzung
- Eignen sich für an Endnutzer gerichtete Anwendungen

**Beispielanwendung**: Abrufen und Erstellen von Kommentaren in Ihrer iOS-App

### DefaultAPI - Serverseitige Methoden

Die `DefaultAPI` enthält authentifizierte Methoden, die einen API-Schlüssel benötigen. Diese Methoden:
- Benötigen Ihren FastComments API-Schlüssel
- Dürfen NUR aus serverseitigem Code aufgerufen werden
- Gewähren vollen Zugriff auf Ihre FastComments-Daten
- Unterliegen einer Ratenbegrenzung pro Mandant

**Beispielanwendung**: Administrative Vorgänge, Massendatenexport, Benutzerverwaltung

### ModerationAPI - Methoden für das Moderator-Dashboard

Die `ModerationAPI` enthält Methoden, die das Moderator-Dashboard antreiben. Diese Methoden umfassen:
- **Kommentarmoderation** - auflisten, zählen, durchsuchen, Protokolle abrufen und Kommentare exportieren
- **Moderationsaktionen** - Kommentare entfernen/wiederherstellen, markieren, Prüf-/Spam-/Freigabestatus setzen, Stimmen verwalten und Threads wieder öffnen/schließen
- **Verbannungen** - einen Benutzer von einem Kommentar sperren, Sperren rückgängig machen, Vor-Sperr-Zusammenfassungen abrufen, Sperrstatus und -einstellungen prüfen und Anzahl gesperrter Benutzer lesen
- **Abzeichen & Vertrauen** - Abzeichen vergeben/entfernen, manuelle Abzeichen auflisten, den Vertrauensfaktor eines Benutzers abrufen/setzen und das interne Profil eines Benutzers lesen

Jede `ModerationAPI`-Methode akzeptiert einen `sso`-Parameter, sodass Moderatoren per SSO authentifiziert werden können.

**Beispielanwendung**: Aufbau einer Moderationserfahrung für die Moderatoren Ihrer Community

**WICHTIG**: Setzen Sie Ihren API-Schlüssel niemals in clientseitigem Code offen. API-Schlüssel sollten nur serverseitig verwendet werden.