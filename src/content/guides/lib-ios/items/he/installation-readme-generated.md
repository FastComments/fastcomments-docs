הוסף את FastCommentsUI לפרויקט שלך באמצעות Swift Package Manager.

ב‑Xcode: **File > Add Package Dependencies**, ואז הזן את כתובת המאגרים.

או הוסף זאת לקובץ `Package.swift`:

```swift
dependencies: [
    .package(url: "https://github.com/fastcomments/fastcomments-ios.git", from: "2.0.0")
]
```

לאחר מכן הוסף את המוצר למטרה שלך:

```swift
.target(
    name: "YourApp",
    dependencies: [
        .product(name: "FastCommentsUI", package: "fastcomments-ios")
    ]
)
```

ייבא את שני המודולים במקומות הנדרשים:

```swift
import FastCommentsUI
import FastCommentsSwift
```