Le système de flux est un SDK distinct (`FastCommentsFeedSDK`) avec sa propre vue.

### Loading and Displaying the Feed

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
                // Présenter la feuille de partage
            }
            .onUserClick { context, userInfo, source in
                // Naviguer vers le profil de l'utilisateur
            }
            .onMediaClick { mediaItem, index in
                // Présenter le visualiseur d'image en plein écran
            }
            .task {
                try? await sdk.loadIfNeeded()
            }
    }
}
```

La vue du flux inclut automatiquement le pull-to-refresh et le défilement infini.
Utilisez `loadIfNeeded()` lors de la réapparition du cycle de vie de l'écran afin qu'un flux existant ou restauré ne revienne pas à la page 1.

### Creating Posts

Utilisez `FeedPostCreateView` pour présenter un formulaire de création de publication :

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

### Reacting to Posts

Le SDK gère les réactions avec des mises à jour optimistes :

```swift
try await sdk.reactPost(postId: post.id, reactionType: "l")

// Check reaction state
let hasLiked = sdk.hasUserReacted(postId: post.id, reactType: "l")
let likeCount = sdk.getLikeCount(postId: post.id)
```

### Opening Comments on a Post

Utilisez `CommentsSheet` pour afficher les commentaires d'une publication du flux. Il crée une instance `FastCommentsSDK` en interne en utilisant la config du SDK de flux :

```swift
.sheet(item: $commentsPost) { post in
    CommentsSheet(post: post, feedSDK: sdk, onUserClick: { context, userInfo, source in
        // Handle user click
    })
}
```

Note : `FeedPost` doit se conformer à `Identifiable` pour `.sheet(item:)`. Ajoutez cette extension :

```swift
extension FeedPost: @retroactive Identifiable {}
```

### Tag-Based Feed Filtering

Implémentez le protocole `TagSupplier` pour filtrer les publications du flux par tags :

```swift
struct TeamTagSupplier: TagSupplier {
    func getTags(currentUser: UserSessionInfo?) -> [String]? {
        guard let user = currentUser else { return nil }
        return ["team:\(user.id ?? "")", "public"]
    }
}

sdk.tagSupplier = TeamTagSupplier()
```

Retournez `nil` pour un flux global non filtré.

### Saving and Restoring Feed State

Conservez l'état de la pagination entre les événements du cycle de vie de la vue :

```swift
let state = sdk.savePaginationState()
// Later...
sdk.restorePaginationState(state)
try? await sdk.loadIfNeeded()
```

Si votre écran disparaît temporairement, la vue du flux met automatiquement en pause les mises à jour en direct et les reprend à la réapparition sans effacer les publications chargées. Appelez `sdk.cleanup()` uniquement lorsque vous avez vraiment terminé avec l'instance du SDK.

### Deleting Posts

```swift
sdk.onPostDeleted = { postId in
    print("Post \(postId) was deleted")
}
```