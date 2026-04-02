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

Podrazumevani stil glasanja prikazuje strelice gore/dole. Prosledite `._1` za glasanje u obliku srca:

```swift
FastCommentsView(sdk: sdk, voteStyle: ._1)
```

| Stil | Izgled |
|-------|------------|
| `._0` | Dugmad sa strelicama gore/dole sa net brojem |
| `._1` | Jedno dugme sa srcem i brojačem |

### Povratni pozivi događaja

Koristite callback funkcije u stilu modifikatora za rukovanje interakcijama korisnika:

```swift
FastCommentsView(sdk: sdk)
    .onCommentPosted { comment in
        print("New comment: \(comment.commentHTML)")
    }
    .onReplyClick { renderableComment in
        print("Replying to: \(renderableComment.comment.id)")
    }
    .onUserClick { context, userInfo, source in
        // izvor je .name ili .avatar
        print("Tapped \(userInfo.displayName)")
    }
```

### Primena teme

Prosledite temu kroz SwiftUI okruženje:

```swift
FastCommentsView(sdk: sdk)
    .fastCommentsTheme(myTheme)
    .task { try? await sdk.load() }
```

Ili je postavite direktno na SDK:

```swift
sdk.theme = FastCommentsTheme.modern
```

### Smer sortiranja

```swift
sdk.defaultSortDirection = .nf  // Najnovije prvo (podrazumevano)
sdk.defaultSortDirection = .of  // Najstarije prvo
sdk.defaultSortDirection = .mr  // Najrelevantnije
```