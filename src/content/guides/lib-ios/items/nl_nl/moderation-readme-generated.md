---
### Acties Beschikbaar voor Alle Gebruikers

- **Markeren/Demarkeren** -- een opmerking melden voor beoordeling

```swift
try await sdk.flagComment(commentId: commentId)
try await sdk.unflagComment(commentId: commentId)
```

- **Blokkeren/De-blokkeren** -- verberg alle opmerkingen van een gebruiker (per kijker)

```swift
try await sdk.blockUser(commentId: commentId)
try await sdk.unblockUser(commentId: commentId)
```

### Alleen voor Beheerders

- **Vastzetten/Losmaken** -- een opmerking bovenaan de thread vastzetten

```swift
try await sdk.pinComment(commentId: commentId)
try await sdk.unpinComment(commentId: commentId)
```

- **Vergrendelen/Ontgrendelen** -- voorkom nieuwe antwoorden op een opmerking, en blokkeer bewerkingen en verwijderingen tot het is ontgrendeld (geldt voor iedereen, inclusief moderators)

```swift
try await sdk.lockComment(commentId: commentId)
try await sdk.unlockComment(commentId: commentId)
```

Alle moderatie‑acties zijn ook beschikbaar via het contextmenu van de opmerking in de UI. Beheerdersacties verschijnen alleen wanneer de huidige gebruiker een site‑beheerder is (ingesteld via SSO `isAdmin` vlag of dashboardconfiguratie).

---
---