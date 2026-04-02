---
Iskanje uporabnikov za podporo samodokončanju omemb (@mention):

```swift
let results = try await sdk.searchUsers(query: "jan")
// Vrne [UserSearchResult] z userId, username, avatar, itd.
```

Vgrajeni `CommentInputBar` samodejno obravnava samodokončanje omemb (@mention).

---
---