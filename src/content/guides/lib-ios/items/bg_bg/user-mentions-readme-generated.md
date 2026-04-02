---
Търсене на потребители за автоматичното довършване при @mention:

```swift
let results = try await sdk.searchUsers(query: "jan")
// Връща [UserSearchResult] с userId, username, avatar и т.н.
```

Вградената `CommentInputBar` се грижи за автоматичното довършване на @mention.

---
---