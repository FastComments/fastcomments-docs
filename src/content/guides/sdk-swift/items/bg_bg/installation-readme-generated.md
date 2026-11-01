---
### Swift Package Manager

Добавете следното към вашия `Package.swift` файл:

```swift
dependencies: [
    .package(url: "https://github.com/fastcomments/fastcomments-swift.git", from: "3.1.1")
]
```

Или в Xcode:
1. File > Add Packages...
2. Въведете URL на хранилището: `https://github.com/fastcomments/fastcomments-swift.git`
3. Изберете версията, която искате да използвате

### Изисквания

- Swift 5.9+
- iOS 13.0+ / macOS 10.15+ / tvOS 13.0+ / watchOS 6.0+
---