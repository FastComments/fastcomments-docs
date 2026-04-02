### 評論工具列按鈕

實作 `CustomToolbarButton` 協定，以新增按鈕到評論輸入工具列：

```swift
struct EmojiButton: CustomToolbarButton {
    let id = "emoji"
    let iconSystemName = "face.smiling"       // SF Symbol 名稱
    let contentDescription = "Add Emoji"
    let badgeText: String? = nil              // 可選的徽章數量

    func onClick(text: Binding<String>) {
        text.wrappedValue += "\u{1F44D}"
    }

    // 可選的覆寫（預設為 true）
    func isEnabled() -> Bool { true }
    func isVisible() -> Bool { true }
}
```

在建立視圖時傳入自訂按鈕：

```swift
FastCommentsView(
    sdk: sdk,
    customToolbarButtons: [EmojiButton(), CodeBlockButton()]
)
```

或在 SDK 上全域新增（套用到所有實例）：

```swift
sdk.addGlobalCustomToolbarButton(EmojiButton())
sdk.removeGlobalCustomToolbarButton(id: "emoji")
sdk.clearGlobalCustomToolbarButtons()
```

### Feed 工具列按鈕

針對貼文建立表單，實作 `FeedCustomToolbarButton`：

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

傳遞它們到建立視圖：

```swift
FeedPostCreateView(
    sdk: sdk,
    customToolbarButtons: [HashtagButton()],
    onPostCreated: { _ in },
    onCancelled: { }
)
```

或在 feed SDK 上全域設定：

```swift
sdk.globalFeedToolbarButtons = [HashtagButton()]
```

---
---