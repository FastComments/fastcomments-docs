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
                // Vis billedfremviser i fuld skærm
            }
            .task {
                try? await sdk.loadIfNeeded()
            }
    }
}
```

Feed-visningen inkluderer automatisk træk-for-at-opdatere og uendelig rullefunktion. Brug `loadIfNeeded()` ved genindtreden i skærmens livscyklus, så et eksisterende eller gendannet feed ikke nulstilles til side 1.

### Oprettelse af indlæg

Brug `FeedPostCreateView` til at vise en formular til oprettelse af indlæg:

```swift
@State private var showCreatePost = false

// I din view-krop:
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

### Reaktioner på indlæg

SDK'en håndterer reaktioner med optimistiske opdateringer:

```swift
try await sdk.reactPost(postId: post.id, reactionType: "l")

// Kontroller reaktionstilstand
let hasLiked = sdk.hasUserReacted(postId: post.id, reactType: "l")
let likeCount = sdk.getLikeCount(postId: post.id)
```

### Åbning af kommentarer på et indlæg

Brug `CommentsSheet` til at vise kommentarer for et feed-indlæg. Den opretter internt en `FastCommentsSDK`-instans ved hjælp af feed-SDK'ens konfiguration:

```swift
.sheet(item: $commentsPost) { post in
    CommentsSheet(post: post, feedSDK: sdk, onUserClick: { context, userInfo, source in
        // Håndter brugerklik
    })
}
```

Bemærk: `FeedPost` skal overholde `Identifiable` for `.sheet(item:)`. Tilføj denne extension:

```swift
extension FeedPost: @retroactive Identifiable {}
```

### Tag-baseret feed-filtrering

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

Returner `nil` for et ufiltreret globalt feed.

### Gemme og gendanne feed-tilstand

Bevar pagineringsstatus på tværs af visningens livscyklus-hændelser:

```swift
let state = sdk.savePaginationState()
// Later...
sdk.restorePaginationState(state)
try? await sdk.loadIfNeeded()
```

Hvis din skærm forsvinder midlertidigt, pauser feed-visningen automatisk live-opdateringer og genoptager dem ved genfremvisning uden at fjerne indlæste indlæg. Kald kun `sdk.cleanup()` når du virkelig er færdig med SDK-instansen.

### Sletning af indlæg

```swift
sdk.onPostDeleted = { postId in
    print("Post \(postId) was deleted")
}
```