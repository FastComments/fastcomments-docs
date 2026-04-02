### 评论工具栏按钮

实现 `CustomToolbarButton` 协议以向评论输入工具栏添加按钮：

```swift
struct EmojiButton: CustomToolbarButton {
    let id = "emoji"
    let iconSystemName = "face.smiling"       // SF Symbol 名称
    let contentDescription = "Add Emoji"
    let badgeText: String? = nil              // 可选的徽章计数

    func onClick(text: Binding<String>) {
        text.wrappedValue += "\u{1F44D}"
    }

    // 可选覆盖（默认为 true）
    func isEnabled() -> Bool { true }
    func isVisible() -> Bool { true }
}
```

在创建视图时传入自定义按钮：

```swift
FastCommentsView(
    sdk: sdk,
    customToolbarButtons: [EmojiButton(), CodeBlockButton()]
)
```

或者将它们全局添加到 SDK（适用于所有实例）：

```swift
sdk.addGlobalCustomToolbarButton(EmojiButton())
sdk.removeGlobalCustomToolbarButton(id: "emoji")
sdk.clearGlobalCustomToolbarButtons()
```

### Feed 工具栏按钮

为帖子创建表单实现 `FeedCustomToolbarButton`：

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

将它们传递给创建视图：

```swift
FeedPostCreateView(
    sdk: sdk,
    customToolbarButtons: [HashtagButton()],
    onPostCreated: { _ in },
    onCancelled: { }
)
```

或者在 feed SDK 上全局设置它们：

```swift
sdk.globalFeedToolbarButtons = [HashtagButton()]
```