### Действия, доступные всем пользователям

- **Flag/Unflag** -- сообщить о комментарии для проверки

```swift
try await sdk.flagComment(commentId: commentId)
try await sdk.unflagComment(commentId: commentId)
```

- **Block/Unblock** -- скрыть все комментарии от пользователя (для конкретного просмотрщика)

```swift
try await sdk.blockUser(commentId: commentId)
try await sdk.unblockUser(commentId: commentId)
```

### Действия только для администраторов

- **Pin/Unpin** -- закрепить комментарий в верхней части ветки

```swift
try await sdk.pinComment(commentId: commentId)
try await sdk.unpinComment(commentId: commentId)
```

- **Lock/Unlock** -- предотвращать новые ответы к комментарию, а также блокировать правки и удаления, пока он не будет разблокирован (применяется ко всем, включая модераторов)

```swift
try await sdk.lockComment(commentId: commentId)
try await sdk.unlockComment(commentId: commentId)
```

Все действия модерации также доступны через контекстное меню комментария в пользовательском интерфейсе. Действия администраторов отображаются только тогда, когда текущий пользователь является администратором сайта (устанавливается через флаг SSO `isAdmin` или конфигурацию панели управления).