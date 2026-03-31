---
Претражите кориснике за аутоматско допуњавање при помињању (@mention):

```swift
let results = try await sdk.searchUsers(query: "jan")
// Враћа [UserSearchResult] са userId, username, avatar, итд.
```

Уграђени `CommentInputBar` аутоматски обавља допуњавање при помињању (@mention).

---
---