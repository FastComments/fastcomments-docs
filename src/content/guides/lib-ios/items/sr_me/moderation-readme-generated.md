### Радње доступне свим корисницима

- **Пријави/Уклони пријаву** -- пријавити коментар на преглед

```swift
try await sdk.flagComment(commentId: commentId)
try await sdk.unflagComment(commentId: commentId)
```

- **Блокирај/Деблокирај** -- сакрити све коментаре од корисника (за сваког гледаоца појединачно)

```swift
try await sdk.blockUser(commentId: commentId)
try await sdk.unblockUser(commentId: commentId)
```

### Радње само за администраторе

- **Закачи/Откачи** -- закачити коментар на врх нити

```swift
try await sdk.pinComment(commentId: commentId)
try await sdk.unpinComment(commentId: commentId)
```

- **Закључај/Откључај** -- спречити нове одговоре на коментар

```swift
try await sdk.lockComment(commentId: commentId)
try await sdk.unlockComment(commentId: commentId)
```

Све модерацијске радње такође су доступне преко контекстног менија коментара у корисничком интерфејсу. Администраторске радње се појављују само када је тренутни корисник администратор сајта (постављено преко SSO `isAdmin` ознаке или конфигурације контролне табле).

---
---