@mention ऑटोकम्प्लीट के लिए उपयोगकर्ताओं को खोजें:

```swift
let results = try await sdk.searchUsers(query: "jan")
// Returns [UserSearchResult] with userId, username, avatar, etc.
```

बिल्ट-इन `CommentInputBar` @mention ऑटोकम्प्लीट को स्वचालित रूप से संभालता है।