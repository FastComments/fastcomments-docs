### Радње доступне свим корисницима

- **Flag/Unflag** -- пријавите коментар за преглед

```swift
try await sdk.flagComment(commentId: commentId)
try await sdk.unflagComment(commentId: commentId)
```

- **Block/Unblock** -- сакријте све коментаре од корисника (по гледаоцу)

```swift
try await sdk.blockUser(commentId: commentId)
try await sdk.unblockUser(commentId: commentId)
```

### Радње само за администраторе

- **Pin/Unpin** -- закачи коментар на врх нити

```swift
try await sdk.pinComment(commentId: commentId)
try await sdk.unpinComment(commentId: commentId)
```

- **Lock/Unlock** -- спречите нове одговоре на коментар, и блокирајте измене и брисања док се не откључа (важи за све, укључујући модераторе)

```swift
try await sdk.lockComment(commentId: commentId)
try await sdk.unlockComment(commentId: commentId)
```

Све модераторске радње су такође доступне преко контекстног менија коментара у корисничком интерфејсу. Администраторске радње се појављују само када је тренутни корисник администратор сајта (подешено преко SSO `isAdmin` заставице или конфигурације контролне табле).