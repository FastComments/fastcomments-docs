---
FastCommentsUI'yi Swift Package Manager kullanarak projenize ekleyin.

Xcode'da: **File > Add Package Dependencies**, ardından depo URL'sini girin.

Veya bunu `Package.swift`'inize ekleyin:

```swift
dependencies: [
    .package(url: "https://github.com/fastcomments/fastcomments-ios.git", from: "1.0.0")
]
```

Daha sonra ürünü hedefinize ekleyin:

```swift
.target(
    name: "YourApp",
    dependencies: [
        .product(name: "FastCommentsUI", package: "fastcomments-ios")
    ]
)
```

Gerekli yerlerde her iki modülü içe aktarın:

```swift
import FastCommentsUI
import FastCommentsSwift
```
---