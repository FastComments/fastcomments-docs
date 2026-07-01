### Swift Package Manager

Додайте наступне у ваш `Package.swift` файл:

```swift
dependencies: [
    .package(url: "https://github.com/fastcomments/fastcomments-swift.git", from: "3.0.0")
]
```

Або в Xcode:
1. Файл > Додати пакети...
2. Введіть URL репозиторію: `https://github.com/fastcomments/fastcomments-swift.git`
3. Виберіть версію, яку ви хочете використати

### Вимоги

- Swift 5.9+
- iOS 13.0+ / macOS 10.15+ / tvOS 13.0+ / watchOS 6.0+