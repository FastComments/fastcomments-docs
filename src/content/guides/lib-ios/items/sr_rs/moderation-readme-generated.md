### Radnje dostupne svim korisnicima

- **Flag/Unflag** -- prijavite komentar za reviziju

```swift
try await sdk.flagComment(commentId: commentId)
try await sdk.unflagComment(commentId: commentId)
```

- **Block/Unblock** -- sakrijte sve komentare korisnika (po‑gledatelju)

```swift
try await sdk.blockUser(commentId: commentId)
try await sdk.unblockUser(commentId: commentId)
```

### Akcije samo za administratore

- **Pin/Unpin** -- zakačite komentar na vrh teme

```swift
try await sdk.pinComment(commentId: commentId)
try await sdk.unpinComment(commentId: commentId)
```

- **Lock/Unlock** -- sprečite nove odgovore na komentar i blokirajte izmene i brisanja dok se ne otključa (važi za sve, uključujući moderatore)

```swift
try await sdk.lockComment(commentId: commentId)
try await sdk.unlockComment(commentId: commentId)
```

Sve akcije moderacije takođe su dostupne putem kontekstnog menija komentara u UI‑u. Administratorske akcije se prikazuju samo kada je trenutni korisnik administrator sajta (podešeno putem SSO `isAdmin` zastavice ili konfiguracije na kontrolnoj tabli).