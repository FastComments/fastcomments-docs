Претрага корисника за подршку аутоматском довршавању @помена:

```swift
let results = try await sdk.searchUsers(query: "jan")
// Враћа [UserSearchResult] са userId, username, avatar итд.
```

Уграђени `CommentInputBar` аутоматски обрађује довршавање @помена.