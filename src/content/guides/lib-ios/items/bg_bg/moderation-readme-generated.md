### Действия, достъпни за всички потребители

- **Flag/Unflag** -- докладване на коментар за преглед

```swift
try await sdk.flagComment(commentId: commentId)
try await sdk.unflagComment(commentId: commentId)
```

- **Block/Unblock** -- скриване на всички коментари от потребител (за всеки потребител по отделно)

```swift
try await sdk.blockUser(commentId: commentId)
try await sdk.unblockUser(commentId: commentId)
```

### Действия, достъпни само за администратори

- **Pin/Unpin** -- закачане на коментар в началото на нишката

```swift
try await sdk.pinComment(commentId: commentId)
try await sdk.unpinComment(commentId: commentId)
```

- **Lock/Unlock** -- предотвратяване на нови отговори към коментар и блокиране на редакции и изтривания, докато е отключен (важи за всички, включително модератори)

```swift
try await sdk.lockComment(commentId: commentId)
try await sdk.unlockComment(commentId: commentId)
```

Всички действия за модериране също са достъпни чрез контекстното меню на коментара в потребителския интерфейс. Действията за администратори се появяват само когато текущият потребител е администратор на сайта (заден чрез SSO `isAdmin` флаг или конфигурация в таблото за управление).