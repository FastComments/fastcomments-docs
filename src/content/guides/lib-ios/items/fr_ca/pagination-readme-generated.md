### Taille de la page

```swift
// Commentaires : valeur par défaut 30
sdk.pageSize = 50

// Fil : valeur par défaut 10
feedSDK.pageSize = 20
```

### Charger plus de commentaires

L'interface affiche automatiquement les contrôles de pagination. Vous pouvez aussi déclencher la pagination par programme :

```swift
// Charger la page suivante
try await sdk.loadMore()

// Charger tout le reste (désactivé si >2000 commentaires pour des raisons de performance)
try await sdk.loadAll()

// Vérifier l'état
sdk.hasMore            // Indique s'il y a d'autres pages
sdk.shouldShowLoadAll()
sdk.getCountRemainingToShow()
```

### Pagination des commentaires enfants

Les réponses imbriquées sont chargées à la demande. Lorsque l'utilisateur développe un fil, les 5 premières réponses se chargent. Un contrôle "charger plus de réponses" apparaît s'il en existe d'autres. Ceci est géré automatiquement par l'interface.

---
---