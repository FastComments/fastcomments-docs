### Swift Package Manager

Додайте наступне до вашого файлу `Package.swift`:

```swift
dependencies: [
    .package(url: "https://github.com/fastcomments/fastcomments-swift.git", from: "0.0.1")
]
```

Або в Xcode:
1. File > Add Packages...
2. Enter the repository URL: `https://github.com/fastcomments/fastcomments-swift.git`
3. Select the version you want to use

### Вимоги

- Swift 5.9+
- iOS 13.0+ / macOS 10.15+ / tvOS 13.0+ / watchOS 6.0+