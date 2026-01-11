### Swift Package Manager

Додајте следеће у ваш фајл `Package.swift`:

```swift
dependencies: [
    .package(url: "https://github.com/fastcomments/fastcomments-swift.git", from: "0.0.1")
]
```

Или у Xcode:
1. File > Add Packages...
2. Унесите URL репозиторија: `https://github.com/fastcomments/fastcomments-swift.git`
3. Изаберите верзију коју желите користити

### Захтеви

- Swift 5.9+
- iOS 13.0+ / macOS 10.15+ / tvOS 13.0+ / watchOS 6.0+