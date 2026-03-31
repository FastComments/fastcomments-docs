---
Søg efter brugere for at understøtte @mention-autofuldførelse:

```swift
let results = try await sdk.searchUsers(query: "jan")
// Returnerer [UserSearchResult] med userId, username, avatar osv.
```

Den indbyggede `CommentInputBar` håndterer @mention-autofuldførelse automatisk.

---
---