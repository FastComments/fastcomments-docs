### Swift Package Manager

Додајте следеће у ваш `Package.swift` фајл:

```swift
dependencies: [
    .package(url: "https://github.com/fastcomments/fastcomments-swift.git", from: "1.2.1")
]
```

Или у Xcode-у:
1. Фајл > Додај пакете...
2. Унесите URL репозиторија: `https://github.com/fastcomments/fastcomments-swift.git`
3. Изаберите верзију коју желите користити

### Захтеви

- Swift 5.9+
- iOS 13.0+ / macOS 10.15+ / tvOS 13.0+ / watchOS 6.0+
---