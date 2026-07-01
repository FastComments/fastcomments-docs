### Дії, доступні всім користувачам

- **Flag/Unflag** -- повідомити про коментар для перегляду

```swift
try await sdk.flagComment(commentId: commentId)
try await sdk.unflagComment(commentId: commentId)
```

- **Block/Unblock** -- приховати всі коментарі від користувача (для кожного переглядача)

```swift
try await sdk.blockUser(commentId: commentId)
try await sdk.unblockUser(commentId: commentId)
```

### Дії лише для адміністраторів

- **Pin/Unpin** -- закріпити коментар у верхній частині теми

```swift
try await sdk.pinComment(commentId: commentId)
try await sdk.unpinComment(commentId: commentId)
```

- **Lock/Unlock** -- запобігати новим відповідям на коментар, та блокувати редагування та видалення, доки коментар не буде розблоковано (стосується всіх, включаючи модераторів)

```swift
try await sdk.lockComment(commentId: commentId)
try await sdk.unlockComment(commentId: commentId)
```

Всі дії модерації також доступні через контекстне меню коментаря в інтерфейсі користувача. Дії адміністратора з’являються лише тоді, коли поточний користувач є адміністратором сайту (встановлено через прапорець SSO `isAdmin` або налаштування панелі).