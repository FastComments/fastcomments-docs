El sistema de feed es un SDK separado (`FastCommentsFeedSDK`) con su propia vista.

### Carga y visualización del feed

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
                // Presentar hoja de compartir
            }
            .onUserClick { context, userInfo, source in
                // Navegar al perfil de usuario
            }
            .onMediaClick { mediaItem, index in
                // Presentar visor de imágenes a pantalla completa
            }
            .task {
                try? await sdk.load()
            }
    }
}
```

La vista de feed incluye arrastrar para actualizar (pull-to-refresh) y desplazamiento infinito automáticamente.

### Crear publicaciones

Usa `FeedPostCreateView` para presentar un formulario de creación de publicaciones:

```swift
@State private var showCreatePost = false

// En el cuerpo de tu vista:
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

### Reaccionar a publicaciones

El SDK gestiona las reacciones con actualizaciones optimistas:

```swift
try await sdk.reactPost(postId: post.id, reactionType: "l")

// Comprobar estado de reacción
let hasLiked = sdk.hasUserReacted(postId: post.id, reactType: "l")
let likeCount = sdk.getLikeCount(postId: post.id)
```

### Abrir comentarios de una publicación

Usa `CommentsSheet` para mostrar los comentarios de una publicación del feed. Crea una instancia de `FastCommentsSDK` internamente usando la configuración del feed SDK:

```swift
.sheet(item: $commentsPost) { post in
    CommentsSheet(post: post, feedSDK: sdk, onUserClick: { context, userInfo, source in
        // Manejar clic en el usuario
    })
}
```

Nota: `FeedPost` debe ajustarse a `Identifiable` para `.sheet(item:)`. Añade esta extensión:

```swift
extension FeedPost: @retroactive Identifiable {}
```

### Filtrado del feed por etiquetas

Implementa el protocolo `TagSupplier` para filtrar las publicaciones del feed por etiquetas:

```swift
struct TeamTagSupplier: TagSupplier {
    func getTags(currentUser: UserSessionInfo?) -> [String]? {
        guard let user = currentUser else { return nil }
        return ["team:\(user.id ?? "")", "public"]
    }
}

sdk.tagSupplier = TeamTagSupplier()
```

Devuelve `nil` para un feed global sin filtrar.

### Guardar y restaurar el estado del feed

Preserva el estado de paginación a través de eventos del ciclo de vida de la vista:

```swift
let state = sdk.savePaginationState()
// Más tarde...
sdk.restorePaginationState(state)
```

### Eliminar publicaciones

```swift
sdk.onPostDeleted = { postId in
    print("Post \(postId) was deleted")
}
```

---
---