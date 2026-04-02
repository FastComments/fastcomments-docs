---
### Uredi

```swift
try await sdk.editComment(commentId: commentId, newText: "Updated text")
```

Strežnik ponovno upodobi HTML. Lokalni komentar se samodejno posodobi.

### Izbriši

```swift
try await sdk.deleteComment(commentId: commentId)
```

Brisanje komentarja prav tako odstrani njegove potomce iz lokalnega drevesa.

Obe dejanji sta na voljo prek kontekstnega menija komentarja v vmesniku, kadar je trenutni uporabnik avtor komentarja (ali skrbnik strani).

---
---