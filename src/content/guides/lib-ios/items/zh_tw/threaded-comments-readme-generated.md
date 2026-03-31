### 基本用法

```swift
struct CommentsPage: View {
    @StateObject private var sdk = FastCommentsSDK(
        config: FastCommentsWidgetConfig(
            tenantId: "YOUR_TENANT_ID",
            urlId: "article-42",
            url: "https://example.com/article/42",
            pageTitle: "Article Title"
        )
    )

    var body: some View {
        FastCommentsView(sdk: sdk)
            .task {
                try? await sdk.load()
            }
    }
}
```

### 投票樣式

預設的投票樣式顯示上下箭頭。傳入 `._1` 可使用愛心樣式的投票：

```swift
FastCommentsView(sdk: sdk, voteStyle: ._1)
```

| 樣式 | 外觀 |
|-------|------------|
| `._0` | 上下箭頭按鈕，顯示淨票數 |
| `._1` | 單一愛心按鈕，並顯示數量 |

### 事件回調

使用 modifier 風格的回調來處理使用者互動：

```swift
FastCommentsView(sdk: sdk)
    .onCommentPosted { comment in
        print("New comment: \(comment.commentHTML)")
    }
    .onReplyClick { renderableComment in
        print("Replying to: \(renderableComment.comment.id)")
    }
    .onUserClick { context, userInfo, source in
        // source 是 .name 或 .avatar
        print("Tapped \(userInfo.displayName)")
    }
```

### 套用主題

透過 SwiftUI 環境傳入主題：

```swift
FastCommentsView(sdk: sdk)
    .fastCommentsTheme(myTheme)
    .task { try? await sdk.load() }
```

或直接在 SDK 上設定：

```swift
sdk.theme = FastCommentsTheme.modern
```

### 排序方向

```swift
sdk.defaultSortDirection = .nf  // 由新到舊（預設）
sdk.defaultSortDirection = .of  // 由舊到新
sdk.defaultSortDirection = .mr  // 最相關
```