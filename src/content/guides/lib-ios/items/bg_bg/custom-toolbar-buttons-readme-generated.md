### Бутони в лентата с инструменти за коментари

Имплементирайте протокола `CustomToolbarButton`, за да добавите бутони в лентата с инструменти за въвеждане на коментари:

```swift
struct EmojiButton: CustomToolbarButton {
    let id = "emoji"
    let iconSystemName = "face.smiling"       // Име на SF Symbol
    let contentDescription = "Add Emoji"
    let badgeText: String? = nil              // Опционален брой за значка

    func onClick(text: Binding<String>) {
        text.wrappedValue += "\u{1F44D}"
    }

    // Optional overrides (default to true)
    func isEnabled() -> Bool { true }
    func isVisible() -> Bool { true }
}
```

Подайте персонализирани бутони при създаване на изгледа:

```swift
FastCommentsView(
    sdk: sdk,
    customToolbarButtons: [EmojiButton(), CodeBlockButton()]
)
```

Или ги добавете глобално в SDK (важи за всички инстанции):

```swift
sdk.addGlobalCustomToolbarButton(EmojiButton())
sdk.removeGlobalCustomToolbarButton(id: "emoji")
sdk.clearGlobalCustomToolbarButtons()
```

### Бутони в лентата с инструменти за емисията

Имплементирайте `FeedCustomToolbarButton` за формата за създаване на публикация:

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

Подайте ги в изгледа за създаване:

```swift
FeedPostCreateView(
    sdk: sdk,
    customToolbarButtons: [HashtagButton()],
    onPostCreated: { _ in },
    onCancelled: { }
)
```

Или ги задайте глобално в feed SDK:

```swift
sdk.globalFeedToolbarButtons = [HashtagButton()]
```

---