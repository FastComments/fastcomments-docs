### Akcje dostępne dla wszystkich użytkowników

- **Flag/Unflag** -- zgłoś komentarz do przeglądu

```swift
try await sdk.flagComment(commentId: commentId)
try await sdk.unflagComment(commentId: commentId)
```

- **Block/Unblock** -- ukryj wszystkie komentarze od użytkownika (dla każdego widza)

```swift
try await sdk.blockUser(commentId: commentId)
try await sdk.unblockUser(commentId: commentId)
```

### Akcje dostępne tylko dla administratorów

- **Pin/Unpin** -- przypnij komentarz na górę wątku

```swift
try await sdk.pinComment(commentId: commentId)
try await sdk.unpinComment(commentId: commentId)
```

- **Lock/Unlock** -- uniemożliwiaj nowe odpowiedzi na komentarz oraz blokuj edycje i usuwanie, dopóki nie zostanie odblokowany (dotyczy wszystkich, w tym moderatorów)

```swift
try await sdk.lockComment(commentId: commentId)
try await sdk.unlockComment(commentId: commentId)
```

Wszystkie akcje moderacyjne są również dostępne w menu kontekstowym komentarza w interfejsie użytkownika. Akcje administratora pojawiają się tylko wtedy, gdy bieżący użytkownik jest administratorem witryny (ustawionym za pomocą flagi SSO `isAdmin` lub konfiguracji w panelu).