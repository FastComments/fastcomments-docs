### Swift Package Manager

Добавете следното в файла `Package.swift`:

```swift
dependencies: [
    .package(url: "https://github.com/fastcomments/fastcomments-swift.git", from: "0.0.1")
]
```

Или в Xcode:
1. Файл > Добавяне на пакети...
2. Въведете URL на хранилището: `https://github.com/fastcomments/fastcomments-swift.git`
3. Изберете версията, която искате да използвате

### Изисквания

- Swift 5.9+
- iOS 13.0+ / macOS 10.15+ / tvOS 13.0+ / watchOS 6.0+