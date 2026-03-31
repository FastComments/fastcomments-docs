---
Kad završite s instancom SDK-a (npr. pogled se zatvara), pozovite `cleanup()` da zatvorite WebSocket vezu i otkažete pozadinske zadatke:

```swift
sdk.cleanup()
```

Za poglede kojima upravlja SwiftUI-ov `@StateObject`, ovo se obično poziva u `.onDisappear` ili kada je pogled dealociran.

---
---