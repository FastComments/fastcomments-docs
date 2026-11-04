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
                // Apresentar a folha de compartilhamento
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

A view de feed inclui pull-to-refresh e scroll infinito automaticamente.
Use `loadIfNeeded()` para reentrada no ciclo de vida da tela, para que um feed existente ou restaurado não seja redefinido de volta para a página 1.

### Criando Posts

Use `FeedPostCreateView` para apresentar um formulário de criação de post:

```swift
@State private var showCreatePost = false

// No corpo da sua view:
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

O SDK lida com as reações com atualizações otimistas:

```swift
try await sdk.reactPost(postId: post.id, reactionType: "l")

// Verificar o estado da reação
let hasLiked = sdk.hasUserReacted(postId: post.id, reactType: "l")
let likeCount = sdk.getLikeCount(postId: post.id)
```

### Abrindo Comentários em um Post

Use `CommentsSheet` para exibir os comentários de um post do feed. Ele cria uma instância de `FastCommentsSDK` internamente usando a configuração do SDK de feed:

```swift
.sheet(item: $commentsPost) { post in
    CommentsSheet(post: post, feedSDK: sdk, onUserClick: { context, userInfo, source in
        // Tratar o clique do usuário
    })
}
```

Observação: `FeedPost` deve estar em conformidade com `Identifiable` para `.sheet(item:)`. Adicione esta extensão:

```swift
extension FeedPost: @retroactive Identifiable {}
```

### Filtragem de Feed por Tags

Implemente o protocolo `TagSupplier` para filtrar os posts do feed por tags:

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

Preserve o estado de paginação entre eventos do ciclo de vida da view:

```swift
let state = sdk.savePaginationState()
// Mais tarde...
sdk.restorePaginationState(state)
try? await sdk.loadIfNeeded()
```

Se sua tela desaparecer temporariamente, a view de feed pausa as atualizações ao vivo automaticamente e as retoma ao reaparecer, sem limpar os posts carregados. Chame `sdk.cleanup()` somente quando realmente terminar de usar a instância do SDK.

### Excluindo Posts

```swift
sdk.onPostDeleted = { postId in
    print("Post \(postId) was deleted")
}
```

---
---