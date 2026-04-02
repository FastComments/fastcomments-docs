@mention otomatik tamamlama için kullanıcıları arayın:

```swift
let results = try await sdk.searchUsers(query: "jan")
// [UserSearchResult] döner; userId, username, avatar vb.
```

Yerleşik `CommentInputBar`, @mention otomatik tamamlama özelliğini yönetir.

---
---