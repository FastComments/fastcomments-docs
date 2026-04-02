Swift Package Manager를 사용하여 FastCommentsUI를 프로젝트에 추가하세요.

Xcode에서: **File > Add Package Dependencies**를 선택한 다음 저장소 URL을 입력하세요.

또는 `Package.swift`에 추가하세요:

```swift
dependencies: [
    .package(url: "https://github.com/fastcomments/fastcomments-ios.git", from: "1.0.0")
]
```

그런 다음 제품을 타깃에 추가하세요:

```swift
.target(
    name: "YourApp",
    dependencies: [
        .product(name: "FastCommentsUI", package: "fastcomments-ios")
    ]
)
```

필요한 곳에서 두 모듈을 임포트하세요:

```swift
import FastCommentsUI
import FastCommentsSwift
```