### Дугмад траке алата за коментаре

Имплементирајте протокол `CustomToolbarButton` да бисте додали дугмад у траку са алаткама за унос коментара:

```swift
struct EmojiButton: CustomToolbarButton {
    let id = "emoji"
    let iconSystemName = "face.smiling"       // Назив SF симбола
    let contentDescription = "Add Emoji"
    let badgeText: String? = nil              // Опционо: број на значки

    func onClick(text: Binding<String>) {
        text.wrappedValue += "\u{1F44D}"
    }

    // Optional overrides (default to true)
    func isEnabled() -> Bool { true }
    func isVisible() -> Bool { true }
}
```

Проследите прилагођена дугмад при креирању view-а:

```swift
FastCommentsView(
    sdk: sdk,
    customToolbarButtons: [EmojiButton(), CodeBlockButton()]
)
```

Или их додајте глобално на SDK (примењује се на све инстанце):

```swift
sdk.addGlobalCustomToolbarButton(EmojiButton())
sdk.removeGlobalCustomToolbarButton(id: "emoji")
sdk.clearGlobalCustomToolbarButtons()
```

### Дугмад траке алата за фид

Имплементирајте `FeedCustomToolbarButton` за форму за креирање објаве:

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

Проследите их у приказ за креирање:

```swift
FeedPostCreateView(
    sdk: sdk,
    customToolbarButtons: [HashtagButton()],
    onPostCreated: { _ in },
    onCancelled: { }
)
```

Или их поставите глобално у feed SDK:

```swift
sdk.globalFeedToolbarButtons = [HashtagButton()]
```

---
---