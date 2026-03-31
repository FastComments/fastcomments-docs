Buscar usuarios para habilitar el autocompletado de @menciones:

```swift
let results = try await sdk.searchUsers(query: "jan")
// Devuelve [UserSearchResult] con userId, username, avatar, etc.
```

La `CommentInputBar` integrada gestiona el autocompletado de @menciones automáticamente.

---
---