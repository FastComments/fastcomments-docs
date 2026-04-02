Nach Benutzern suchen, um die @mention-Autovervollständigung zu unterstützen:

```swift
let results = try await sdk.searchUsers(query: "jan")
// Gibt [UserSearchResult] mit userId, username, avatar usw. zurück.
```

Die integrierte `CommentInputBar` kümmert sich automatisch um die @mention-Autovervollständigung.

---
---