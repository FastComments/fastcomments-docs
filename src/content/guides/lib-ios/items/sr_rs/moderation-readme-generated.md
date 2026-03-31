### Radnje dostupne svim korisnicima

- **Označi/Ukloni oznaku** -- prijavi komentar na pregled

```swift
try await sdk.flagComment(commentId: commentId)
try await sdk.unflagComment(commentId: commentId)
```

- **Blokiraj/Odblokiraj** -- sakrij sve komentare od korisnika (po posmatraču)

```swift
try await sdk.blockUser(commentId: commentId)
try await sdk.unblockUser(commentId: commentId)
```

### Radnje samo za administratore

- **Postavi na vrh/Ukloni sa vrha** -- postavi komentar na vrh niti

```swift
try await sdk.pinComment(commentId: commentId)
try await sdk.unpinComment(commentId: commentId)
```

- **Zaključaj/Otključaj** -- onemogući nove odgovore na komentar

```swift
try await sdk.lockComment(commentId: commentId)
try await sdk.unlockComment(commentId: commentId)
```

Sve radnje moderacije su takođe dostupne kroz kontekstni meni komentara u korisničkom interfejsu. Administratorske radnje se pojavljuju samo kada je trenutni korisnik administrator sajta (postavljeno putem SSO `isAdmin` zastavice ili konfiguracije kontrolne table).

---
---