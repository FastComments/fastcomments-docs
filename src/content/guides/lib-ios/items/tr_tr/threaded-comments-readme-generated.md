### Temel Kullanım

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

### Oy Stilleri

Varsayılan oy stili yukarı/aşağı oklarını gösterir. Kalp-stili oylar için `._1` kullanın:

```swift
FastCommentsView(sdk: sdk, voteStyle: ._1)
```

| Stil | Görünüm |
|-------|------------|
| `._0` | Yukarı/aşağı ok düğmeleri ve net sayı |
| `._1` | Tek kalp düğmesi ve sayı |

### Olay Geri Çağrıları

Kullanıcı etkileşimlerini işlemek için modifier tarzı geri çağrıları kullanın:

```swift
FastCommentsView(sdk: sdk)
    .onCommentPosted { comment in
        print("New comment: \(comment.commentHTML)")
    }
    .onReplyClick { renderableComment in
        print("Replying to: \(renderableComment.comment.id)")
    }
    .onUserClick { context, userInfo, source in
        // source .name veya .avatar
        print("Tapped \(userInfo.displayName)")
    }
```

### Tema Uygulama

SwiftUI ortamı aracılığıyla bir tema iletin:

```swift
FastCommentsView(sdk: sdk)
    .fastCommentsTheme(myTheme)
    .task { try? await sdk.load() }
```

Veya doğrudan SDK üzerinde ayarlayın:

```swift
sdk.theme = FastCommentsTheme.modern
```

### Sıralama Yönü

```swift
sdk.defaultSortDirection = .nf  // En yeni önce (varsayılan)
sdk.defaultSortDirection = .of  // En eski önce
sdk.defaultSortDirection = .mr  // En alakalı
```