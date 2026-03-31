---
Cuando haya terminado con una instancia del SDK (por ejemplo, cuando se descarta la vista), llame a `cleanup()` para cerrar la conexión WebSocket y cancelar tareas en segundo plano:

```swift
sdk.cleanup()
```

Para vistas gestionadas por `@StateObject` de SwiftUI, esto normalmente se llama en `.onDisappear` o cuando la vista se libera.

---
---