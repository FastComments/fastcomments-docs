Προσθέστε το FastCommentsUI στο project σας χρησιμοποιώντας το Swift Package Manager.

Στο Xcode: **File > Add Package Dependencies**, και μετά εισάγετε το URL του repository.

Ή προσθέστε το στο `Package.swift` σας:

```swift
dependencies: [
    .package(url: "https://github.com/fastcomments/fastcomments-ios.git", from: "2.1.0")
]
```

Στη συνέχεια προσθέστε το product στο target σας:

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