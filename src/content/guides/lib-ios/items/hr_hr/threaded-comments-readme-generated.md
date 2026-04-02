### Osnovna upotreba

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

### Stilovi glasanja

Zadani stil glasanja prikazuje strelice gore/dolje. Proslijedite `._1` za stil glasanja u obliku srca:

```swift
FastCommentsView(sdk: sdk, voteStyle: ._1)
```

| Stil | Izgled |
|-------|------------|
| `._0` | Gumbi sa strelicama gore/dolje s neto brojem |
| `._1` | Pojedinačni gumb srca s brojačem |

### Povratni pozivi događaja

Koristite povratne pozive u obliku modifikatora za rukovanje korisničkim interakcijama:

```swift
FastCommentsView(sdk: sdk)
    .onCommentPosted { comment in
        print("New comment: \(comment.commentHTML)")
    }
    .onReplyClick { renderableComment in
        print("Replying to: \(renderableComment.comment.id)")
    }
    .onUserClick { context, userInfo, source in
        // source je .name ili .avatar
        print("Tapped \(userInfo.displayName)")
    }
```

### Primjena teme

Proslijedite temu putem SwiftUI okruženja:

```swift
FastCommentsView(sdk: sdk)
    .fastCommentsTheme(myTheme)
    .task { try? await sdk.load() }
```

Ili postavite izravno na SDK:

```swift
sdk.theme = FastCommentsTheme.modern
```

### Smjer sortiranja

```swift
sdk.defaultSortDirection = .nf  // Najnovije prvo (zadano)
sdk.defaultSortDirection = .of  // Najstarije prvo
sdk.defaultSortDirection = .mr  // Najrelevantnije
```