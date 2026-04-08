---
Il sistema dei feed è un SDK separato (`FastCommentsFeedSDK`) con la propria vista.

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
                // Naviga al profilo utente
            }
            .onMediaClick { mediaItem, index in
                // Presenta il visualizzatore di immagini a schermo intero
            }
            .task {
                try? await sdk.loadIfNeeded()
            }
    }
}
```

La vista del feed include automaticamente il pull-to-refresh e lo scrolling infinito.
Usa `loadIfNeeded()` al rientro nel ciclo di vita della schermata in modo che un feed esistente o ripristinato non venga riportato alla pagina 1.

### Creazione dei post

Usa `FeedPostCreateView` per presentare un modulo di creazione del post:

```swift
@State private var showCreatePost = false

// In your view body:
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

// Check reaction state
let hasLiked = sdk.hasUserReacted(postId: post.id, reactType: "l")
let likeCount = sdk.getLikeCount(postId: post.id)
```

### Aprire i commenti di un post

Usa `CommentsSheet` per mostrare i commenti di un post del feed. Crea internamente un'istanza di `FastCommentsSDK` usando la configurazione dello SDK del feed:

```swift
.sheet(item: $commentsPost) { post in
    CommentsSheet(post: post, feedSDK: sdk, onUserClick: { context, userInfo, source in
        // Gestire il clic sull'utente
    })
}
```

Nota: `FeedPost` deve conformarsi a `Identifiable` per `.sheet(item:)`. Aggiungi questa estensione:

```swift
extension FeedPost: @retroactive Identifiable {}
```

### Filtraggio del feed basato sui tag

Implementa il protocollo `TagSupplier` per filtrare i post del feed per tag:

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

Preserva lo stato di paginazione durante gli eventi del ciclo di vita della vista:

```swift
let state = sdk.savePaginationState()
// Later...
sdk.restorePaginationState(state)
try? await sdk.loadIfNeeded()
```

Se la tua schermata scompare temporaneamente, la vista del feed mette in pausa gli aggiornamenti in tempo reale automaticamente e li riprende al riapparire senza cancellare i post già caricati. Chiama `sdk.cleanup()` solo quando hai davvero finito con l'istanza dell'SDK.

### Eliminazione dei post

```swift
sdk.onPostDeleted = { postId in
    print("Post \(postId) was deleted")
}
```

---