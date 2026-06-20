### PyPI

```bash
pip install fastcomments
```

### Bibliotheksinhalte

Diese Bibliothek enthält zwei Module: den generierten API-Client und die Core-Python-Bibliothek, die handgeschriebene Hilfsfunktionen enthält, um die Arbeit mit der API zu erleichtern, einschließlich SSO-Unterstützung.

- [API-Client-Bibliotheksdokumentation](https://github.com/FastComments/fastcomments-python/blob/main/client/README.md)
- [Dokumentation der Core-Bibliothek, einschließlich SSO-Beispiele](https://github.com/FastComments/fastcomments-python/blob/main/sso/README.md)

### Öffentliche vs. gesicherte APIs

Für den API-Client gibt es drei Klassen: `DefaultApi`, `PublicApi` und `ModerationApi`. Die `DefaultApi` enthält Methoden, die Ihren API-Schlüssel erfordern, und die `PublicApi` enthält Methoden, die direkt aus einem Browser/auf einem mobilen Gerät/etc. ohne Authentifizierung aufgerufen werden können. Die `ModerationApi` betreibt das Moderatoren-Dashboard und enthält Methoden zur Moderation von Kommentaren (auflisten, zählen, suchen, Protokolle, Export), Moderationsaktionen (entfernen/wiederherstellen, markieren, Überprüfungs-/Spam-/Freigabestatus setzen, Stimmen, Thread wieder öffnen/schließen), Sperren (vom Kommentieren ausschließen, rückgängig machen, Vor-Sperr-Zusammenfassungen, Sperrstatus/-einstellungen, Anzahl gesperrter Benutzer) sowie Abzeichen & Vertrauen (Abzeichen vergeben/entfernen, manuelle Abzeichen, Vertrauensfaktor abrufen/setzen, internes Benutzerprofil). Jede Methode der `ModerationApi` akzeptiert einen `sso`-Parameter, sodass sie im Auftrag eines via SSO authentifizierten Moderators aufgerufen werden kann.