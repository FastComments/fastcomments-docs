### 基本的な使用法

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

### 投票スタイル

デフォルトの投票スタイルは上下の矢印を表示します。ハート型の投票にするには `._1` を渡します:

```swift
FastCommentsView(sdk: sdk, voteStyle: ._1)
```

| Style | 外観 |
|-------|------------|
| `._0` | Up/down arrow buttons with net count |
| `._1` | Single heart button with count |

### イベントコールバック

モディファイア形式のコールバックを使ってユーザー操作を処理します:

```swift
FastCommentsView(sdk: sdk)
    .onCommentPosted { comment in
        print("New comment: \(comment.commentHTML)")
    }
    .onReplyClick { renderableComment in
        print("Replying to: \(renderableComment.comment.id)")
    }
    .onUserClick { context, userInfo, source in
        // source は .name または .avatar です
        print("Tapped \(userInfo.displayName)")
    }
```

### テーマの適用

SwiftUI の環境を通じてテーマを渡します:

```swift
FastCommentsView(sdk: sdk)
    .fastCommentsTheme(myTheme)
    .task { try? await sdk.load() }
```

または SDK に直接設定します:

```swift
sdk.theme = FastCommentsTheme.modern
```

### ソート方向

```swift
sdk.defaultSortDirection = .nf  // 新しい順（デフォルト）
sdk.defaultSortDirection = .of  // 古い順
sdk.defaultSortDirection = .mr  // 最も関連性の高い
```

---
---