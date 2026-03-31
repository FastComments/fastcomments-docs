### Basic Usage

```swift
struct CommentsPage: View {
    @StateObject private var sdk = FastCommentsSDK(
        config: FastCommentsWidgetConfig(
            tenantId: "YOUR_TENANT_ID",
            urlId: "article-42",
            url: "https://example.com/article/42",
            pageTitle: "Article Title"
        )
    )

    var body: some View {
        FastCommentsView(sdk: sdk)
            .task {
                try? await sdk.load()
            }
    }
}
```

### Vote Styles

The default vote style shows up/down arrows. Pass `._1` for heart-style votes:

```swift
FastCommentsView(sdk: sdk, voteStyle: ._1)
```

| Style | Appearance |
|-------|------------|
| `._0` | Up/down arrow buttons with net count |
| `._1` | Single heart button with count |

### Event Callbacks

Use modifier-style callbacks to handle user interactions:

```swift
FastCommentsView(sdk: sdk)
    .onCommentPosted { comment in
        print("New comment: \(comment.commentHTML)")
    }
    .onReplyClick { renderableComment in
        print("Replying to: \(renderableComment.comment.id)")
    }
    .onUserClick { context, userInfo, source in
        // source is .name or .avatar
        print("Tapped \(userInfo.displayName)")
    }
```

### Applying a Theme

Pass a theme through the SwiftUI environment:

```swift
FastCommentsView(sdk: sdk)
    .fastCommentsTheme(myTheme)
    .task { try? await sdk.load() }
```

Or set it directly on the SDK:

```swift
sdk.theme = FastCommentsTheme.modern
```

### Sort Direction

```swift
sdk.defaultSortDirection = .nf  // Newest first (default)
sdk.defaultSortDirection = .of  // Oldest first
sdk.defaultSortDirection = .mr  // Most relevant
```

---