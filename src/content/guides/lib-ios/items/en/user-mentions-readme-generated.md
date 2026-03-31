Search for users to support @mention autocomplete:

```swift
let results = try await sdk.searchUsers(query: "jan")
// Returns [UserSearchResult] with userId, username, avatar, etc.
```

The built-in `CommentInputBar` handles @mention autocomplete automatically.

---