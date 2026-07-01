### Aktionen, die allen Benutzern zur Verfügung stehen

- **Flag/Unflag** -- einen Kommentar zur Überprüfung melden

```swift
try await sdk.flagComment(commentId: commentId)
try await sdk.unflagComment(commentId: commentId)
```

- **Block/Unblock** -- alle Kommentare eines Benutzers ausblenden (pro Betrachter)

```swift
try await sdk.blockUser(commentId: commentId)
try await sdk.unblockUser(commentId: commentId)
```

### Nur für Administratoren verfügbare Aktionen

- **Pin/Unpin** -- einen Kommentar an den Anfang des Threads anheften

```swift
try await sdk.pinComment(commentId: commentId)
try await sdk.unpinComment(commentId: commentId)
```

- **Lock/Unlock** -- neue Antworten auf einen Kommentar verhindern und Bearbeitungen sowie Löschungen blockieren, bis er entsperrt ist (gilt für alle, einschließlich Moderatoren)

```swift
try await sdk.lockComment(commentId: commentId)
try await sdk.unlockComment(commentId: commentId)
```

Alle Moderationsaktionen sind ebenfalls über das Kontextmenü des Kommentars in der UI verfügbar. Administrator‑Aktionen werden nur angezeigt, wenn der aktuelle Benutzer ein Site‑Administrator ist (gesetzt über das SSO `isAdmin`‑Flag oder die Dashboard‑Konfiguration).