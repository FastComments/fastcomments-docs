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

### 投票样式

默认的投票样式显示上下箭头。传入 `._1` 可使用爱心样式投票：

```swift
FastCommentsView(sdk: sdk, voteStyle: ._1)
```

| 样式 | 外观 |
|-------|------------|
| `._0` | Up/down arrow buttons with net count |
| `._1` | Single heart button with count |

### 事件回调

使用修饰器样式的回调来处理用户交互：

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

### 应用主题

通过 SwiftUI 环境传入主题：

```swift
FastCommentsView(sdk: sdk)
    .fastCommentsTheme(myTheme)
    .task { try? await sdk.load() }
```

或直接在 SDK 上设置：

```swift
sdk.theme = FastCommentsTheme.modern
```

### 排序方向

```swift
sdk.defaultSortDirection = .nf  // Newest first (default)
sdk.defaultSortDirection = .of  // Oldest first
sdk.defaultSortDirection = .mr  // Most relevant
```

---
---