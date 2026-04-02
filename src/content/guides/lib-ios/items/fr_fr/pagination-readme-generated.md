### Taille de page

```swift
// Commentaires : 30 par défaut
sdk.pageSize = 50

// Flux : 10 par défaut
feedSDK.pageSize = 20
```

### Chargement de commentaires supplémentaires

L'interface affiche automatiquement les contrôles de pagination. Vous pouvez également déclencher la pagination par programmation :

```swift
// Charger la page suivante
try await sdk.loadMore()

// Charger tous les restants (désactivé si >2000 commentaires pour des raisons de performances)
try await sdk.loadAll()

// Vérifier l'état
sdk.hasMore            // S'il existe d'autres pages
sdk.shouldShowLoadAll()
sdk.getCountRemainingToShow()
```

### Pagination des réponses imbriquées

Les réponses imbriquées sont chargées à la demande. Lorsqu'un utilisateur développe un fil, les 5 premières réponses sont chargées. Un contrôle « charger plus de réponses » apparaît s'il y en a d'autres. Ceci est géré automatiquement par l'interface.

---
---