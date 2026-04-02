---
Wyszukaj użytkowników, aby obsłużyć autouzupełnianie @mention:

```swift
let results = try await sdk.searchUsers(query: "jan")
// Zwraca [UserSearchResult] z userId, username, avatar, itd.
```

Wbudowany `CommentInputBar` automatycznie obsługuje autouzupełnianie @mention.

---
---