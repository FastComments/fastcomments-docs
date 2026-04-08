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
                // Toon het deelvenster
            }
            .onUserClick { context, userInfo, source in
                // Navigeer naar gebruikersprofiel
            }
            .onMediaClick { mediaItem, index in
                // Toon afbeeldingsviewer op volledig scherm
            }
            .task {
                try? await sdk.loadIfNeeded()
            }
    }
}
```

De feed-weergave bevat automatisch pull-to-refresh en infinite scroll.
Gebruik `loadIfNeeded()` bij het terugkeren in de lifecycle van het scherm, zodat een bestaande of herstelde feed niet teruggezet wordt naar pagina 1.

### Berichten maken

Gebruik `FeedPostCreateView` om een formulier voor het maken van een bericht te tonen:

```swift
@State private var showCreatePost = false

// In de body van je view:
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

### Reacties op een bericht openen

Gebruik `CommentsSheet` om reacties voor een feed-bericht weer te geven. Deze maakt intern een `FastCommentsSDK`-instantie aan met de config van de feed SDK:

```swift
.sheet(item: $commentsPost) { post in
    CommentsSheet(post: post, feedSDK: sdk, onUserClick: { context, userInfo, source in
        // Behandel gebruikersklik
    })
}
```

Opmerking: `FeedPost` moet voldoen aan `Identifiable` voor `.sheet(item:)`. Voeg deze extensie toe:

```swift
extension FeedPost: @retroactive Identifiable {}
```

### Feedfiltering op basis van tags

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

### Feed-status opslaan en herstellen

Behoud de paginatoestand over weergave-levenscyclusgebeurtenissen:

```swift
let state = sdk.savePaginationState()
// Later...
sdk.restorePaginationState(state)
try? await sdk.loadIfNeeded()
```

Als je scherm tijdelijk verdwijnt, pauzeert de feed-weergave automatisch live-updates en hervat deze bij terugkeer zonder geladen berichten te wissen. Roep `sdk.cleanup()` alleen aan wanneer je echt klaar bent met de SDK-instantie.

### Berichten verwijderen

```swift
sdk.onPostDeleted = { postId in
    print("Post \(postId) was deleted")
}
```