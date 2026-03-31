### Przyciski paska narzędzi komentarza

Zaimplementuj protokół `CustomToolbarButton`, aby dodać przyciski do paska narzędzi pola wprowadzania komentarza:

```swift
struct EmojiButton: CustomToolbarButton {
    let id = "emoji"
    let iconSystemName = "face.smiling"       // nazwa symbolu SF
    let contentDescription = "Add Emoji"
    let badgeText: String? = nil              // Opcjonalny licznik odznaki

    func onClick(text: Binding<String>) {
        text.wrappedValue += "\u{1F44D}"
    }

    // Opcjonalne nadpisania (domyślnie true)
    func isEnabled() -> Bool { true }
    func isVisible() -> Bool { true }
}
```

Przekaż niestandardowe przyciski podczas tworzenia widoku:

```swift
FastCommentsView(
    sdk: sdk,
    customToolbarButtons: [EmojiButton(), CodeBlockButton()]
)
```

Lub dodaj je globalnie do SDK (ma zastosowanie do wszystkich instancji):

```swift
sdk.addGlobalCustomToolbarButton(EmojiButton())
sdk.removeGlobalCustomToolbarButton(id: "emoji")
sdk.clearGlobalCustomToolbarButtons()
```

### Przyciski paska narzędzi kanału

Zaimplementuj `FeedCustomToolbarButton` dla formularza tworzenia posta:

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

Przekaż je do widoku tworzenia:

```swift
FeedPostCreateView(
    sdk: sdk,
    customToolbarButtons: [HashtagButton()],
    onPostCreated: { _ in },
    onCancelled: { }
)
```

Lub ustaw je globalnie w SDK kanału:

```swift
sdk.globalFeedToolbarButtons = [HashtagButton()]
```

---
---