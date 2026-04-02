---
Pretražite korisnike radi automatskog dovršavanja @mention:

```swift
let results = try await sdk.searchUsers(query: "jan")
// Vraća [UserSearchResult] sa userId, username, avatar, itd.
```

Ugrađeni `CommentInputBar` automatski podržava dovršavanje @mention.

---
---