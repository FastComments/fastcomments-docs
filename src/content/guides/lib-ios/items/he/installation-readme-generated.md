הוסיפו את FastCommentsUI לפרויקט שלכם באמצעות Swift Package Manager.

ב-Xcode: **File > Add Package Dependencies**, ולאחר מכן הזינו את כתובת ה-URL של המאגר.

או הוסיפו אותו ל-`Package.swift` שלכם:

```swift
dependencies: [
    .package(url: "https://github.com/fastcomments/fastcomments-ios.git", from: "2.1.0")
]
```

לאחר מכן הוסיפו את המוצר ל-target שלכם:

```swift
.target(
    name: "YourApp",
    dependencies: [
        .product(name: "FastCommentsUI", package: "fastcomments-ios")
    ]
)
```

ייבאו את שני המודולים היכן שנדרש:

```swift
import FastCommentsUI
import FastCommentsSwift
```