Il sistema del feed è un SDK separato (`FastCommentsFeedSDK`) con la propria view.

### Caricamento e visualizzazione del feed

```swift
struct FeedPage: View {
    @StateObject private var sdk: FastCommentsFeedSDK = {
        let config = FastCommentsWidgetConfig(
            tenantId: "YOUR_TENANT_ID",
            urlId: "my-feed",
            sso: ssoToken
        )
        return FastCommentsFeedSDK(config: config)
    }()

    @State private var commentsPost: FeedPost?

    var body: some View {
        FastCommentsFeedView(sdk: sdk)
            .onPostSelected { post in
                commentsPost = post
            }
            .onCommentsRequested { post in
                commentsPost = post
            }
            .onSharePost { post in
                // Mostra il foglio di condivisione
            }
            .onUserClick { context, userInfo, source in
                // Vai al profilo utente
            }
            .onMediaClick { mediaItem, index in
                // Mostra visualizzatore immagini a schermo intero
            }
            .task {
                try? await sdk.load()
            }
    }
}
```

La view del feed include automaticamente pull-to-refresh e infinite scroll.

### Creazione di post

Usa `FeedPostCreateView` per presentare un modulo di creazione post:

```swift
@State private var showCreatePost = false

// Nel corpo della tua view:
.sheet(isPresented: $showCreatePost) {
    FeedPostCreateView(
        sdk: sdk,
        onPostCreated: { post in
            showCreatePost = false
            Task { try? await sdk.refresh() }
        },
        onCancelled: {
            showCreatePost = false
        }
    )
}
```

### Reazioni ai post

L'SDK gestisce le reazioni con aggiornamenti ottimistici:

```swift
try await sdk.reactPost(postId: post.id, reactionType: "l")

// Verifica lo stato della reazione
let hasLiked = sdk.hasUserReacted(postId: post.id, reactType: "l")
let likeCount = sdk.getLikeCount(postId: post.id)
```

### Apertura dei commenti su un post

Usa `CommentsSheet` per visualizzare i commenti di un post del feed. Crea internamente un'istanza di `FastCommentsSDK` usando la config del feed SDK:

```swift
.sheet(item: $commentsPost) { post in
    CommentsSheet(post: post, feedSDK: sdk, onUserClick: { context, userInfo, source in
        // Gestisci il click dell'utente
    })
}
```

Nota: `FeedPost` deve conformarsi a `Identifiable` per `.sheet(item:)`. Aggiungi questa extension:

```swift
extension FeedPost: @retroactive Identifiable {}
```

### Filtraggio del feed basato sui tag

Implementa il protocollo `TagSupplier` per filtrare i post del feed in base ai tag:

```swift
struct TeamTagSupplier: TagSupplier {
    func getTags(currentUser: UserSessionInfo?) -> [String]? {
        guard let user = currentUser else { return nil }
        return ["team:\(user.id ?? "")", "public"]
    }
}

sdk.tagSupplier = TeamTagSupplier()
```

Ritorna `nil` per un feed globale non filtrato.

### Salvataggio e ripristino dello stato del feed

Conserva lo stato di paginazione tra gli eventi del ciclo di vita della view:

```swift
let state = sdk.savePaginationState()
// Successivamente...
sdk.restorePaginationState(state)
```

### Eliminazione dei post

```swift
sdk.onPostDeleted = { postId in
    print("Post \(postId) was deleted")
}
```