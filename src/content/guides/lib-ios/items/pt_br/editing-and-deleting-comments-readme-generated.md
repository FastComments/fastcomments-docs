### Editar

```swift
try await sdk.editComment(commentId: commentId, newText: "Updated text")
```

O servidor renderiza novamente o HTML. O comentário local é atualizado automaticamente.

### Excluir

```swift
try await sdk.deleteComment(commentId: commentId)
```

Ao excluir um comentário, seus descendentes também são removidos da árvore local.

Ambas as ações estão disponíveis através do menu de contexto do comentário na interface quando o usuário atual for o autor do comentário (ou um administrador do site).

---
---