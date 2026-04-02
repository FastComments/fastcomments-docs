Le système de flux est un SDK séparé (`FastCommentsFeedSDK`) avec sa propre vue.

### Chargement et affichage du flux

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
                // Naviguer vers le profil utilisateur
            }
            .onMediaClick { mediaItem, index in
                // Présenter le visualiseur d'images en plein écran
            }
            .task {
                try? await sdk.load()
            }
    }
}
```

La vue du flux inclut automatiquement le pull-to-refresh et le défilement infini.

### Création de publications

Utilisez `FeedPostCreateView` pour afficher un formulaire de création de publication :

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

### Réagir aux publications

Le SDK gère les réactions avec des mises à jour optimistes :

```swift
try await sdk.reactPost(postId: post.id, reactionType: "l")

// Check reaction state
let hasLiked = sdk.hasUserReacted(postId: post.id, reactType: "l")
let likeCount = sdk.getLikeCount(postId: post.id)
```

### Ouvrir les commentaires d'une publication

Utilisez `CommentsSheet` pour afficher les commentaires d'une publication du flux. Il crée une instance `FastCommentsSDK` en interne en utilisant la configuration du SDK de flux :

```swift
.sheet(item: $commentsPost) { post in
    CommentsSheet(post: post, feedSDK: sdk, onUserClick: { context, userInfo, source in
        // Gérer le clic sur l'utilisateur
    })
}
```

Note : `FeedPost` doit conformer à `Identifiable` pour `.sheet(item:)`. Ajoutez cette extension :

```swift
extension FeedPost: @retroactive Identifiable {}
```

### Filtrage du flux par étiquette

Implémentez le protocole `TagSupplier` pour filtrer les publications du flux par étiquettes :

```swift
struct TeamTagSupplier: TagSupplier {
    func getTags(currentUser: UserSessionInfo?) -> [String]? {
        guard let user = currentUser else { return nil }
        return ["team:\(user.id ?? "")", "public"]
    }
}

sdk.tagSupplier = TeamTagSupplier()
```

Retournez `nil` pour un flux global sans filtre.

### Enregistrer et restaurer l'état du flux

Préservez l'état de la pagination lors des événements du cycle de vie de la vue :

```swift
let state = sdk.savePaginationState()
// Later...
sdk.restorePaginationState(state)
```

### Suppression de publications

```swift
sdk.onPostDeleted = { postId in
    print("Post \(postId) was deleted")
}
```