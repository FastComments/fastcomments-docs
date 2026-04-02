---
Het feed-systeem is een aparte SDK (`FastCommentsFeedSDK`) met zijn eigen weergave.

### Laden en weergeven van de feed

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
                // Toon deelvenster
            }
            .onUserClick { context, userInfo, source in
                // Navigeer naar gebruikersprofiel
            }
            .onMediaClick { mediaItem, index in
                // Toon afbeeldingsviewer op volledig scherm
            }
            .task {
                try? await sdk.load()
            }
    }
}
```

De feed-weergave bevat automatisch pull-to-refresh en infinite scroll.

### Berichten maken

Gebruik `FeedPostCreateView` om een formulier voor het aanmaken van een bericht weer te geven:

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

### Reageren op berichten

De SDK verwerkt reacties met optimistische updates:

```swift
try await sdk.reactPost(postId: post.id, reactionType: "l")

// Check reaction state
let hasLiked = sdk.hasUserReacted(postId: post.id, reactType: "l")
let likeCount = sdk.getLikeCount(postId: post.id)
```

### Reacties openen bij een bericht

Gebruik `CommentsSheet` om reacties voor een feed-bericht weer te geven. Het maakt intern een `FastCommentsSDK`-instantie aan met de configuratie van de feed SDK:

```swift
.sheet(item: $commentsPost) { post in
    CommentsSheet(post: post, feedSDK: sdk, onUserClick: { context, userInfo, source in
        // Verwerk gebruikersklik
    })
}
```

Opmerking: `FeedPost` moet voldoen aan `Identifiable` voor `.sheet(item:)`. Voeg deze extensie toe:

```swift
extension FeedPost: @retroactive Identifiable {}
```

### Filteren van de feed op basis van tags

Implementeer het `TagSupplier`-protocol om feed-berichten op tags te filteren:

```swift
struct TeamTagSupplier: TagSupplier {
    func getTags(currentUser: UserSessionInfo?) -> [String]? {
        guard let user = currentUser else { return nil }
        return ["team:\(user.id ?? "")", "public"]
    }
}

sdk.tagSupplier = TeamTagSupplier()
```

Geef `nil` terug voor een niet-gefilterde globale feed.

### Feedstatus opslaan en herstellen

Behoud de pagineringstoestand tijdens levenscyclusgebeurtenissen van de view:

```swift
let state = sdk.savePaginationState()
// Later...
sdk.restorePaginationState(state)
```

### Berichten verwijderen

```swift
sdk.onPostDeleted = { postId in
    print("Post \(postId) was deleted")
}
```

---
---