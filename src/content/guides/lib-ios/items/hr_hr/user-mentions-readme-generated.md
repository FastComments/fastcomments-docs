Pretraživanje korisnika za podršku automatskom dovršavanju @mention:

```swift
let results = try await sdk.searchUsers(query: "jan")
// Vraća [UserSearchResult] s userId, username, avatar, itd.
```

Ugrađeni `CommentInputBar` automatski upravlja dovršavanjem @mention.

---
---