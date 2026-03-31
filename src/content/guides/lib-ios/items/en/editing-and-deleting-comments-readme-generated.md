### Edit

```swift
try await sdk.editComment(commentId: commentId, newText: "Updated text")
```

The server re-renders the HTML. The local comment updates automatically.

### Delete

```swift
try await sdk.deleteComment(commentId: commentId)
```

Deleting a comment also removes its descendants from the local tree.

Both actions are available through the comment context menu in the UI when the current user is the comment author (or a site admin).

---