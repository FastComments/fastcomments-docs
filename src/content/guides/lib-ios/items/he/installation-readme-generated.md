הוסף את FastCommentsUI לפרויקט שלך באמצעות Swift Package Manager.

ב‑Xcode: **File > Add Package Dependencies**, ואז הזן את כתובת המאגר.

או הוסף אותו ל־`Package.swift`:

```swift
dependencies: [
    .package(url: "https://github.com/fastcomments/fastcomments-ios.git", from: "1.0.0")
]
```

לאחר מכן הוסף את המוצר ל־target שלך:

```swift
.target(
    name: "YourApp",
    dependencies: [
        .product(name: "FastCommentsUI", package: "fastcomments-ios")
    ]
)
```

ייבא את שני המודולים היכן שנדרש:

```swift
import FastCommentsUI
import FastCommentsSwift
```