The feed system is a separate SDK (`FastCommentsFeedSDK`) with its own view.

### טעינה והצגת הפיד

```swift
struct FeedPage: View {
    @StateObject private var sdk: FastCommentsFeedSDK = {
        let config = FastCommentsWidgetConfig(
            tenantId: "YOUR_TENANT_ID",
            urlId: "my-feed",
            sso: ssoToken
        )
        return FastCommentsFeedSDK(config: config)
    }()

    @State private var commentsPost: FeedPost?

    var body: some View {
        FastCommentsFeedView(sdk: sdk)
            .onPostSelected { post in
                commentsPost = post
            }
            .onCommentsRequested { post in
                commentsPost = post
            }
            .onSharePost { post in
                // Present share sheet
            }
            .onUserClick { context, userInfo, source in
                // Navigate to user profile
            }
            .onMediaClick { mediaItem, index in
                // Present full-screen image viewer
            }
            .task {
                try? await sdk.load()
            }
    }
}
```

תצוגת הפיד כוללת משיכה לרענון וגלילה אינסופית באופן אוטומטי.

### יצירת פוסטים

Use `FeedPostCreateView` to present a post creation form:

```swift
@State private var showCreatePost = false

// In your view body:
.sheet(isPresented: $showCreatePost) {
    FeedPostCreateView(
        sdk: sdk,
        onPostCreated: { post in
            showCreatePost = false
            Task { try? await sdk.refresh() }
        },
        onCancelled: {
            showCreatePost = false
        }
    )
}
```

### תגובות לפוסטים

The SDK handles reactions with optimistic updates:

```swift
try await sdk.reactPost(postId: post.id, reactionType: "l")

// Check reaction state
let hasLiked = sdk.hasUserReacted(postId: post.id, reactType: "l")
let likeCount = sdk.getLikeCount(postId: post.id)
```

### פתיחת תגובות לפוסט

Use `CommentsSheet` to display comments for a feed post. It creates a `FastCommentsSDK` instance internally using the feed SDK's config:

```swift
.sheet(item: $commentsPost) { post in
    CommentsSheet(post: post, feedSDK: sdk, onUserClick: { context, userInfo, source in
        // Handle user click
    })
}
```

הערה: `FeedPost` חייב להתאים לפרוטוקול `Identifiable` עבור `.sheet(item:)`. הוסף את ההרחבה הזו:

```swift
extension FeedPost: @retroactive Identifiable {}
```

### סינון פיד מבוסס תגיות

Implement the `TagSupplier` protocol to filter feed posts by tags:

```swift
struct TeamTagSupplier: TagSupplier {
    func getTags(currentUser: UserSessionInfo?) -> [String]? {
        guard let user = currentUser else { return nil }
        return ["team:\(user.id ?? "")", "public"]
    }
}

sdk.tagSupplier = TeamTagSupplier()
```

החזר `nil` כדי לקבל פיד גלובלי ללא סינון.

### שמירה ושחזור מצב הפיד

Preserve pagination state across view lifecycle events:

```swift
let state = sdk.savePaginationState()
// Later...
sdk.restorePaginationState(state)
```

### מחיקת פוסטים

```swift
sdk.onPostDeleted = { postId in
    print("Post \(postId) was deleted")
}
```

---
---