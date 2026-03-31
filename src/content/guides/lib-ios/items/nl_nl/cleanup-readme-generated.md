---
Wanneer u klaar bent met een SDK-instantie (bijv. wanneer de view wordt gesloten), roept u `cleanup()` aan om de WebSocket-verbinding te sluiten en achtergrondtaken te annuleren:

```swift
sdk.cleanup()
```

Voor views die worden beheerd door SwiftUI's `@StateObject` wordt dit meestal aangeroepen in `.onDisappear` of wanneer de view wordt gedealloceerd.

---
---