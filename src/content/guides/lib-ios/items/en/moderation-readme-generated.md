### Actions Available to All Users

- **Flag/Unflag** -- report a comment for review

```swift
try await sdk.flagComment(commentId: commentId)
try await sdk.unflagComment(commentId: commentId)
```

- **Block/Unblock** -- hide all comments from a user (per-viewer)

```swift
try await sdk.blockUser(commentId: commentId)
try await sdk.unblockUser(commentId: commentId)
```

### Admin-Only Actions

- **Pin/Unpin** -- pin a comment to the top of the thread

```swift
try await sdk.pinComment(commentId: commentId)
try await sdk.unpinComment(commentId: commentId)
```

- **Lock/Unlock** -- prevent new replies on a comment

```swift
try await sdk.lockComment(commentId: commentId)
try await sdk.unlockComment(commentId: commentId)
```

All moderation actions are also available through the comment context menu in the UI. Admin actions only appear when the current user is a site admin (set via SSO `isAdmin` flag or dashboard configuration).

---