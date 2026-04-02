### Uredi

```swift
try await sdk.editComment(commentId: commentId, newText: "Updated text")
```

Poslužitelj ponovno renderira HTML. Lokalni komentar se automatski ažurira.

### Izbriši

```swift
try await sdk.deleteComment(commentId: commentId)
```

Brisanje komentara također uklanja njegove potomke iz lokalnog stabla.

Obje radnje dostupne su putem kontekstnog izbornika komentara u korisničkom sučelju (UI) kada je trenutni korisnik autor komentara (ili administrator stranice).