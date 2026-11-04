### Dugmad trake sa alatima za komentare

Implementirajte protokol `CustomToolbarButton` da biste dodali dugmad na traku sa alatima za unos komentara:

```swift
struct EmojiButton: CustomToolbarButton {
    let id = "emoji"
    let iconSystemName = "face.smiling"       // SF Symbol name
    let contentDescription = "Add Emoji"
    let badgeText: String? = nil              // Optional badge count

    func onClick(text: Binding<String>) {
        text.wrappedValue += "\u{1F44D}"
    }

    // Optional overrides (default to true)
    func isEnabled() -> Bool { true }
    func isVisible() -> Bool { true }
}
```

Prosledite prilagođena dugmad prilikom kreiranja prikaza:

```swift
FastCommentsView(
    sdk: sdk,
    customToolbarButtons: [EmojiButton(), CodeBlockButton()]
)
```

Ili ih dodajte globalno na SDK (primenjuje se na sve instance):

```swift
sdk.addGlobalCustomToolbarButton(EmojiButton())
sdk.removeGlobalCustomToolbarButton(id: "emoji")
sdk.clearGlobalCustomToolbarButtons()
```

### Dugmad trake sa alatima za feed

Implementirajte `FeedCustomToolbarButton` za formu kreiranja objave:

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

Prosledite ih prikazu za kreiranje:

```swift
FeedPostCreateView(
    sdk: sdk,
    customToolbarButtons: [HashtagButton()],
    onPostCreated: { _ in },
    onCancelled: { }
)
```

Ili ih postavite globalno na feed SDK:

```swift
sdk.globalFeedToolbarButtons = [HashtagButton()]
```

---
---