---
### Rediger

```swift
try await sdk.editComment(commentId: commentId, newText: "Updated text")
```

Serveren renderer HTML'en igen. Den lokale kommentar opdateres automatisk.

### Slet

```swift
try await sdk.deleteComment(commentId: commentId)
```

Sletning af en kommentar fjerner også dens efterkommere fra det lokale træ.

Begge handlinger er tilgængelige via kommentarens kontekstmenu i UI'en, når den aktuelle bruger er kommentarens forfatter (eller en webstedsadministrator).

---
---