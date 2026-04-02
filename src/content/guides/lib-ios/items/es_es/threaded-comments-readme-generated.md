### Uso básico

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

### Estilos de voto

El estilo de voto por defecto muestra flechas arriba/abajo. Usa `._1` para votos estilo corazón:

```swift
FastCommentsView(sdk: sdk, voteStyle: ._1)
```

| Estilo | Apariencia |
|-------|------------|
| `._0` | Botones de flecha arriba/abajo con recuento neto |
| `._1` | Botón único de corazón con contador |

### Callbacks de eventos

Usa callbacks estilo modificador para manejar las interacciones del usuario:

```swift
FastCommentsView(sdk: sdk)
    .onCommentPosted { comment in
        print("New comment: \(comment.commentHTML)")
    }
    .onReplyClick { renderableComment in
        print("Replying to: \(renderableComment.comment.id)")
    }
    .onUserClick { context, userInfo, source in
        // source es .name o .avatar
        print("Tapped \(userInfo.displayName)")
    }
```

### Aplicar un tema

Pasa un tema a través del entorno de SwiftUI:

```swift
FastCommentsView(sdk: sdk)
    .fastCommentsTheme(myTheme)
    .task { try? await sdk.load() }
```

O configúralo directamente en el SDK:

```swift
sdk.theme = FastCommentsTheme.modern
```

### Dirección de orden

```swift
sdk.defaultSortDirection = .nf  // Más recientes primero (predeterminado)
sdk.defaultSortDirection = .of  // Más antiguos primero
sdk.defaultSortDirection = .mr  // Más relevantes
```

---
---