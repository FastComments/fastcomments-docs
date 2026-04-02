---
Zoek naar gebruikers ter ondersteuning van @mention-autocompletie:

```swift
let results = try await sdk.searchUsers(query: "jan")
// Geeft [UserSearchResult] terug met userId, username, avatar, enz.
```

De ingebouwde `CommentInputBar` verzorgt automatisch de @mention-autocompletie.

---
---