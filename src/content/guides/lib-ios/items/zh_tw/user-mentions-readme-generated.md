搜尋使用者以支援 @mention 的自動完成功能：

```swift
let results = try await sdk.searchUsers(query: "jan")
// 回傳 [UserSearchResult]，包含 userId、username、avatar 等。
```

內建的 `CommentInputBar` 會自動處理 @mention 的自動完成功能。

---
---