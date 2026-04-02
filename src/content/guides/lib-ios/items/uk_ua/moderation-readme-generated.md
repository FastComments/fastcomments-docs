### Дії, доступні всім користувачам

- **Поскаржитися/Зняти скаргу** -- повідомити про коментар для перевірки

```swift
try await sdk.flagComment(commentId: commentId)
try await sdk.unflagComment(commentId: commentId)
```

- **Заблокувати/Розблокувати** -- приховати всі коментарі від користувача (для поточного переглядача)

```swift
try await sdk.blockUser(commentId: commentId)
try await sdk.unblockUser(commentId: commentId)
```

### Дії лише для адміністраторів

- **Закріпити/Відкріпити** -- закріпити коментар у верхній частині теми

```swift
try await sdk.pinComment(commentId: commentId)
try await sdk.unpinComment(commentId: commentId)
```

- **Заблокувати/Розблокувати** -- заборонити нові відповіді на коментар

```swift
try await sdk.lockComment(commentId: commentId)
try await sdk.unlockComment(commentId: commentId)
```

Усі дії модерації також доступні через контекстне меню коментаря в інтерфейсі. Дії адміністратора з'являються лише коли поточний користувач є адміністратором сайту (встановлюється через SSO `isAdmin` flag or dashboard configuration).