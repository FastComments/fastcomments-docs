### Swift Paket Yöneticisi

Aşağıdakileri `Package.swift` dosyanıza ekleyin:

```swift
dependencies: [
    .package(url: "https://github.com/fastcomments/fastcomments-swift.git", from: "0.0.1")
]
```

Veya Xcode'da:
1. File > Add Packages...
2. Depo URL'sini girin: `https://github.com/fastcomments/fastcomments-swift.git`
3. Kullanmak istediğiniz sürümü seçin

### Gereksinimler

- Swift 5.9+
- iOS 13.0+ / macOS 10.15+ / tvOS 13.0+ / watchOS 6.0+
---