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
                // Present share sheet
            }
            .onUserClick { context, userInfo, source in
                // Navigate to user profile
            }
            .onMediaClick { mediaItem, index in
                // Present full-screen image viewer
            }
            .task {
                try? await sdk.loadIfNeeded()
            }
    }
}
```

La vista del feed incluye pull-to-refresh e infinite scroll automáticamente.
Usa `loadIfNeeded()` para la reentrada del ciclo de vida de la pantalla para que un feed existente o restaurado no se reinicie a la página 1.

### Creación de publicaciones

Usa `FeedPostCreateView` para presentar un formulario de creación de publicaciones:

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

### Reacciones a publicaciones

El SDK maneja las reacciones con actualizaciones optimistas:

```swift
try await sdk.reactPost(postId: post.id, reactionType: "l")

// Check reaction state
let hasLiked = sdk.hasUserReacted(postId: post.id, reactType: "l")
let likeCount = sdk.getLikeCount(postId: post.id)
```

### Abrir comentarios en una publicación

Usa `CommentsSheet` para mostrar los comentarios de una publicación del feed. Crea internamente una instancia de `FastCommentsSDK` usando la configuración del feed SDK:

```swift
.sheet(item: $commentsPost) { post in
    CommentsSheet(post: post, feedSDK: sdk, onUserClick: { context, userInfo, source in
        // Handle user click
    })
}
```

Nota: `FeedPost` debe conformar a `Identifiable` para `.sheet(item:)`. Añade esta extensión:

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

Devuelve `nil` para un feed global sin filtros.

### Guardar y restaurar el estado del feed

Preserva el estado de la paginación a través de los eventos del ciclo de vida de la vista:

```swift
let state = sdk.savePaginationState()
// Later...
sdk.restorePaginationState(state)
try? await sdk.loadIfNeeded()
```

Si tu pantalla desaparece temporalmente, la vista del feed pausa las actualizaciones en vivo automáticamente y las reanuda al reaparecer sin borrar las publicaciones cargadas. Llama a `sdk.cleanup()` solo cuando realmente hayas terminado con la instancia del SDK.

### Eliminación de publicaciones

```swift
sdk.onPostDeleted = { postId in
    print("Post \(postId) was deleted")
}
```

---
---