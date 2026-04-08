Le système de fil est un SDK séparé (`FastCommentsFeedSDK`) avec sa propre vue.

### Chargement et affichage du fil

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
                // Afficher la feuille de partage
            }
            .onUserClick { context, userInfo, source in
                // Naviguer vers le profil utilisateur
            }
            .onMediaClick { mediaItem, index in
                // Afficher le visualiseur d'images en plein écran
            }
            .task {
                try? await sdk.loadIfNeeded()
            }
    }
}
```

La vue du fil inclut automatiquement le « tirer pour rafraîchir » et le défilement infini.
Utilisez `loadIfNeeded()` lors du retour dans le cycle de vie de l'écran afin qu'un fil existant ou restauré ne soit pas remis à la page 1.

### Création de publications

Utilisez `FeedPostCreateView` pour présenter un formulaire de création de publication :

```swift
@State private var showCreatePost = false

// Dans le corps de votre vue :
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

// Vérifier l'état des réactions
let hasLiked = sdk.hasUserReacted(postId: post.id, reactType: "l")
let likeCount = sdk.getLikeCount(postId: post.id)
```

### Ouverture des commentaires d'une publication

Utilisez `CommentsSheet` pour afficher les commentaires d'une publication du fil. Il crée en interne une instance `FastCommentsSDK` en utilisant la configuration du SDK de feed :

```swift
.sheet(item: $commentsPost) { post in
    CommentsSheet(post: post, feedSDK: sdk, onUserClick: { context, userInfo, source in
        // Gérer le clic sur l'utilisateur
    })
}
```

Remarque : `FeedPost` doit implémenter `Identifiable` pour `.sheet(item:)`. Ajoutez cette extension :

```swift
extension FeedPost: @retroactive Identifiable {}
```

### Filtrage du fil par tags

Implémentez le protocole `TagSupplier` pour filtrer les publications du fil par tags :

```swift
struct TeamTagSupplier: TagSupplier {
    func getTags(currentUser: UserSessionInfo?) -> [String]? {
        guard let user = currentUser else { return nil }
        return ["team:\(user.id ?? "")", "public"]
    }
}

sdk.tagSupplier = TeamTagSupplier()
```

Retournez `nil` pour un fil global non filtré.

### Sauvegarde et restauration de l'état du fil

Conservez l'état de la pagination lors des événements du cycle de vie de la vue :

```swift
let state = sdk.savePaginationState()
// Later...
sdk.restorePaginationState(state)
try? await sdk.loadIfNeeded()
```

Si votre écran disparaît temporairement, la vue du fil met en pause automatiquement les mises à jour en direct et les reprend lorsqu'elle réapparaît sans effacer les publications chargées. Appelez `sdk.cleanup()` uniquement lorsque vous avez réellement terminé avec l'instance du SDK.

### Suppression de publications

```swift
sdk.onPostDeleted = { postId in
    print("Post \(postId) was deleted")
}
```

---
---