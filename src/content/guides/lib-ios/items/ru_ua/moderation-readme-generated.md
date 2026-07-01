### Доступные действия для всех пользователей

- **Flag/Unflag** -- сообщить о комментарии для проверки

```swift
try await sdk.flagComment(commentId: commentId)
try await sdk.unflagComment(commentId: commentId)
```

- **Block/Unblock** -- скрыть все комментарии от пользователя (для отдельного просмотрщика)

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

- **Lock/Unlock** -- предотвратить новые ответы к комментарию и блокировать редактирование и удаление, пока он не будет разблокирован (действует на всех, включая модераторов)

```swift
try await sdk.lockComment(commentId: commentId)
try await sdk.unlockComment(commentId: commentId)
```

Все действия модерации также доступны через контекстное меню комментария в пользовательском интерфейсе. Действия администратора отображаются только когда текущий пользователь является администратором сайта (установлено через флаг SSO `isAdmin` или конфигурацию в панели управления).