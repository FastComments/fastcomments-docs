Kada završite sa instancom SDK-a (npr. kada se prikaz odbacuje), pozovite `cleanup()` da zatvorite WebSocket konekciju i otkažete pozadinske zadatke:

```swift
sdk.cleanup()
```

Za poglede koje upravlja SwiftUI-jev `@StateObject`, ovo se obično poziva u `.onDisappear` ili kada se prikaz dealocira.