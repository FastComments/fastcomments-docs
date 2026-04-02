### Uso di base

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

### Stili di voto

Lo stile di voto predefinito mostra frecce su/giù. Passa `._1` per voti a forma di cuore:

```swift
FastCommentsView(sdk: sdk, voteStyle: ._1)
```

| Stile | Aspetto |
|-------|---------|
| `._0` | Pulsanti freccia su/giu con conteggio netto |
| `._1` | Singolo pulsante a forma di cuore con conteggio |

### Callback per eventi

Usa callback in stile modifier per gestire le interazioni dell'utente:

```swift
FastCommentsView(sdk: sdk)
    .onCommentPosted { comment in
        print("New comment: \(comment.commentHTML)")
    }
    .onReplyClick { renderableComment in
        print("Replying to: \(renderableComment.comment.id)")
    }
    .onUserClick { context, userInfo, source in
        // source è .name o .avatar
        print("Tapped \(userInfo.displayName)")
    }
```

### Applicare un tema

Passa un tema tramite l'ambiente SwiftUI:

```swift
FastCommentsView(sdk: sdk)
    .fastCommentsTheme(myTheme)
    .task { try? await sdk.load() }
```

Oppure impostalo direttamente sull'SDK:

```swift
sdk.theme = FastCommentsTheme.modern
```

### Direzione di ordinamento

```swift
sdk.defaultSortDirection = .nf  // I più recenti prima (predefinito)
sdk.defaultSortDirection = .of  // I più vecchi prima
sdk.defaultSortDirection = .mr  // Più rilevanti
```