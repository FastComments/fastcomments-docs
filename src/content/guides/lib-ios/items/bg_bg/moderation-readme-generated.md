### Действия, достъпни за всички потребители

- **Поставяне/Премахване на флаг** -- докладване на коментар за преглед

```swift
try await sdk.flagComment(commentId: commentId)
try await sdk.unflagComment(commentId: commentId)
```

- **Блокиране/Разблокиране** -- скриване на всички коментари от потребител (за текущия зрител)

```swift
try await sdk.blockUser(commentId: commentId)
try await sdk.unblockUser(commentId: commentId)
```

### Действия само за администратори

- **Прикрепване/Откачване** -- прикрепяне на коментар в горната част на нишката

```swift
try await sdk.pinComment(commentId: commentId)
try await sdk.unpinComment(commentId: commentId)
```

- **Заключване/Отключване** -- предотвратяване на нови отговори на коментар

```swift
try await sdk.lockComment(commentId: commentId)
try await sdk.unlockComment(commentId: commentId)
```

Всички модерационни действия са налични и чрез контекстното меню на коментара в потребителския интерфейс. Администраторските действия се показват само когато текущият потребител е администратор на сайта (зададено чрез SSO флага `isAdmin` или конфигурация в таблото за управление).

---
---