### Verwendung von Nimble

```bash
nimble install fastcomments
```

### Aus dem Quellcode bauen

```bash
nimble build
```

### Inhalt der Bibliothek

Diese Bibliothek enthält den generierten API-Client und die SSO-Dienstprogramme, um die Arbeit mit der API zu vereinfachen.

- [Dokumentation der API-Client-Bibliothek](https://github.com/FastComments/fastcomments-nim/blob/master/client/README.md)

### Öffentliche vs. gesicherte APIs

Für den API-Client gibt es drei API-Module, `api_default`, `api_public`, und `api_moderation`. Das `api_default` enthält Methoden, die Ihren API-Schlüssel erfordern, und `api_public` enthält API-Aufrufe
die direkt aus einem Browser/mobilem Gerät/etc. ohne Authentifizierung ausgeführt werden können. Das `api_moderation`-Modul enthält Methoden für das Moderatoren-Dashboard.

Die `api_moderation`-Methoden decken das Auflisten, Zählen, Suchen und Exportieren von Kommentaren und deren Protokollen ab; Moderationsaktionen wie Entfernen/Wiederherstellen von Kommentaren, Markieren, Festlegen des Review-/Spam-/Genehmigungsstatus, Anpassen von Stimmen und Wiederöffnen/Schließen von Threads; Sperren (einen Benutzer von einem Kommentar sperren, eine Sperre rückgängig machen, Vor-Sperr-Zusammenfassungen, Sperrstatus und -einstellungen sowie Anzahl gesperrter Benutzer); und Abzeichen & Vertrauen (Verleihen/Entfernen eines Abzeichens, Auflisten manueller Abzeichen, Abrufen/Setzen des Vertrauensfaktors eines Benutzers und Abrufen des internen Profils eines Benutzers). Jede `api_moderation`-Methode akzeptiert einen `sso`-Parameter, sodass der Aufruf als SSO-Moderator authentifiziert wird.