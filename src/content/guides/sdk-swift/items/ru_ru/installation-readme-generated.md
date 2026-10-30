### Swift Package Manager

Add the following to your `Package.swift` file:

```swift
dependencies: [
    .package(url: "https://github.com/fastcomments/fastcomments-swift.git", from: "3.1.0")
]
```

Or in Xcode:
1. File > Добавить пакеты...
2. Введите URL репозитория: `https://github.com/fastcomments/fastcomments-swift.git`
3. Выберите версию, которую хотите использовать

### Требования

- Swift 5.9+
- iOS 13.0+ / macOS 10.15+ / tvOS 13.0+ / watchOS 6.0+