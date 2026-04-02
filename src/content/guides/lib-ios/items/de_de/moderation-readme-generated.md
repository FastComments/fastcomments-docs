### Für alle Benutzer verfügbare Aktionen

- **Melden/Markierung aufheben** -- einen Kommentar zur Überprüfung melden

```swift
try await sdk.flagComment(commentId: commentId)
try await sdk.unflagComment(commentId: commentId)
```

- **Blockieren/Blockierung aufheben** -- alle Kommentare eines Nutzers ausblenden (pro Betrachter)

```swift
try await sdk.blockUser(commentId: commentId)
try await sdk.unblockUser(commentId: commentId)
```

### Nur für Administratoren

- **Anpinnen/Entpinnen** -- einen Kommentar oben im Thread anpinnen

```swift
try await sdk.pinComment(commentId: commentId)
try await sdk.unpinComment(commentId: commentId)
```

- **Sperren/Entsperren** -- neue Antworten auf einen Kommentar verhindern

```swift
try await sdk.lockComment(commentId: commentId)
try await sdk.unlockComment(commentId: commentId)
```

Alle Moderationsaktionen sind außerdem über das Kontextmenü des Kommentars in der Benutzeroberfläche verfügbar. Administratoraktionen erscheinen nur, wenn der aktuelle Benutzer ein Site-Administrator ist (festgelegt über das SSO `isAdmin` Flag oder die Dashboard-Konfiguration).

---
---