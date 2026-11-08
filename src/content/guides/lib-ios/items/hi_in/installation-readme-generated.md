Swift Package Manager का उपयोग करके अपने प्रोजेक्ट में FastCommentsUI जोड़ें।

Xcode में: **File > Add Package Dependencies**, फिर रिपॉज़िटरी URL दर्ज करें।

या इसे अपने `Package.swift` में जोड़ें:

```swift
dependencies: [
    .package(url: "https://github.com/fastcomments/fastcomments-ios.git", from: "2.0.0")
]
```

फिर अपने टारगेट में प्रोडक्ट जोड़ें:

```swift
.target(
    name: "YourApp",
    dependencies: [
        .product(name: "FastCommentsUI", package: "fastcomments-ios")
    ]
)
```

जहाँ आवश्यक हो, दोनों मॉड्यूल इम्पोर्ट करें:

```swift
import FastCommentsUI
import FastCommentsSwift
```