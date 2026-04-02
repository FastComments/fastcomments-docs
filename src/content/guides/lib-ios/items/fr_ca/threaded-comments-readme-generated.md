### Utilisation de base

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

### Styles de vote

Le style de vote par défaut affiche des flèches haut/bas. Passez `._1` pour un style de vote en forme de cœur :

```swift
FastCommentsView(sdk: sdk, voteStyle: ._1)
```

| Style | Apparence |
|-------|------------|
| `._0` | Boutons flèche haut/bas affichant le total net |
| `._1` | Bouton en forme de cœur unique avec le total |

### Rappels d'événements

Utilisez des callbacks de type modificateur pour gérer les interactions des utilisateurs :

```swift
FastCommentsView(sdk: sdk)
    .onCommentPosted { comment in
        print("New comment: \(comment.commentHTML)")
    }
    .onReplyClick { renderableComment in
        print("Replying to: \(renderableComment.comment.id)")
    }
    .onUserClick { context, userInfo, source in
        // source est .name ou .avatar
        print("Tapped \(userInfo.displayName)")
    }
```

### Application d'un thème

Passez un thème via l'environnement SwiftUI :

```swift
FastCommentsView(sdk: sdk)
    .fastCommentsTheme(myTheme)
    .task { try? await sdk.load() }
```

Ou définissez-le directement sur le SDK :

```swift
sdk.theme = FastCommentsTheme.modern
```

### Direction de tri

```swift
sdk.defaultSortDirection = .nf  // Les plus récents d'abord (par défaut)
sdk.defaultSortDirection = .of  // Les plus anciens d'abord
sdk.defaultSortDirection = .mr  // Les plus pertinents
```

---
---