### Acciones disponibles para todos los usuarios

- **Marcar/Desmarcar** -- reportar un comentario para revisión

```swift
try await sdk.flagComment(commentId: commentId)
try await sdk.unflagComment(commentId: commentId)
```

- **Bloquear/Desbloquear** -- ocultar todos los comentarios de un usuario (por espectador)

```swift
try await sdk.blockUser(commentId: commentId)
try await sdk.unblockUser(commentId: commentId)
```

### Acciones solo para administradores

- **Fijar/Desfijar** -- fijar un comentario en la parte superior del hilo

```swift
try await sdk.pinComment(commentId: commentId)
try await sdk.unpinComment(commentId: commentId)
```

- **Bloquear/Desbloquear** -- impedir nuevas respuestas en un comentario

```swift
try await sdk.lockComment(commentId: commentId)
try await sdk.unlockComment(commentId: commentId)
```

Todas las acciones de moderación también están disponibles a través del menú contextual del comentario en la interfaz de usuario. Las acciones de administrador solo aparecen cuando el usuario actual es administrador del sitio (configurado mediante la bandera SSO `isAdmin` o la configuración del panel).

---
---