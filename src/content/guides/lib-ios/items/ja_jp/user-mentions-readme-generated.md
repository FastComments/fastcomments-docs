---
@メンションのオートコンプリートをサポートするユーザーを検索:

```swift
let results = try await sdk.searchUsers(query: "jan")
// [UserSearchResult] を返します（userId、username、avatar など）
```

組み込みの `CommentInputBar` は @メンションのオートコンプリートを自動的に処理します。

---
---