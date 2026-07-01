### Ações Disponíveis para Todos os Usuários

- **Flag/Unflag** -- denunciar um comentário para revisão

```swift
try await sdk.flagComment(commentId: commentId)
try await sdk.unflagComment(commentId: commentId)
```

- **Block/Unblock** -- ocultar todos os comentários de um usuário (por visualizador)

```swift
try await sdk.blockUser(commentId: commentId)
try await sdk.unblockUser(commentId: commentId)
```

### Ações Apenas para Administradores

- **Pin/Unpin** -- fixar um comentário no topo da conversa

```swift
try await sdk.pinComment(commentId: commentId)
try await sdk.unpinComment(commentId: commentId)
```

- **Lock/Unlock** -- impedir novas respostas a um comentário e bloquear edições e exclusões até que seja desbloqueado (aplica-se a todos, incluindo moderadores)

```swift
try await sdk.lockComment(commentId: commentId)
try await sdk.unlockComment(commentId: commentId)
```

Todas as ações de moderação também estão disponíveis através do menu de contexto do comentário na interface. As ações de administrador só aparecem quando o usuário atual é um administrador do site (definido via flag SSO `isAdmin` ou configuração do painel).