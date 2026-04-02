Feed-systemet er et separat SDK (`FastCommentsFeedSDK`) med sin egen visning.

### Indlæsning og visning af feedet

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
                // Vis delingsark
            }
            .onUserClick { context, userInfo, source in
                // Naviger til brugerprofil
            }
            .onMediaClick { mediaItem, index in
                // Vis billedviser i fuld skærm
            }
            .task {
                try? await sdk.load()
            }
    }
}
```

Feed-visningen inkluderer pull-to-refresh og automatisk uendelig scroll.

### Oprette indlæg

Brug `FeedPostCreateView` til at vise en formular til oprettelse af indlæg:

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

### Reagere på indlæg

SDK'en håndterer reaktioner med optimistiske opdateringer:

```swift
try await sdk.reactPost(postId: post.id, reactionType: "l")

// Check reaction state
let hasLiked = sdk.hasUserReacted(postId: post.id, reactType: "l")
let likeCount = sdk.getLikeCount(postId: post.id)
```

### Åbne kommentarer for et indlæg

Brug `CommentsSheet` til at vise kommentarer for et feed-indlæg. Den opretter internt en `FastCommentsSDK`-instans ved hjælp af feed-SDK'ens konfiguration:

```swift
.sheet(item: $commentsPost) { post in
    CommentsSheet(post: post, feedSDK: sdk, onUserClick: { context, userInfo, source in
        // Håndter klik på bruger
    })
}
```

Bemærk: `FeedPost` skal overholde `Identifiable` for `.sheet(item:)`. Tilføj denne extension:

```swift
extension FeedPost: @retroactive Identifiable {}
```

### Tag-baseret filtrering af feed

Implementer `TagSupplier`-protokollen for at filtrere feed-indlæg efter tags:

```swift
struct TeamTagSupplier: TagSupplier {
    func getTags(currentUser: UserSessionInfo?) -> [String]? {
        guard let user = currentUser else { return nil }
        return ["team:\(user.id ?? "")", "public"]
    }
}

sdk.tagSupplier = TeamTagSupplier()
```

Returnér `nil` for et ufiltreret globalt feed.

### Gemme og gendanne feed-tilstand

Bevar pagineringsstatus på tværs af visningens livscyklus-begivenheder:

```swift
let state = sdk.savePaginationState()
// Later...
sdk.restorePaginationState(state)
```

### Slette indlæg

```swift
sdk.onPostDeleted = { postId in
    print("Post \(postId) was deleted")
}
```

---
---