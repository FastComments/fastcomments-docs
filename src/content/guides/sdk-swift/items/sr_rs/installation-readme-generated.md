### Swift Package Manager

Додајте следеће у ваш `Package.swift` фајл:

```swift
dependencies: [
    .package(url: "https://github.com/fastcomments/fastcomments-swift.git", from: "3.1.0")
]
```

Или у Xcode-у:
1. File > Add Packages...
2. Унесите URL репозиторију: `https://github.com/fastcomments/fastcomments-swift.git`
3. Изаберите верзију коју желите да користите

### Захтеви

- Swift 5.9+
- iOS 13.0+ / macOS 10.15+ / tvOS 13.0+ / watchOS 6.0+