When you are done with an SDK instance (e.g., the view is being dismissed), call `cleanup()` to close the WebSocket connection and cancel background tasks:

```swift
sdk.cleanup()
```

For views managed by SwiftUI's `@StateObject`, this is typically called in `.onDisappear` or when the view is deallocated.

---