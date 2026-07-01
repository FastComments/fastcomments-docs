### Radnje dostupne svim korisnicima

- **Flag/Unflag** -- prijavi komentar za reviziju

```swift
try await sdk.flagComment(commentId: commentId)
try await sdk.unflagComment(commentId: commentId)
```

- **Block/Unblock** -- sakrij sve komentare od korisnika (po gledatelju)

```swift
try await sdk.blockUser(commentId: commentId)
try await sdk.unblockUser(commentId: commentId)
```

### Akcije samo za admina

- **Pin/Unpin** -- zakači komentar na vrh teme

```swift
try await sdk.pinComment(commentId: commentId)
try await sdk.unpinComment(commentId: commentId)
```

- **Lock/Unlock** -- spriječi nove odgovore na komentar i blokiraj uređivanja i brisanje dok se ne otključa (važi za sve, uključujući moderatore)

```swift
try await sdk.lockComment(commentId: commentId)
try await sdk.unlockComment(commentId: commentId)
```

Sve akcije moderacije takođe su dostupne kroz kontekstni meni komentara u UI. Administratorske akcije se prikazuju samo kada je trenutni korisnik admin sajta (postavljen putem SSO `isAdmin` flag ili konfiguracijom na kontrolnoj tabli).