Add FastCommentsUI를 Swift Package Manager를 사용하여 프로젝트에 추가하십시오.

Xcode에서: **File > Add Package Dependencies**, 그런 다음 저장소 URL을 입력하십시오.

`Package.swift`에 추가하거나:

```swift
dependencies: [
    .package(url: "https://github.com/fastcomments/fastcomments-ios.git", from: "2.0.0")
]
```

그런 다음 제품을 대상에 추가하십시오:

```swift
.target(
    name: "YourApp",
    dependencies: [
        .product(name: "FastCommentsUI", package: "fastcomments-ios")
    ]
)
```

필요한 곳에 두 모듈을 모두 import하십시오:

```swift
import FastCommentsUI
import FastCommentsSwift
```