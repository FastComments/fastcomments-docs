---
Quando hai finito di usare un'istanza dell'SDK (ad es., la vista viene chiusa), chiama `cleanup()` per chiudere la connessione WebSocket e annullare le attività in background:

```swift
sdk.cleanup()
```

Per le viste gestite da `@StateObject` di SwiftUI, questo viene tipicamente chiamato in `.onDisappear` o quando la vista viene deallocata.

---
---