`FastCommentsSDK` 和 `FastCommentsFeedSDK` 都是具有 `@Published` 屬性的 `ObservableObject` 類別。您可以在 SwiftUI 視圖中觀察這些屬性，以實現響應式的 UI 更新。

### FastCommentsSDK Published 屬性

| 屬性 | 類型 | 說明 |
|----------|------|-------------|
| `commentCountOnServer` | `Int` | 伺服器上的留言總數 |
| `newRootCommentCount` | `Int` | 緩衝的新留言(當 `showLiveRightAway` 為 false 時) |
| `currentUser` | `UserSessionInfo?` | 目前已驗證的使用者 |
| `isSiteAdmin` | `Bool` | 目前使用者是否為網站管理員 |
| `isClosed` | `Bool` | 留言串是否已關閉 |
| `hasBillingIssue` | `Bool` | 是否存在帳務問題 |
| `isLoading` | `Bool` | 是否有網路請求正在進行中 |
| `hasMore` | `Bool` | 是否存在更多頁的留言 |
| `blockingErrorMessage` | `String?` | 阻止 UI 正常運作的錯誤 |
| `warningMessage` | `String?` | 非阻斷性的警告訊息 |
| `isDemo` | `Bool` | 是否在示範模式下執行 |
| `commentsVisible` | `Bool` | 留言可見性的切換開關 |
| `toolbarEnabled` | `Bool` | 是否顯示格式化工具列 |

### FastCommentsFeedSDK Published 屬性

| 屬性 | 類型 | 說明 |
|----------|------|-------------|
| `feedPosts` | `[FeedPost]` | 目前已載入的動態貼文 |
| `hasMore` | `Bool` | 是否存在更多頁面 |
| `currentUser` | `UserSessionInfo?` | 目前已驗證的使用者 |
| `blockingErrorMessage` | `String?` | 阻斷性錯誤訊息 |
| `isLoading` | `Bool` | 是否有網路請求正在進行中 |
| `newPostsCount` | `Int` | 自上次載入以來的新貼文數量 |

### 留言樹

留言樹可透過 `sdk.commentsTree` 存取：

```swift
// 用於渲染的可見節點扁平列表
sdk.commentsTree.visibleNodes

// 依 ID 查詢留言
sdk.commentsTree.commentsById["comment-id"]
```