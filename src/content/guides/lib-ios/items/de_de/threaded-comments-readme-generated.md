### Grundlegende Verwendung

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

### Abstimmungsstile

Der standardmäßige Abstimmungsstil zeigt Auf-/Ab-Pfeile. Verwende `._1` für Herz-Style-Stimmen:

```swift
FastCommentsView(sdk: sdk, voteStyle: ._1)
```

| Stil | Erscheinungsbild |
|-------|------------|
| `._0` | Auf-/Ab-Pfeile mit Nettoanzahl |
| `._1` | Ein einzelner Herz-Button mit Zähler |

### Ereignis-Callbacks

Verwende Callback-Modifier, um Benutzerinteraktionen zu verarbeiten:

```swift
FastCommentsView(sdk: sdk)
    .onCommentPosted { comment in
        print("New comment: \(comment.commentHTML)")
    }
    .onReplyClick { renderableComment in
        print("Replying to: \(renderableComment.comment.id)")
    }
    .onUserClick { context, userInfo, source in
        // Quelle ist .name oder .avatar
        print("Tapped \(userInfo.displayName)")
    }
```

### Theme anwenden

Übergebe ein Theme über die SwiftUI-Umgebung:

```swift
FastCommentsView(sdk: sdk)
    .fastCommentsTheme(myTheme)
    .task { try? await sdk.load() }
```

Oder setze es direkt im SDK:

```swift
sdk.theme = FastCommentsTheme.modern
```

### Sortierreihenfolge

```swift
sdk.defaultSortDirection = .nf  // Neueste zuerst (Standard)
sdk.defaultSortDirection = .of  // Älteste zuerst
sdk.defaultSortDirection = .mr  // Am relevantesten
```

---
---