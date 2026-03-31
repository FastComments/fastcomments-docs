O sistema de feed é um SDK separado (`FastCommentsFeedSDK`) com sua própria view.

### Carregando e Exibindo o Feed

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
                try? await sdk.load()
            }
    }
}
```

A visualização do feed inclui pull-to-refresh e rolagem infinita automaticamente.

### Criando Posts

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

### Reagindo a Posts

O SDK lida com reações com atualizações otimistas:

```swift
try await sdk.reactPost(postId: post.id, reactionType: "l")

// Check reaction state
let hasLiked = sdk.hasUserReacted(postId: post.id, reactType: "l")
let likeCount = sdk.getLikeCount(postId: post.id)
```

### Abrindo Comentários em um Post

Use `CommentsSheet` para exibir comentários de um post do feed. Ele cria internamente uma instância de `FastCommentsSDK` usando a config do feed SDK:

```swift
.sheet(item: $commentsPost) { post in
    CommentsSheet(post: post, feedSDK: sdk, onUserClick: { context, userInfo, source in
        // Lidar com clique do usuário
    })
}
```

Nota: `FeedPost` deve conformar ao `Identifiable` para `.sheet(item:)`. Adicione esta extensão:

```swift
extension FeedPost: @retroactive Identifiable {}
```

### Filtragem do Feed por Tags

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

### Salvando e Restaurando o Estado do Feed

Preserve o estado de paginação durante eventos do ciclo de vida da view:

```swift
let state = sdk.savePaginationState()
// Later...
sdk.restorePaginationState(state)
```

### Excluindo Posts

```swift
sdk.onPostDeleted = { postId in
    print("Post \(postId) was deleted")
}
```

---
---