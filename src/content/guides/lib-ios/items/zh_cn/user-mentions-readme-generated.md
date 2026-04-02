搜索用户以支持 @mention 自动完成：

```swift
let results = try await sdk.searchUsers(query: "jan")
// 返回 [UserSearchResult]，包含 userId、username、avatar 等。
```

内置的 `CommentInputBar` 会自动处理 @mention 的自动完成。

---
---