### Abhängigkeiten installieren

```bash
sudo apt install libcpprest-dev libboost-all-dev
```

### Aus dem Quellcode bauen

```bash
mkdir build
cd build
cmake ..
make
```

### Installieren

```bash
sudo make install
```

### Inhalt der Bibliothek

Diese Bibliothek enthält den generierten API-Client und die SSO-Dienstprogramme, um die Arbeit mit der API zu erleichtern.

- [API Client Library Docs](https://github.com/FastComments/fastcomments-cpp/blob/master/client/README.md)

### Öffentliche vs. gesicherte APIs

Für den API-Client gibt es drei Klassen, `DefaultApi`, `PublicApi` und `ModerationApi`. Die `DefaultApi` enthält Methoden, die Ihren API-Schlüssel benötigen, und `PublicApi` enthält
Methoden, die direkt aus einem Browser/Mobilgerät/usw. ohne Authentifizierung aufgerufen werden können. Die `ModerationApi` enthält Methoden, die das Moderatoren-Dashboard antreiben — Auflisten,
Zählen, Suchen, Exportieren und Abrufen von Logs für Kommentare, Moderationsaktionen (Entfernen/Wiederherstellen, Melden, Überprüfungs-/Spam-/Genehmigungsstatus festlegen, Stimmen anpassen, Threads wieder öffnen/schließen),
Sperren (aus einem Kommentar sperren, Sperren rückgängig machen, Vor-Sperr-Zusammenfassungen, Sperrstatus und -einstellungen, Anzahl gesperrter Nutzer) sowie Abzeichen & Vertrauen (Abzeichen vergeben/entfernen, manuelle Abzeichen, Vertrauensfaktor abrufen/setzen, internes Benutzerprofil). Jede `ModerationApi`-Methode akzeptiert einen `sso`-Parameter, sodass der Aufruf im Namen eines per SSO authentifizierten Moderators durchgeführt wird.