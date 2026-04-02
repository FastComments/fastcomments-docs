### Действия, доступные всем пользователям

- **Пожаловаться/Отменить жалобу** -- сообщить о комментарии для проверки

```swift
try await sdk.flagComment(commentId: commentId)
try await sdk.unflagComment(commentId: commentId)
```

- **Заблокировать/Разблокировать** -- скрыть все комментарии от пользователя (локально, для каждого просматривающего)

```swift
try await sdk.blockUser(commentId: commentId)
try await sdk.unblockUser(commentId: commentId)
```

### Действия только для администраторов

- **Закрепить/Открепить** -- поместить комментарий в начало ветки

```swift
try await sdk.pinComment(commentId: commentId)
try await sdk.unpinComment(commentId: commentId)
```

- **Закрыть/Открыть** -- запретить новые ответы на комментарий

```swift
try await sdk.lockComment(commentId: commentId)
try await sdk.unlockComment(commentId: commentId)
```

Все модерационные действия также доступны через контекстное меню комментария в интерфейсе. Действия администратора отображаются только тогда, когда текущий пользователь является администратором сайта (установлено через SSO флаг `isAdmin` или в настройках панели управления).

---
---