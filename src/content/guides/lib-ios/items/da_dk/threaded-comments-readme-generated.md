### Grundlæggende brug

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

### Stemmetyper

Standard stemmestil viser op-/ned-pile. Angiv `._1` for hjerte-stemmer:

```swift
FastCommentsView(sdk: sdk, voteStyle: ._1)
```

| Stil | Udseende |
|-------|------------|
| `._0` | Op-/ned-pileknapper med nettetal |
| `._1` | Enkel hjerteknap med antal |

### Hændelses-callbacks

Brug callbacks i modifier-stil til at håndtere brugerinteraktioner:

```swift
FastCommentsView(sdk: sdk)
    .onCommentPosted { comment in
        print("New comment: \(comment.commentHTML)")
    }
    .onReplyClick { renderableComment in
        print("Replying to: \(renderableComment.comment.id)")
    }
    .onUserClick { context, userInfo, source in
        // source er .name eller .avatar
        print("Tapped \(userInfo.displayName)")
    }
```

### Anvend et tema

Angiv et tema via SwiftUI-miljøet:

```swift
FastCommentsView(sdk: sdk)
    .fastCommentsTheme(myTheme)
    .task { try? await sdk.load() }
```

Eller indstil det direkte på SDK'en:

```swift
sdk.theme = FastCommentsTheme.modern
```

### Sorteringsretning

```swift
sdk.defaultSortDirection = .nf  // Nyeste først (standard)
sdk.defaultSortDirection = .of  // Ældste først
sdk.defaultSortDirection = .mr  // Mest relevante
```