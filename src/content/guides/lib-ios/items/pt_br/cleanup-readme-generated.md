Quando você terminar com uma instância do SDK (por exemplo, quando a view estiver sendo dispensada), chame `cleanup()` para fechar a conexão WebSocket e cancelar tarefas em segundo plano:

```swift
sdk.cleanup()
```

Para views gerenciadas pelo `@StateObject` do SwiftUI, isso normalmente é chamado em `.onDisappear` ou quando a view é desalocada.