### Acties beschikbaar voor alle gebruikers

- **Melden/Ontmelden** -- rapporteer een reactie ter beoordeling

```swift
try await sdk.flagComment(commentId: commentId)
try await sdk.unflagComment(commentId: commentId)
```

- **Blokkeren/Deblokkeren** -- verberg alle reacties van een gebruiker (per kijker)

```swift
try await sdk.blockUser(commentId: commentId)
try await sdk.unblockUser(commentId: commentId)
```

### Alleen voor beheerders

- **Vastpinnen/Lospinnen** -- zet een reactie vast bovenaan de draad

```swift
try await sdk.pinComment(commentId: commentId)
try await sdk.unpinComment(commentId: commentId)
```

- **Vergrendelen/Ontgrendelen** -- voorkom nieuwe antwoorden op een reactie

```swift
try await sdk.lockComment(commentId: commentId)
try await sdk.unlockComment(commentId: commentId)
```

Alle moderatieacties zijn ook beschikbaar via het contextmenu van de reactie in de UI. Beheerderacties verschijnen alleen wanneer de huidige gebruiker een sitebeheerder is (ingesteld via de SSO `isAdmin` vlag of de dashboardconfiguratie).

---
---