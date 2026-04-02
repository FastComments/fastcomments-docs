### Ações disponíveis para todos os usuários

- **Denunciar/Remover denúncia** -- denunciar um comentário para revisão

```swift
try await sdk.flagComment(commentId: commentId)
try await sdk.unflagComment(commentId: commentId)
```

- **Bloquear/Desbloquear** -- ocultar todos os comentários de um usuário (por visualizador)

```swift
try await sdk.blockUser(commentId: commentId)
try await sdk.unblockUser(commentId: commentId)
```

### Ações apenas para administradores

- **Fixar/Desfixar** -- fixar um comentário no topo da thread

```swift
try await sdk.pinComment(commentId: commentId)
try await sdk.unpinComment(commentId: commentId)
```

- **Bloquear/Desbloquear** -- impedir novas respostas a um comentário

```swift
try await sdk.lockComment(commentId: commentId)
try await sdk.unlockComment(commentId: commentId)
```

Todas as ações de moderação também estão disponíveis através do menu de contexto do comentário na interface. As ações de administrador aparecem apenas quando o usuário atual é um administrador do site (definido via SSO `isAdmin` flag ou configuração do painel).

---
---