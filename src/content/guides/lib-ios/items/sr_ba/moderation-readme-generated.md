### Radnje dostupne svim korisnicima

- **Prijavi/Opozovi prijavu** -- prijavi komentar na pregled

```swift
try await sdk.flagComment(commentId: commentId)
try await sdk.unflagComment(commentId: commentId)
```

- **Blokiraj/Odblokiraj** -- sakrij sve komentare od korisnika (po gledaocu)

```swift
try await sdk.blockUser(commentId: commentId)
try await sdk.unblockUser(commentId: commentId)
```

### Radnje samo za administratore

- **Zakači/Odkači** -- zakači komentar na vrh rasprave

```swift
try await sdk.pinComment(commentId: commentId)
try await sdk.unpinComment(commentId: commentId)
```

- **Zaključaj/Otključaj** -- spriječi nove odgovore na komentar

```swift
try await sdk.lockComment(commentId: commentId)
try await sdk.unlockComment(commentId: commentId)
```

Sve radnje moderacije su takođe dostupne kroz kontekstni meni komentara u korisničkom sučelju. Administratorske radnje se pojavljuju samo kada je trenutni korisnik administrator sajta (podešeno putem SSO `isAdmin` zastavice ili konfiguracije nadzorne ploče).

---
---