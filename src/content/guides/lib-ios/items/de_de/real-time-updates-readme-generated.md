Nach dem Aufruf von `sdk.load()` abonniert das SDK automatisch WebSocket-Ereignisse für die konfigurierte `urlId`. Die folgenden Ereignisse werden behandelt:

- Neue Kommentare, Bearbeitungen und Löschungen
- Stimmen (neu und entfernt)
- Änderungen beim Anpinnen, Sperren, Melden und Blockieren
- Benutzeranwesenheit (Beitritt/Verlassen)
- Thread öffnen/schließen
- Vergabe von Abzeichen
- Aktualisierungen der Serverkonfiguration

### Live-Anzeige steuern

Standardmäßig werden neue Kommentare anderer Nutzer sofort angezeigt:

```swift
sdk.showLiveRightAway = true   // Standard: sofort anzeigen
```

Setzen Sie dies auf `false`, um neue Kommentare hinter einer „N neue Kommentare“-Schaltfläche zu puffern und dem Benutzer die Wahl zu lassen, wann diese angezeigt werden:

```swift
sdk.showLiveRightAway = false
```

### Benutzeranwesenheit

Online-/Offline-Indikatoren erscheinen automatisch auf Benutzer-Avataren, wenn der Server die Präsenzverfolgung aktiviert. Auf der Clientseite ist keine zusätzliche Konfiguration erforderlich.

---
---