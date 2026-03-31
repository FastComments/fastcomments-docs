---
Поиск пользователей для автодополнения @mention:

```swift
let results = try await sdk.searchUsers(query: "jan")
// Возвращает [UserSearchResult] с userId, username, avatar и т.д.
```

Встроенный `CommentInputBar` автоматически обрабатывает автозаполнение @mention.

---
---