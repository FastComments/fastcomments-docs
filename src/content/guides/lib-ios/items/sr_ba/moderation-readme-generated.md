### Radnje dostupne svim korisnicima

- **Flag/Unflag** -- prijavite komentar na pregled

```swift
try await sdk.flagComment(commentId: commentId)
try await sdk.unflagComment(commentId: commentId)
```

- **Block/Unblock** -- sakrijte sve komentare od korisnika (po‑gledatelju)

```swift
try await sdk.blockUser(commentId: commentId)
try await sdk.unblockUser(commentId: commentId)
```

### Radnje samo za administratore

- **Pin/Unpin** -- zakačite komentar na vrh niti

```swift
try await sdk.pinComment(commentId: commentId)
try await sdk.unpinComment(commentId: commentId)
```

- **Lock/Unlock** -- spriječite nove odgovore na komentar i blokirajte uređivanje i brisanje dok se ne otključa (odnosi se na sve, uključujući moderatore)

```swift
try await sdk.lockComment(commentId: commentId)
try await sdk.unlockComment(commentId: commentId)
```

Sve akcije moderacije su također dostupne kroz kontekstni meni komentara u UI‑u. Administratorske radnje se pojavljuju samo kada je trenutni korisnik administrator stranice (postavljeno putem SSO `isAdmin` zastavice ili konfiguracije nadzorne ploče).