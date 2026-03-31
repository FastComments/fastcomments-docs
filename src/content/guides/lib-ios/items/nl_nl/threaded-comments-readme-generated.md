### Basisgebruik

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

### Stemstijlen

De standaard stemstijl toont pijlen omhoog/omlaag. Geef `._1` op voor hart-stijl stemmen:

```swift
FastCommentsView(sdk: sdk, voteStyle: ._1)
```

| Stijl | Weergave |
|-------|----------|
| `._0` | Pijltjes omhoog/omlaag met nettotelling |
| `._1` | Enkele hartknop met telling |

### Evenement-callbacks

Gebruik modifier-stijl callbacks om gebruikersinteracties af te handelen:

```swift
FastCommentsView(sdk: sdk)
    .onCommentPosted { comment in
        print("New comment: \(comment.commentHTML)")
    }
    .onReplyClick { renderableComment in
        print("Replying to: \(renderableComment.comment.id)")
    }
    .onUserClick { context, userInfo, source in
        // bron is .name of .avatar
        print("Tapped \(userInfo.displayName)")
    }
```

### Een thema toepassen

Geef een thema door via de SwiftUI-omgeving:

```swift
FastCommentsView(sdk: sdk)
    .fastCommentsTheme(myTheme)
    .task { try? await sdk.load() }
```

Of stel het rechtstreeks in op de SDK:

```swift
sdk.theme = FastCommentsTheme.modern
```

### Sorteerrichting

```swift
sdk.defaultSortDirection = .nf  // Nieuwste eerst (standaard)
sdk.defaultSortDirection = .of  // Oudste eerst
sdk.defaultSortDirection = .mr  // Meest relevant
```