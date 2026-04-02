### Gumbi alatne trake za komentare

Implementirajte protokol `CustomToolbarButton` kako biste dodali gumbe u alatnu traku za unos komentara:

```swift
struct EmojiButton: CustomToolbarButton {
    let id = "emoji"
    let iconSystemName = "face.smiling"       // Naziv SF simbola
    let contentDescription = "Add Emoji"
    let badgeText: String? = nil              // Neobavezni broj značke

    func onClick(text: Binding<String>) {
        text.wrappedValue += "\u{1F44D}"
    }

    // Neobavezni overridei (zadano true)
    func isEnabled() -> Bool { true }
    func isVisible() -> Bool { true }
}
```

Proslijedite prilagođene gumbe prilikom stvaranja prikaza:

```swift
FastCommentsView(
    sdk: sdk,
    customToolbarButtons: [EmojiButton(), CodeBlockButton()]
)
```

Ili ih dodajte globalno na SDK (primjenjuje se na sve instance):

```swift
sdk.addGlobalCustomToolbarButton(EmojiButton())
sdk.removeGlobalCustomToolbarButton(id: "emoji")
sdk.clearGlobalCustomToolbarButtons()
```

### Gumbi alatne trake feeda

Implementirajte `FeedCustomToolbarButton` za obrazac za stvaranje objava:

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

Proslijedite ih prikazu za stvaranje:

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