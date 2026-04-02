### Edytuj

```swift
try await sdk.editComment(commentId: commentId, newText: "Updated text")
```

Serwer ponownie renderuje HTML. Lokalny komentarz aktualizuje się automatycznie.

### Usuń

```swift
try await sdk.deleteComment(commentId: commentId)
```

Usunięcie komentarza usuwa również jego potomków z lokalnego drzewa.

Obie akcje są dostępne w menu kontekstowym komentarza w interfejsie użytkownika, gdy aktualny użytkownik jest autorem komentarza (lub administratorem strony).