### Uso Básico

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

### Estilos de Voto

O estilo de voto padrão mostra setas para cima/para baixo. Passe `._1` para votos em forma de coração:

```swift
FastCommentsView(sdk: sdk, voteStyle: ._1)
```

| Estilo | Aparência |
|--------|-----------|
| `._0`  | Botões de seta para cima/baixo com contagem líquida |
| `._1`  | Único botão de coração com contagem |

### Callbacks de Evento

Use callbacks no estilo de modificador para lidar com interações do usuário:

```swift
FastCommentsView(sdk: sdk)
    .onCommentPosted { comment in
        print("New comment: \(comment.commentHTML)")
    }
    .onReplyClick { renderableComment in
        print("Replying to: \(renderableComment.comment.id)")
    }
    .onUserClick { context, userInfo, source in
        // source é .name ou .avatar
        print("Tapped \(userInfo.displayName)")
    }
```

### Aplicando um Tema

Passe um tema através do ambiente SwiftUI:

```swift
FastCommentsView(sdk: sdk)
    .fastCommentsTheme(myTheme)
    .task { try? await sdk.load() }
```

Ou defina-o diretamente no SDK:

```swift
sdk.theme = FastCommentsTheme.modern
```

### Direção de Ordenação

```swift
sdk.defaultSortDirection = .nf  // Mais recentes primeiro (padrão)
sdk.defaultSortDirection = .of  // Mais antigos primeiro
sdk.defaultSortDirection = .mr  // Mais relevantes
```