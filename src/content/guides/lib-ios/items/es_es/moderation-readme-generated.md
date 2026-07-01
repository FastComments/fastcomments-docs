### Acciones disponibles para todos los usuarios

- **Flag/Unflag** -- reportar un comentario para revisión

```swift
try await sdk.flagComment(commentId: commentId)
try await sdk.unflagComment(commentId: commentId)
```

- **Block/Unblock** -- ocultar todos los comentarios de un usuario (por visor)

```swift
try await sdk.blockUser(commentId: commentId)
try await sdk.unblockUser(commentId: commentId)
```

### Acciones solo para administradores

- **Pin/Unpin** -- fijar un comentario en la parte superior del hilo

```swift
try await sdk.pinComment(commentId: commentId)
try await sdk.unpinComment(commentId: commentId)
```

- **Lock/Unlock** -- impedir nuevas respuestas a un comentario y bloquear ediciones y eliminaciones hasta que se desbloquee (se aplica a todos, incluidos los moderadores)

```swift
try await sdk.lockComment(commentId: commentId)
try await sdk.unlockComment(commentId: commentId)
```

Todas las acciones de moderación también están disponibles a través del menú contextual del comentario en la UI. Las acciones de administrador solo aparecen cuando el usuario actual es un administrador del sitio (establecido mediante la bandera `isAdmin` de SSO o la configuración del panel).