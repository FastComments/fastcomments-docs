FastCommentsUI'ı projenize Swift Package Manager kullanarak ekleyin.

Xcode'da: **File > Add Package Dependencies**, ardından depo URL'sini girin.

Ya da `Package.swift` dosyanıza ekleyin:

```swift
dependencies: [
    .package(url: "https://github.com/fastcomments/fastcomments-ios.git", from: "2.0.0")
]
```

Ardından ürünü hedefinize (target) ekleyin:

```swift
.target(
    name: "YourApp",
    dependencies: [
        .product(name: "FastCommentsUI", package: "fastcomments-ios")
    ]
)
```

Gerektiği yerde her iki modülü de içe aktarın:

```swift
import FastCommentsUI
import FastCommentsSwift
```