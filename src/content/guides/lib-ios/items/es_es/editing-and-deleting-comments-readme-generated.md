### Editar

```swift
try await sdk.editComment(commentId: commentId, newText: "Updated text")
```

El servidor vuelve a renderizar el HTML. El comentario local se actualiza automáticamente.

### Eliminar

```swift
try await sdk.deleteComment(commentId: commentId)
```

Eliminar un comentario también elimina sus descendientes del árbol local.

Ambas acciones están disponibles a través del menú contextual del comentario en la interfaz de usuario cuando el usuario actual es el autor del comentario (o un administrador del sitio).

---
---