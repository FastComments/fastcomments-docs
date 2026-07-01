### Actions Available to All Users

- **Flag/Unflag** -- prijavite komentar za pregled

```swift
try await sdk.flagComment(commentId: commentId)
try await sdk.unflagComment(commentId: commentId)
```

- **Block/Unblock** -- sakrijte sve komentare od korisnika (po‑gledatelju)

```swift
try await sdk.blockUser(commentId: commentId)
try await sdk.unblockUser(commentId: commentId)
```

### Admin-Only Actions

- **Pin/Unpin** -- prikačite komentar na vrh teme

```swift
try await sdk.pinComment(commentId: commentId)
try await sdk.unpinComment(commentId: commentId)
```

- **Lock/Unlock** -- spriječite nove odgovore na komentar, i blokirajte uređivanja i brisanja dok nije otključan (važi za sve, uključujući moderatore)

```swift
try await sdk.lockComment(commentId: commentId)
try await sdk.unlockComment(commentId: commentId)
```

All moderation actions are also available through the comment context menu in the UI. Admin actions only appear when the current user is a site admin (set via SSO `isAdmin` flag or dashboard configuration).