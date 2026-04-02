Les deux `FastCommentsSDK` et `FastCommentsFeedSDK` sont des classes `ObservableObject` disposant de propriétés `@Published`. Vous pouvez les observer dans vos vues SwiftUI pour mettre à jour l'interface utilisateur de manière réactive.

### FastCommentsSDK Published Properties

| Property | Type | Description |
|----------|------|-------------|
| `commentCountOnServer` | `Int` | Nombre total de commentaires sur le serveur |
| `newRootCommentCount` | `Int` | Commentaires nouveaux en attente (lorsque `showLiveRightAway` est false) |
| `currentUser` | `UserSessionInfo?` | Utilisateur actuellement authentifié |
| `isSiteAdmin` | `Bool` | Indique si l'utilisateur courant est administrateur du site |
| `isClosed` | `Bool` | Indique si le fil de commentaires est fermé |
| `hasBillingIssue` | `Bool` | Indique s'il y a un problème de facturation |
| `isLoading` | `Bool` | Indique si une requête réseau est en cours |
| `hasMore` | `Bool` | Indique s'il existe d'autres pages de commentaires |
| `blockingErrorMessage` | `String?` | Erreur empêchant le fonctionnement de l'interface utilisateur |
| `warningMessage` | `String?` | Message d'avertissement non bloquant |
| `isDemo` | `Bool` | Indique si l'application fonctionne en mode démo |
| `commentsVisible` | `Bool` | Contrôle la visibilité des commentaires |
| `toolbarEnabled` | `Bool` | Indique si la barre d'outils de formatage est affichée |

### FastCommentsFeedSDK Published Properties

| Property | Type | Description |
|----------|------|-------------|
| `feedPosts` | `[FeedPost]` | Publications du fil actuellement chargées |
| `hasMore` | `Bool` | Indique s'il existe d'autres pages |
| `currentUser` | `UserSessionInfo?` | Utilisateur actuellement authentifié |
| `blockingErrorMessage` | `String?` | Message d'erreur bloquant |
| `isLoading` | `Bool` | Indique si une requête réseau est en cours |
| `newPostsCount` | `Int` | Nombre de nouveaux posts depuis le dernier chargement |

### Comment Tree

L'arbre des commentaires est accessible via `sdk.commentsTree`:

```swift
// Liste plate des nœuds visibles pour le rendu
sdk.commentsTree.visibleNodes

// Rechercher un commentaire par ID
sdk.commentsTree.commentsById["comment-id"]
```

---
---