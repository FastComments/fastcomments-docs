### Comment Toolbar Buttons

댓글 입력 툴바에 버튼을 추가하려면 `CustomToolbarButton` 프로토콜을 구현하세요:

```swift
struct EmojiButton: CustomToolbarButton {
    let id = "emoji"
    let iconSystemName = "face.smiling"       // SF 심볼 이름
    let contentDescription = "Add Emoji"
    let badgeText: String? = nil              // 선택적 배지 카운트

    func onClick(text: Binding<String>) {
        text.wrappedValue += "\u{1F44D}"
    }

    // Optional overrides (default to true)
    func isEnabled() -> Bool { true }
    func isVisible() -> Bool { true }
}
```

뷰를 생성할 때 커스텀 버튼을 전달하세요:

```swift
FastCommentsView(
    sdk: sdk,
    customToolbarButtons: [EmojiButton(), CodeBlockButton()]
)
```

또는 SDK에 전역으로 추가하여 모든 인스턴스에 적용할 수 있습니다:

```swift
sdk.addGlobalCustomToolbarButton(EmojiButton())
sdk.removeGlobalCustomToolbarButton(id: "emoji")
sdk.clearGlobalCustomToolbarButtons()
```

### Feed Toolbar Buttons

게시물 생성 폼에 대해 `FeedCustomToolbarButton`을 구현하세요:

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

생성 뷰에 전달하세요:

```swift
FeedPostCreateView(
    sdk: sdk,
    customToolbarButtons: [HashtagButton()],
    onPostCreated: { _ in },
    onCancelled: { }
)
```

또는 피드 SDK에서 전역으로 설정하세요:

```swift
sdk.globalFeedToolbarButtons = [HashtagButton()]
```

---
---