### Handlinger Tilgængelige for Alle Brugere

- **Flag/Unflag** -- rapportér en kommentar til gennemsyn

```swift
try await sdk.flagComment(commentId: commentId)
try await sdk.unflagComment(commentId: commentId)
```

- **Block/Unblock** -- skjul alle kommentarer fra en bruger (pr. visning)

```swift
try await sdk.blockUser(commentId: commentId)
try await sdk.unblockUser(commentId: commentId)
```

### Kun Admin-handlinger

- **Pin/Unpin** -- fastgør en kommentar til toppen af tråden

```swift
try await sdk.pinComment(commentId: commentId)
try await sdk.unpinComment(commentId: commentId)
```

- **Lock/Unlock** -- forhindre nye svar på en kommentar, og blokere redigeringer og sletninger indtil den låses op (gælder for alle, inklusive moderatorer)

```swift
try await sdk.lockComment(commentId: commentId)
try await sdk.unlockComment(commentId: commentId)
```

Alle moderationshandlinger er også tilgængelige via kommentarens kontekstmenu i UI'et. Admin-handlinger vises kun, når den aktuelle bruger er en site-admin (sat via SSO `isAdmin`-flaget eller dashboard-konfigurationen).