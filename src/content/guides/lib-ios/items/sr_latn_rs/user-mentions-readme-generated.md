Pretražite korisnike da biste podržali automatsko dovršavanje @mention:

```swift
let results = try await sdk.searchUsers(query: "jan")
// Vraća [UserSearchResult] sa userId, username, avatar, itd.
```

Ugrađeni `CommentInputBar` automatski omogućava dovršavanje @mention.

---
---