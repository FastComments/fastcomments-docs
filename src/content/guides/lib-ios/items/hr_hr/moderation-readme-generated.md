### Radnje dostupne svim korisnicima

- **Flag/Unflag** -- prijavite komentar na pregled

```swift
try await sdk.flagComment(commentId: commentId)
try await sdk.unflagComment(commentId: commentId)
```

- **Block/Unblock** -- sakrij sve komentare od korisnika (po pregledniku)

```swift
try await sdk.blockUser(commentId: commentId)
try await sdk.unblockUser(commentId: commentId)
```

### Radnje samo za administratore

- **Pin/Unpin** -- prikvačite komentar na vrh razgovora

```swift
try await sdk.pinComment(commentId: commentId)
try await sdk.unpinComment(commentId: commentId)
```

- **Lock/Unlock** -- sprječava nove odgovore na komentar i blokira uređivanja i brisanja dok nije otključan (primjenjuje se na sve, uključujući moderatore)

```swift
try await sdk.lockComment(commentId: commentId)
try await sdk.unlockComment(commentId: commentId)
```

Sve radnje moderiranja također su dostupne kroz kontekstni izbornik komentara u UI-ju. Administratorske radnje pojavljuju se samo kada je trenutni korisnik administrator stranice (postavljen putem SSO `isAdmin` oznake ili konfiguracije nadzorne ploče).