Προσθέστε το FastCommentsUI στο έργο σας χρησιμοποιώντας το Swift Package Manager.

Στο Xcode: **File > Add Package Dependencies**, στη συνέχεια εισαγάγετε το URL του αποθετηρίου.

Ή προσθέστε το στο `Package.swift` σας:

```swift
dependencies: [
    .package(url: "https://github.com/fastcomments/fastcomments-ios.git", from: "2.0.0")
]
```

Στη συνέχεια προσθέστε το προϊόν στον στόχο σας:

```swift
.target(
    name: "YourApp",
    dependencies: [
        .product(name: "FastCommentsUI", package: "fastcomments-ios")
    ]
)
```

Εισάγετε και τα δύο modules όπου απαιτείται:

```swift
import FastCommentsUI
import FastCommentsSwift
```