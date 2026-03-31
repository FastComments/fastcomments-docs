### Comment Toolbar Buttons

Реализуйте протокол `CustomToolbarButton`, чтобы добавить кнопки в панель ввода комментария:

```swift
struct EmojiButton: CustomToolbarButton {
    let id = "emoji"
    let iconSystemName = "face.smiling"       // Имя SF Symbol
    let contentDescription = "Add Emoji"
    let badgeText: String? = nil              // Необязательный счётчик бейджа

    func onClick(text: Binding<String>) {
        text.wrappedValue += "\u{1F44D}"
    }

    // Необязательные переопределения (по умолчанию: true)
    func isEnabled() -> Bool { true }
    func isVisible() -> Bool { true }
}
```

Передайте пользовательские кнопки при создании представления:

```swift
FastCommentsView(
    sdk: sdk,
    customToolbarButtons: [EmojiButton(), CodeBlockButton()]
)
```

Или добавьте их глобально в SDK (применяется ко всем экземплярам):

```swift
sdk.addGlobalCustomToolbarButton(EmojiButton())
sdk.removeGlobalCustomToolbarButton(id: "emoji")
sdk.clearGlobalCustomToolbarButtons()
```

### Feed Toolbar Buttons

Реализуйте `FeedCustomToolbarButton` для формы создания поста:

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

Передайте их в представление создания:

```swift
FeedPostCreateView(
    sdk: sdk,
    customToolbarButtons: [HashtagButton()],
    onPostCreated: { _ in },
    onCancelled: { }
)
```

Или установите их глобально в SDK ленты:

```swift
sdk.globalFeedToolbarButtons = [HashtagButton()]
```

---
---