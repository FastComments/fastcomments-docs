Προσθέστε το FastCommentsUI στο έργο σας χρησιμοποιώντας το Swift Package Manager.

Στο Xcode: **File > Add Package Dependencies**, στη συνέχεια εισάγετε το URL του αποθετηρίου.

Ή προσθέστε το στο `Package.swift` σας:

```swift
dependencies: [
    .package(url: "https://github.com/fastcomments/fastcomments-ios.git", from: "1.0.0")
]
```

Στη συνέχεια προσθέστε το προϊόν στο target σας:

```swift
.target(
    name: "YourApp",
    dependencies: [
        .product(name: "FastCommentsUI", package: "fastcomments-ios")
    ]
)
```

Εισάγετε και τα δύο modules όπου χρειάζεται:

```swift
import FastCommentsUI
import FastCommentsSwift
```