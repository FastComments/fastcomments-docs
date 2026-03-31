Les deux `FastCommentsSDK` et `FastCommentsFeedSDK` sont des classes `ObservableObject` avec des propriétés `@Published`. Vous pouvez les observer dans vos vues SwiftUI pour des mises à jour réactives de l'interface utilisateur.

### Propriétés @Published de FastCommentsSDK

| Property | Type | Description |
|----------|------|-------------|
| `commentCountOnServer` | `Int` | Nombre total de commentaires sur le serveur |
| `newRootCommentCount` | `Int` | Nouveaux commentaires mis en tampon (lorsque `showLiveRightAway` est false) |
| `currentUser` | `UserSessionInfo?` | Utilisateur actuellement authentifié |
| `isSiteAdmin` | `Bool` | Si l'utilisateur actuel est un administrateur du site |
| `isClosed` | `Bool` | Si le fil de commentaires est fermé |
| `hasBillingIssue` | `Bool` | S'il y a un problème de facturation |
| `isLoading` | `Bool` | Si une requête réseau est en cours |
| `hasMore` | `Bool` | S'il existe d'autres pages de commentaires |
| `blockingErrorMessage` | `String?` | Erreur empêchant le fonctionnement de l'interface |
| `warningMessage` | `String?` | Message d'avertissement non bloquant |
| `isDemo` | `Bool` | Si l'exécution est en mode démo |
| `commentsVisible` | `Bool` | Basculer la visibilité des commentaires |
| `toolbarEnabled` | `Bool` | Si la barre d'outils de formatage est affichée |

### Propriétés @Published de FastCommentsFeedSDK

| Property | Type | Description |
|----------|------|-------------|
| `feedPosts` | `[FeedPost]` | Publications du fil actuellement chargées |
| `hasMore` | `Bool` | S'il existe d'autres pages |
| `currentUser` | `UserSessionInfo?` | Utilisateur actuellement authentifié |
| `blockingErrorMessage` | `String?` | Message d'erreur bloquant |
| `isLoading` | `Bool` | Si une requête réseau est en cours |
| `newPostsCount` | `Int` | Nombre de nouvelles publications depuis le dernier chargement |

### Arbre de commentaires

L'arbre de commentaires est accessible via `sdk.commentsTree`:

```swift
// Liste aplatie des nœuds visibles pour le rendu
sdk.commentsTree.visibleNodes

// Rechercher un commentaire par ID
sdk.commentsTree.commentsById["comment-id"]
```

---
---