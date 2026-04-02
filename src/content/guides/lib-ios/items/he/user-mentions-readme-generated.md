Search for users to support @mention autocomplete:

```swift
let results = try await sdk.searchUsers(query: "jan")
// מחזיר [UserSearchResult] עם userId, username, avatar וכו'.
```

The built-in `CommentInputBar` מטפל בהשלמה האוטומטית של @mention.

---
---