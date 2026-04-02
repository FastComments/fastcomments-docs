### Dejanja na voljo vsem uporabnikom

- **Prijavi/Prekliči prijavo** -- prijavi komentar v pregled

```swift
try await sdk.flagComment(commentId: commentId)
try await sdk.unflagComment(commentId: commentId)
```

- **Blokiraj/Odblokiraj** -- skrij vse komentarje od uporabnika (za posameznega gledalca)

```swift
try await sdk.blockUser(commentId: commentId)
try await sdk.unblockUser(commentId: commentId)
```

### Dejanja samo za skrbnike

- **Pripni/Odpripni** -- pripni komentar na vrh niti

```swift
try await sdk.pinComment(commentId: commentId)
try await sdk.unpinComment(commentId: commentId)
```

- **Zakleni/Odkleni** -- prepreči nove odgovore na komentarju

```swift
try await sdk.lockComment(commentId: commentId)
try await sdk.unlockComment(commentId: commentId)
```

Vsa moderacijska dejanja so na voljo tudi prek kontekstnega menija komentarja v uporabniškem vmesniku (UI). Dejavnosti skrbnika se prikažejo samo, ko je trenutni uporabnik skrbnik strani (nastavljeno prek SSO zastavice `isAdmin` ali konfiguracije nadzorne plošče).

---
---