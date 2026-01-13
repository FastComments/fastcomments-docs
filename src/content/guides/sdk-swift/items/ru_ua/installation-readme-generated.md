### Менеджер пакетов Swift

Добавьте следующее в файл `Package.swift`:

```swift
dependencies: [
    .package(url: "https://github.com/fastcomments/fastcomments-swift.git", from: "0.0.1")
]
```

Или в Xcode:
1. Файл > Добавить пакеты...
2. Введите URL репозитория: `https://github.com/fastcomments/fastcomments-swift.git`
3. Выберите версию, которую хотите использовать

### Требования

- Swift 5.9+
- iOS 13.0+ / macOS 10.15+ / tvOS 13.0+ / watchOS 6.0+