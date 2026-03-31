---
Αναζήτηση χρηστών για υποστήριξη της αυτόματης συμπλήρωσης @mention:

```swift
let results = try await sdk.searchUsers(query: "jan")
// Επιστρέφει [UserSearchResult] με userId, username, avatar, κ.λπ.
```

Το ενσωματωμένο `CommentInputBar` χειρίζεται την αυτόματη συμπλήρωση @mention.

---
---