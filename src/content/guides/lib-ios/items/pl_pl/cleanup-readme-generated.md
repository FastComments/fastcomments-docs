Gdy zakończysz pracę z instancją SDK (np. gdy widok jest zamykany), wywołaj `cleanup()`, aby zamknąć połączenie WebSocket i anulować zadania w tle:

```swift
sdk.cleanup()
```

W przypadku widoków zarządzanych przez SwiftUI przy użyciu `@StateObject`, zwykle wywołuje się to w `.onDisappear` lub gdy widok zostanie zdealokowany.