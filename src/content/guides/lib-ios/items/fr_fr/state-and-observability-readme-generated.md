`FastCommentsSDK` et `FastCommentsFeedSDK` sont des classes `ObservableObject` avec des propriétés `@Published`. Vous pouvez les observer dans vos vues SwiftUI pour des mises à jour réactives de l'interface.

### Propriétés Published de FastCommentsSDK

| Propriété | Type | Description |
|----------|------|-------------|
| `commentCountOnServer` | `Int` | Nombre total de commentaires sur le serveur |
| `newRootCommentCount` | `Int` | Nouveaux commentaires mis en mémoire tampon (lorsque `showLiveRightAway` est false) |
| `currentUser` | `UserSessionInfo?` | Utilisateur authentifié actuel |
| `isSiteAdmin` | `Bool` | Indique si l'utilisateur actuel est un administrateur du site |
| `isClosed` | `Bool` | Indique si le fil de commentaires est fermé |
| `hasBillingIssue` | `Bool` | Indique s'il y a un problème de facturation |
| `isLoading` | `Bool` | Indique si une requête réseau est en cours |
| `hasMore` | `Bool` | Indique s'il existe d'autres pages de commentaires |
| `blockingErrorMessage` | `String?` | Erreur empêchant le fonctionnement de l'interface |
| `warningMessage` | `String?` | Message d'avertissement non bloquant |
| `isDemo` | `Bool` | Indique si l'application fonctionne en mode démo |
| `commentsVisible` | `Bool` | Bascule pour la visibilité des commentaires |
| `toolbarEnabled` | `Bool` | Indique si la barre de formatage est affichée |

### Propriétés Published de FastCommentsFeedSDK

| Propriété | Type | Description |
|----------|------|-------------|
| `feedPosts` | `[FeedPost]` | Publications du fil actuellement chargées |
| `hasMore` | `Bool` | Indique s'il existe d'autres pages |
| `currentUser` | `UserSessionInfo?` | Utilisateur authentifié actuel |
| `blockingErrorMessage` | `String?` | Message d'erreur bloquant |
| `isLoading` | `Bool` | Indique si une requête réseau est en cours |
| `newPostsCount` | `Int` | Nombre de nouvelles publications depuis le dernier chargement |

### Arbre des commentaires

L'arbre des commentaires est accessible via `sdk.commentsTree` :

```swift
// Flat list of visible nodes for rendering
sdk.commentsTree.visibleNodes

// Lookup a comment by ID
sdk.commentsTree.commentsById["comment-id"]
```

---
---