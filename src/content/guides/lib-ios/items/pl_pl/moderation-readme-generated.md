### Akcje dostępne dla wszystkich użytkowników

- **Zgłoś/Usuń zgłoszenie** -- zgłoś komentarz do przeglądu

```swift
try await sdk.flagComment(commentId: commentId)
try await sdk.unflagComment(commentId: commentId)
```

- **Zablokuj użytkownika/Odblokuj użytkownika** -- ukryj wszystkie komentarze od użytkownika (dla danego widza)

```swift
try await sdk.blockUser(commentId: commentId)
try await sdk.unblockUser(commentId: commentId)
```

### Akcje tylko dla administratorów

- **Przypnij/Odprzypnij** -- przypnij komentarz na górze wątku

```swift
try await sdk.pinComment(commentId: commentId)
try await sdk.unpinComment(commentId: commentId)
```

- **Zablokuj/Odblokuj** -- uniemożliwić dodawanie nowych odpowiedzi do komentarza

```swift
try await sdk.lockComment(commentId: commentId)
try await sdk.unlockComment(commentId: commentId)
```

Wszystkie akcje moderacyjne są również dostępne poprzez menu kontekstowe komentarza w interfejsie użytkownika. Akcje administracyjne pojawiają się tylko, gdy bieżący użytkownik jest administratorem serwisu (ustawione poprzez flagę SSO `isAdmin` lub konfigurację w panelu administracyjnym).

---
---