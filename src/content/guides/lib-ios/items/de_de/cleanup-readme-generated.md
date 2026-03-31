---
Wenn Sie mit einer SDK-Instanz fertig sind (z. B. wenn die Ansicht verworfen wird), rufen Sie `cleanup()` auf, um die WebSocket-Verbindung zu schließen und Hintergrundaufgaben abzubrechen:

```swift
sdk.cleanup()
```

Für Ansichten, die von SwiftUI's `@StateObject` verwaltet werden, wird dies typischerweise in `.onDisappear` aufgerufen oder wenn die Ansicht dealloziert wird.

---
---