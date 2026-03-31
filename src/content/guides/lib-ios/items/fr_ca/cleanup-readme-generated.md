---
Lorsque vous avez terminé avec une instance du SDK (p. ex., la vue est sur le point d'être fermée), appelez `cleanup()` pour fermer la connexion WebSocket et annuler les tâches d'arrière-plan :

```swift
sdk.cleanup()
```

Pour les vues gérées par `@StateObject` de SwiftUI, cela est généralement appelé dans `.onDisappear` ou lorsque la vue est désallouée.
---
---