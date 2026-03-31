---
Quand vous avez terminé avec une instance du SDK (par exemple, lorsque la vue est en train d'être fermée), appelez `cleanup()` pour fermer la connexion WebSocket et annuler les tâches en arrière-plan :

```swift
sdk.cleanup()
```

Pour les vues gérées par `@StateObject` de SwiftUI, ceci est généralement appelé dans `.onDisappear` ou lorsque la vue est désallouée.

---
---