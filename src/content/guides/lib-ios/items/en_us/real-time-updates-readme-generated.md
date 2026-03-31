After calling `sdk.load()`, the SDK automatically subscribes to WebSocket events for the configured `urlId`. The following events are handled:

- New comments, edits, and deletions
- Votes (new and removed)
- Pin, lock, flag, and block state changes
- User presence (join/leave)
- Thread open/close
- Badge awards
- Server configuration updates

### Controlling Live Display

By default, new comments from other users appear immediately:

```swift
sdk.showLiveRightAway = true   // Default: show instantly
```

Set this to `false` to buffer new comments behind a "N new comments" button, letting the user choose when to reveal them:

```swift
sdk.showLiveRightAway = false
```

### User Presence

Online/offline indicators appear automatically on user avatars when the server enables presence tracking. No additional configuration is needed on the client.

---