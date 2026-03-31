### Akcije dostupne svim korisnicima

- **Prijavi/Ukloni prijavu** -- prijavi komentar na pregled

```swift
try await sdk.flagComment(commentId: commentId)
try await sdk.unflagComment(commentId: commentId)
```

- **Blokiraj/Odblokiraj** -- sakrij sve komentare korisnika (za pojedinačnog gledatelja)

```swift
try await sdk.blockUser(commentId: commentId)
try await sdk.unblockUser(commentId: commentId)
```

### Samo za administratore

- **Zakači/Otkači** -- zakači komentar na vrh niti

```swift
try await sdk.pinComment(commentId: commentId)
try await sdk.unpinComment(commentId: commentId)
```

- **Zaključaj/Otključaj** -- onemogući nove odgovore na komentar

```swift
try await sdk.lockComment(commentId: commentId)
try await sdk.unlockComment(commentId: commentId)
```

Sve moderacijske radnje su takođe dostupne kroz kontekstni meni komentara u UI. Administratorske radnje se pojavljuju samo kada je trenutni korisnik administrator sajta (podešeno putem SSO `isAdmin` flag ili konfiguracije na kontrolnoj tabli).

---
---