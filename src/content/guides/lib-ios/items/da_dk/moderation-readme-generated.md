### Handlinger tilgængelige for alle brugere

- **Flag/Unflag** -- anmeld en kommentar til gennemgang

```swift
try await sdk.flagComment(commentId: commentId)
try await sdk.unflagComment(commentId: commentId)
```

- **Block/Unblock** -- skjul alle kommentarer fra en bruger (for hver enkelt seer)

```swift
try await sdk.blockUser(commentId: commentId)
try await sdk.unblockUser(commentId: commentId)
```

### Handlinger kun for administratorer

- **Pin/Unpin** -- fastgør en kommentar til toppen af tråden

```swift
try await sdk.pinComment(commentId: commentId)
try await sdk.unpinComment(commentId: commentId)
```

- **Lock/Unlock** -- forhindre nye svar på en kommentar

```swift
try await sdk.lockComment(commentId: commentId)
try await sdk.unlockComment(commentId: commentId)
```

Alle moderationshandlinger er også tilgængelige gennem kommentarens kontekstmenu i brugergrænsefladen. Admin-handlinger vises kun, når den aktuelle bruger er site-admin (angivet via SSO `isAdmin`-flaget eller dashboard-konfiguration).

---