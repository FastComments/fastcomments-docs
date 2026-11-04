### 댓글 툴바 버튼

`CustomToolbarButton` 프로토콜을 구현하여 댓글 입력 툴바에 버튼을 추가합니다:

```swift
struct EmojiButton: CustomToolbarButton {
    let id = "emoji"
    let iconSystemName = "face.smiling"       // SF Symbol 이름
    let contentDescription = "Add Emoji"
    let badgeText: String? = nil              // 선택적 배지 개수

    func onClick(text: Binding<String>) {
        text.wrappedValue += "\u{1F44D}"
    }

    // 선택적 오버라이드 (기본값은 true)
    func isEnabled() -> Bool { true }
    func isVisible() -> Bool { true }
}
```

뷰를 생성할 때 커스텀 버튼을 전달합니다:

```swift
FastCommentsView(
    sdk: sdk,
    customToolbarButtons: [EmojiButton(), CodeBlockButton()]
)
```

또는 SDK에 전역으로 추가합니다(모든 인스턴스에 적용됨):

```swift
sdk.addGlobalCustomToolbarButton(EmojiButton())
sdk.removeGlobalCustomToolbarButton(id: "emoji")
sdk.clearGlobalCustomToolbarButtons()
```

### 피드 툴바 버튼

게시물 작성 폼에는 `FeedCustomToolbarButton`을 구현합니다:

```swift
struct HashtagButton: FeedCustomToolbarButton {
    let id = "hashtag"
    let iconSystemName = "number"
    let contentDescription = "Add Hashtag"

    func onClick(content: Binding<String>) {
        content.wrappedValue += "#"
    }
}
```

작성 뷰에 버튼을 전달합니다:

```swift
FeedPostCreateView(
    sdk: sdk,
    customToolbarButtons: [HashtagButton()],
    onPostCreated: { _ in },
    onCancelled: { }
)
```

또는 피드 SDK에 전역으로 설정합니다:

```swift
sdk.globalFeedToolbarButtons = [HashtagButton()]
```

---
---