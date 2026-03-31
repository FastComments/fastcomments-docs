Dodaj FastCommentsUI do swojego projektu przy użyciu Menedżera pakietów Swift (Swift Package Manager).

W Xcode: **File > Add Package Dependencies**, następnie wpisz URL repozytorium.

Albo dodaj go do swojego `Package.swift`:

```swift
dependencies: [
    .package(url: "https://github.com/fastcomments/fastcomments-ios.git", from: "1.0.0")
]
```

Następnie dodaj produkt do swojego targetu:

```swift
.target(
    name: "YourApp",
    dependencies: [
        .product(name: "FastCommentsUI", package: "fastcomments-ios")
    ]
)
```

Zaimportuj oba moduły w miejscach, gdzie są potrzebne:

```swift
import FastCommentsUI
import FastCommentsSwift
```