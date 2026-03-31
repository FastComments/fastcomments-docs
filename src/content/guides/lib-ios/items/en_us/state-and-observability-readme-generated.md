Both `FastCommentsSDK` and `FastCommentsFeedSDK` are `ObservableObject` classes with `@Published` properties. You can observe these in your SwiftUI views for reactive UI updates.

### FastCommentsSDK Published Properties

| Property | Type | Description |
|----------|------|-------------|
| `commentCountOnServer` | `Int` | Total comment count on the server |
| `newRootCommentCount` | `Int` | Buffered new comments (when `showLiveRightAway` is false) |
| `currentUser` | `UserSessionInfo?` | Current authenticated user |
| `isSiteAdmin` | `Bool` | Whether the current user is a site admin |
| `isClosed` | `Bool` | Whether the comment thread is closed |
| `hasBillingIssue` | `Bool` | Whether there is a billing problem |
| `isLoading` | `Bool` | Whether a network request is in progress |
| `hasMore` | `Bool` | Whether more pages of comments exist |
| `blockingErrorMessage` | `String?` | Error that prevents the UI from functioning |
| `warningMessage` | `String?` | Non-blocking warning message |
| `isDemo` | `Bool` | Whether running in demo mode |
| `commentsVisible` | `Bool` | Toggle for comment visibility |
| `toolbarEnabled` | `Bool` | Whether the formatting toolbar is shown |

### FastCommentsFeedSDK Published Properties

| Property | Type | Description |
|----------|------|-------------|
| `feedPosts` | `[FeedPost]` | Currently loaded feed posts |
| `hasMore` | `Bool` | Whether more pages exist |
| `currentUser` | `UserSessionInfo?` | Current authenticated user |
| `blockingErrorMessage` | `String?` | Blocking error message |
| `isLoading` | `Bool` | Whether a network request is in progress |
| `newPostsCount` | `Int` | Number of new posts since last load |

### Comment Tree

The comment tree is accessible via `sdk.commentsTree`:

```swift
// Flat list of visible nodes for rendering
sdk.commentsTree.visibleNodes

// Lookup a comment by ID
sdk.commentsTree.commentsById["comment-id"]
```

---