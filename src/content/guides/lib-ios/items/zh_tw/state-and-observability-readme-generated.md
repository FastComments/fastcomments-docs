---
Both `FastCommentsSDK` and `FastCommentsFeedSDK` are `ObservableObject` classes with `@Published` properties. You can observe these in your SwiftUI views for reactive UI updates.

### FastCommentsSDK Published Properties

| Property | Type | Description |
|----------|------|-------------|
| `commentCountOnServer` | `Int` | 伺服器上的評論總數 |
| `newRootCommentCount` | `Int` | 緩衝的新評論（當 `showLiveRightAway` 為 false 時） |
| `currentUser` | `UserSessionInfo?` | 當前已驗證的使用者 |
| `isSiteAdmin` | `Bool` | 當前使用者是否為網站管理員 |
| `isClosed` | `Bool` | 評論串是否已關閉 |
| `hasBillingIssue` | `Bool` | 是否存在帳單問題 |
| `isLoading` | `Bool` | 是否有網路請求正在進行 |
| `hasMore` | `Bool` | 是否存在更多評論頁面 |
| `blockingErrorMessage` | `String?` | 阻礙 UI 正常運作的錯誤 |
| `warningMessage` | `String?` | 非阻斷的警告訊息 |
| `isDemo` | `Bool` | 是否以示範模式執行 |
| `commentsVisible` | `Bool` | 評論可見性的切換 |
| `toolbarEnabled` | `Bool` | 是否顯示格式工具列 |

### FastCommentsFeedSDK Published Properties

| Property | Type | Description |
|----------|------|-------------|
| `feedPosts` | `[FeedPost]` | 目前已載入的 Feed 帖文 |
| `hasMore` | `Bool` | 是否存在更多頁面 |
| `currentUser` | `UserSessionInfo?` | 當前已驗證的使用者 |
| `blockingErrorMessage` | `String?` | 阻斷性錯誤訊息 |
| `isLoading` | `Bool` | 是否有網路請求正在進行 |
| `newPostsCount` | `Int` | 自上次載入以來的新帖數量 |

### Comment Tree

The comment tree is accessible via `sdk.commentsTree`:

```swift
// 用於渲染的可見節點扁平列表
sdk.commentsTree.visibleNodes

// 透過 ID 查找評論
sdk.commentsTree.commentsById["comment-id"]
```

---
---