Dodaj FastCommentsUI do swojego projektu używając Swift Package Manager.

W Xcode: **File > Add Package Dependencies**, a następnie wprowadź adres URL repozytorium.

Lub dodaj go do swojego `Package.swift`:

```swift
dependencies: [
    .package(url: "https://github.com/fastcomments/fastcomments-ios.git", from: "2.0.0")
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

Zaimportuj oba moduły w potrzebnych miejscach:

```swift
import FastCommentsUI
import FastCommentsSwift
```