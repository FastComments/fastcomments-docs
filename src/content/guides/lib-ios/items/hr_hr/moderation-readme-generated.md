### Radnje dostupne svim korisnicima

- **Prijavi/Otkaži prijavu** -- prijavite komentar na pregled

```swift
try await sdk.flagComment(commentId: commentId)
try await sdk.unflagComment(commentId: commentId)
```

- **Blokiraj/Odblokiraj** -- sakrij sve komentare od korisnika (po gledatelju)

```swift
try await sdk.blockUser(commentId: commentId)
try await sdk.unblockUser(commentId: commentId)
```

### Radnje samo za administratore

- **Prikvači/Odveži** -- prikvači komentar na vrh rasprave

```swift
try await sdk.pinComment(commentId: commentId)
try await sdk.unpinComment(commentId: commentId)
```

- **Zaključaj/Otključaj** -- spriječi nove odgovore na komentar

```swift
try await sdk.lockComment(commentId: commentId)
try await sdk.unlockComment(commentId: commentId)
```

Sve radnje moderiranja također su dostupne kroz kontekstni izbornik komentara u korisničkom sučelju. Administratorske radnje pojavljuju se samo kada je trenutni korisnik administrator web-mjesta (postavljeno putem SSO `isAdmin` zastavice ili konfiguracije nadzorne ploče).

---
---