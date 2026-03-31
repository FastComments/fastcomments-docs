### コメント入力ツールバーのボタン

コメント入力ツールバーにボタンを追加するには、`CustomToolbarButton`プロトコルを実装します:

```swift
struct EmojiButton: CustomToolbarButton {
    let id = "emoji"
    let iconSystemName = "face.smiling"       // SF シンボル名
    let contentDescription = "Add Emoji"
    let badgeText: String? = nil              // オプションのバッジ数

    func onClick(text: Binding<String>) {
        text.wrappedValue += "\u{1F44D}"
    }

    // オプションのオーバーライド（デフォルトは true）
    func isEnabled() -> Bool { true }
    func isVisible() -> Bool { true }
}
```

ビュー作成時にカスタムボタンを渡します:

```swift
FastCommentsView(
    sdk: sdk,
    customToolbarButtons: [EmojiButton(), CodeBlockButton()]
)
```

または SDK にグローバルに追加します（すべてのインスタンスに適用）:

```swift
sdk.addGlobalCustomToolbarButton(EmojiButton())
sdk.removeGlobalCustomToolbarButton(id: "emoji")
sdk.clearGlobalCustomToolbarButtons()
```

### フィードツールバーのボタン

投稿作成フォーム用に `FeedCustomToolbarButton` を実装します:

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

作成ビューに渡します:

```swift
FeedPostCreateView(
    sdk: sdk,
    customToolbarButtons: [HashtagButton()],
    onPostCreated: { _ in },
    onCancelled: { }
)
```

またはフィード SDK にグローバル設定します:

```swift
sdk.globalFeedToolbarButtons = [HashtagButton()]
```

---
---