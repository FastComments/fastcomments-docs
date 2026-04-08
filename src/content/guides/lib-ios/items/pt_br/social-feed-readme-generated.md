O sistema de feed é um SDK separado (`FastCommentsFeedSDK`) com sua própria visualização.

### Loading and Displaying the Feed

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
                // Apresentar folha de compartilhamento
            }
            .onUserClick { context, userInfo, source in
                // Navegar para o perfil do usuário
            }
            .onMediaClick { mediaItem, index in
                // Apresentar visualizador de imagem em tela cheia
            }
            .task {
                try? await sdk.loadIfNeeded()
            }
    }
}
```

A visualização do feed inclui pull-to-refresh e infinite scroll automaticamente.
Use `loadIfNeeded()` para reentrada no ciclo de vida da tela para que um feed existente ou restaurado não seja resetado para a página 1.

### Creating Posts

Use `FeedPostCreateView` para apresentar um formulário de criação de post:

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

### Reacting to Posts

O SDK lida com reações usando atualizações otimistas:

```swift
try await sdk.reactPost(postId: post.id, reactionType: "l")

// Check reaction state
let hasLiked = sdk.hasUserReacted(postId: post.id, reactType: "l")
let likeCount = sdk.getLikeCount(postId: post.id)
```

### Opening Comments on a Post

Use `CommentsSheet` para exibir comentários de um post do feed. Ele cria uma instância de `FastCommentsSDK` internamente usando a config do feed SDK:

```swift
.sheet(item: $commentsPost) { post in
    CommentsSheet(post: post, feedSDK: sdk, onUserClick: { context, userInfo, source in
        // Handle user click
    })
}
```

Note: `FeedPost` must conform to `Identifiable` for `.sheet(item:)`. Add this extension:

```swift
extension FeedPost: @retroactive Identifiable {}
```

### Tag-Based Feed Filtering

Implemente o protocolo `TagSupplier` para filtrar posts do feed por tags:

```swift
struct TeamTagSupplier: TagSupplier {
    func getTags(currentUser: UserSessionInfo?) -> [String]? {
        guard let user = currentUser else { return nil }
        return ["team:\(user.id ?? "")", "public"]
    }
}

sdk.tagSupplier = TeamTagSupplier()
```

Retorne `nil` para um feed global sem filtro.

### Saving and Restoring Feed State

Preserve o estado de paginação através dos eventos de ciclo de vida da view:

```swift
let state = sdk.savePaginationState()
// Later...
sdk.restorePaginationState(state)
try? await sdk.loadIfNeeded()
```

Se sua tela desaparecer temporariamente, a visualização do feed pausa atualizações ao vivo automaticamente e as retoma ao reaparecer sem limpar os posts carregados. Chame `sdk.cleanup()` somente quando você realmente terminar com a instância do SDK.

### Deleting Posts

```swift
sdk.onPostDeleted = { postId in
    print("Post \(postId) was deleted")
}
```