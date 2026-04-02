---
Пошук користувачів для автодоповнення @mention:

```swift
let results = try await sdk.searchUsers(query: "jan")
// Повертає [UserSearchResult] з userId, username, avatar тощо.
```

Вбудований `CommentInputBar` автоматично обробляє автодоповнення @mention.

---
---