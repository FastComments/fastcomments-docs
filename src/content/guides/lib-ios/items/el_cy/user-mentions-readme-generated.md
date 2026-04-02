---
Αναζήτηση χρηστών για υποστήριξη της αυτόματης συμπλήρωσης @mention:

```swift
let results = try await sdk.searchUsers(query: "jan")
// Επιστρέφει [UserSearchResult] με userId, username, avatar, κ.λπ.
```

Το ενσωματωμένο `CommentInputBar` αναλαμβάνει την αυτόματη συμπλήρωση για @mention.

---
---