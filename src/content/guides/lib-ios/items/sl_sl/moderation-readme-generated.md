### Dejanje, ki je na voljo vsem uporabnikom

- **Flag/Unflag** – prijavi komentar v pregled

```swift
try await sdk.flagComment(commentId: commentId)
try await sdk.unflagComment(commentId: commentId)
```

- **Block/Unblock** – skrij vse komentarje določenega uporabnika (za posameznega gledalca)

```swift
try await sdk.blockUser(commentId: commentId)
try await sdk.unblockUser(commentId: commentId)
```

### Dejanja, ki so na voljo samo skrbnikom

- **Pin/Unpin** – pripni komentar na vrh niza

```swift
try await sdk.pinComment(commentId: commentId)
try await sdk.unpinComment(commentId: commentId)
```

- **Lock/Unlock** – prepreči nove odgovore na komentar in blokiraj urejanja ter brisanja, dokler ni odklenjeno (velja za vse, tudi za moderatorje)

```swift
try await sdk.lockComment(commentId: commentId)
try await sdk.unlockComment(commentId: commentId)
```

Vsa moderacijska dejanja so prav tako na voljo preko kontekstnega menija komentarjev v uporabniškem vmesniku. Dejanja skrbnika se prikažejo le, ko je trenutni uporabnik spletiščni skrbnik (nastavljeno prek SSO `isAdmin` zastavice ali konfiguracije v nadzorni plošči).