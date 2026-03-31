### Izmeni

```swift
try await sdk.editComment(commentId: commentId, newText: "Updated text")
```

Server ponovo renderuje HTML. Lokalni komentar se automatski ažurira.

### Obriši

```swift
try await sdk.deleteComment(commentId: commentId)
```

Brisanje komentara takođe uklanja njegove potomke iz lokalnog stabla.

Obe radnje su dostupne kroz kontekstni meni komentara u UI kada je trenutni korisnik autor komentara (ili administrator sajta).