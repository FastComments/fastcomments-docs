### Bearbeiten

```swift
try await sdk.editComment(commentId: commentId, newText: "Updated text")
```

Der Server rendert das HTML neu. Der lokale Kommentar wird automatisch aktualisiert.

### Löschen

```swift
try await sdk.deleteComment(commentId: commentId)
```

Das Löschen eines Kommentars entfernt auch seine Nachkommen aus dem lokalen Baum.

Beide Aktionen sind über das Kommentarkontextmenü in der Benutzeroberfläche verfügbar, wenn der aktuelle Benutzer der Kommentarautor (oder ein Seitenadministrator) ist.

---
---