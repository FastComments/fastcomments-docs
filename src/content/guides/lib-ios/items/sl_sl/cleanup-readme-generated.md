Ko končate z instanco SDK (npr. ko se pogled zapira), pokličite `cleanup()`, da zaprete WebSocket povezavo in prekličete ozadna opravila:

```swift
sdk.cleanup()
```

Za poglede, ki jih upravlja `@StateObject` v SwiftUI, se to običajno kliče v `.onDisappear` ali ko se pogled dealocira.