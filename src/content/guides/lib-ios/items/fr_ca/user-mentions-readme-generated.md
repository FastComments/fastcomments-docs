---
Rechercher des utilisateurs pour prendre en charge l'autocomplétion des @mentions :

```swift
let results = try await sdk.searchUsers(query: "jan")
// Retourne [UserSearchResult] avec userId, username, avatar, etc.
```

La `CommentInputBar` intégrée gère automatiquement l'autocomplétion des @mentions.

---
---