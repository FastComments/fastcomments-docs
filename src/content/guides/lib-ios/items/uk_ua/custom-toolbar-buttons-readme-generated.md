---
### Кнопки панелі інструментів коментаря

Реалізуйте протокол `CustomToolbarButton`, щоб додати кнопки до панелі введення коментаря:

```swift
struct EmojiButton: CustomToolbarButton {
    let id = "emoji"
    let iconSystemName = "face.smiling"       // назва SF Symbol
    let contentDescription = "Add Emoji"
    let badgeText: String? = nil              // Необов'язковий лічильник значка

    func onClick(text: Binding<String>) {
        text.wrappedValue += "\u{1F44D}"
    }

    // Необов'язкові перевизначення (за замовчуванням true)
    func isEnabled() -> Bool { true }
    func isVisible() -> Bool { true }
}
```

Передайте власні кнопки при створенні представлення:

```swift
FastCommentsView(
    sdk: sdk,
    customToolbarButtons: [EmojiButton(), CodeBlockButton()]
)
```

Або додайте їх глобально в SDK (застосовується до всіх екземплярів):

```swift
sdk.addGlobalCustomToolbarButton(EmojiButton())
sdk.removeGlobalCustomToolbarButton(id: "emoji")
sdk.clearGlobalCustomToolbarButtons()
```

### Кнопки панелі інструментів стрічки

Реалізуйте `FeedCustomToolbarButton` для форми створення допису:

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

Передайте їх у представлення створення:

```swift
FeedPostCreateView(
    sdk: sdk,
    customToolbarButtons: [HashtagButton()],
    onPostCreated: { _ in },
    onCancelled: { }
)
```

Або встановіть їх глобально в SDK стрічки:

```swift
sdk.globalFeedToolbarButtons = [HashtagButton()]
```

---