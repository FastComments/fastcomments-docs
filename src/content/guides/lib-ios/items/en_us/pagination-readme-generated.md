### Page Size

```swift
// Comments: default 30
sdk.pageSize = 50

// Feed: default 10
feedSDK.pageSize = 20
```

### Loading More Comments

The UI shows pagination controls automatically. You can also trigger pagination programmatically:

```swift
// Load next page
try await sdk.loadMore()

// Load all remaining (disabled if >2000 comments for performance)
try await sdk.loadAll()

// Check state
sdk.hasMore            // Whether more pages exist
sdk.shouldShowLoadAll()
sdk.getCountRemainingToShow()
```

### Child Comment Pagination

Nested replies load lazily. When a user expands a thread, the first 5 children load. A "load more replies" control appears if more exist. This is handled automatically by the UI.

---