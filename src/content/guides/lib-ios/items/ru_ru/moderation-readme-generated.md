---
### Действия, доступные всем пользователям

- **Flag/Unflag** -- сообщить о комментарии для проверки

```swift
try await sdk.flagComment(commentId: commentId)
try await sdk.unflagComment(commentId: commentId)
```

- **Block/Unblock** -- скрыть все комментарии пользователя (для конкретного просматривающего)

```swift
try await sdk.blockUser(commentId: commentId)
try await sdk.unblockUser(commentId: commentId)
```

### Действия, доступные только администраторам

- **Pin/Unpin** -- закрепить/открепить комментарий вверху ветки обсуждения

```swift
try await sdk.pinComment(commentId: commentId)
try await sdk.unpinComment(commentId: commentId)
```

- **Lock/Unlock** -- запретить/разрешить новые ответы на комментарий

```swift
try await sdk.lockComment(commentId: commentId)
try await sdk.unlockComment(commentId: commentId)
```

Все действия модерации также доступны через контекстное меню комментария в интерфейсе. Действия администратора отображаются только когда текущий пользователь является администратором сайта (устанавливается через SSO `isAdmin` флаг или конфигурацию в панели управления).

---
---