@mention 자동완성을 지원하기 위해 사용자를 검색:

```swift
let results = try await sdk.searchUsers(query: "jan")
// [UserSearchResult]를 반환합니다. userId, username, avatar 등을 포함합니다.
```

내장된 `CommentInputBar`가 @mention 자동완성을 자동으로 처리합니다.

---
---