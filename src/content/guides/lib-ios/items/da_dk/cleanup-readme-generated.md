---
Når du er færdig med en SDK-instans (f.eks. når visningen lukkes), kald `cleanup()` for at lukke WebSocket-forbindelsen og annullere baggrundsopgaver:

```swift
sdk.cleanup()
```

For visninger styret af SwiftUI's `@StateObject` kaldes dette typisk i `.onDisappear` eller når visningen bliver deallokeret.

---
---