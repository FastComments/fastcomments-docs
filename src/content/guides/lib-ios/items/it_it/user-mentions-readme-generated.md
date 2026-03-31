---
Cerca utenti per supportare l'autocompletamento delle @mention:

```swift
let results = try await sdk.searchUsers(query: "jan")
// Restituisce [UserSearchResult] con userId, username, avatar, ecc.
```

La `CommentInputBar` integrata gestisce automaticamente l'autocompletamento delle @mention.

---
---