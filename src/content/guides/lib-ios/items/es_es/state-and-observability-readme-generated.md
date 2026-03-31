Ambas `FastCommentsSDK` y `FastCommentsFeedSDK` son clases `ObservableObject` con propiedades `@Published`. Puedes observarlas en tus vistas SwiftUI para actualizaciones reactivas de la interfaz.

### Propiedades Publicadas de FastCommentsSDK

| Propiedad | Tipo | Descripción |
|----------|------|-------------|
| `commentCountOnServer` | `Int` | Recuento total de comentarios en el servidor |
| `newRootCommentCount` | `Int` | Comentarios nuevos en búfer (cuando `showLiveRightAway` es false) |
| `currentUser` | `UserSessionInfo?` | Usuario autenticado actual |
| `isSiteAdmin` | `Bool` | Si el usuario actual es administrador del sitio |
| `isClosed` | `Bool` | Si el hilo de comentarios está cerrado |
| `hasBillingIssue` | `Bool` | Si existe un problema de facturación |
| `isLoading` | `Bool` | Si hay una petición de red en curso |
| `hasMore` | `Bool` | Si existen más páginas de comentarios |
| `blockingErrorMessage` | `String?` | Error que impide que la interfaz funcione |
| `warningMessage` | `String?` | Mensaje de advertencia no bloqueante |
| `isDemo` | `Bool` | Si se está ejecutando en modo demo |
| `commentsVisible` | `Bool` | Control para la visibilidad de los comentarios |
| `toolbarEnabled` | `Bool` | Si se muestra la barra de herramientas de formato |

### Propiedades Publicadas de FastCommentsFeedSDK

| Propiedad | Tipo | Descripción |
|----------|------|-------------|
| `feedPosts` | `[FeedPost]` | Publicaciones del feed actualmente cargadas |
| `hasMore` | `Bool` | Si existen más páginas |
| `currentUser` | `UserSessionInfo?` | Usuario autenticado actual |
| `blockingErrorMessage` | `String?` | Mensaje de error bloqueante |
| `isLoading` | `Bool` | Si hay una petición de red en curso |
| `newPostsCount` | `Int` | Número de nuevas publicaciones desde la última carga |

### Árbol de comentarios

El árbol de comentarios es accesible a través de `sdk.commentsTree`:

```swift
// Lista plana de nodos visibles para renderizar
sdk.commentsTree.visibleNodes

// Buscar un comentario por ID
sdk.commentsTree.commentsById["comment-id"]
```

---
---