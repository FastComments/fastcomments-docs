### Gumbi orodne vrstice za komentarje

Implementirajte protokol `CustomToolbarButton`, da dodate gumbe v orodno vrstico za vnos komentarja:

```swift
struct EmojiButton: CustomToolbarButton {
    let id = "emoji"
    let iconSystemName = "face.smiling"       // Ime SF simbola
    let contentDescription = "Add Emoji"
    let badgeText: String? = nil              // Neobvezen števec značke

    func onClick(text: Binding<String>) {
        text.wrappedValue += "\u{1F44D}"
    }

    // Neobvezne preglasitve (privzeto true)
    func isEnabled() -> Bool { true }
    func isVisible() -> Bool { true }
}
```

Posredujte prilagojene gumbe pri ustvarjanju pogleda:

```swift
FastCommentsView(
    sdk: sdk,
    customToolbarButtons: [EmojiButton(), CodeBlockButton()]
)
```

Ali jih dodajte globalno v SDK (velja za vse primere):

```swift
sdk.addGlobalCustomToolbarButton(EmojiButton())
sdk.removeGlobalCustomToolbarButton(id: "emoji")
sdk.clearGlobalCustomToolbarButtons()
```

### Gumbi orodne vrstice vira

Implementirajte `FeedCustomToolbarButton` za obrazec za ustvarjanje objave:

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

Posredujte jih v pogled za ustvarjanje objave:

```swift
FeedPostCreateView(
    sdk: sdk,
    customToolbarButtons: [HashtagButton()],
    onPostCreated: { _ in },
    onCancelled: { }
)
```

Ali jih nastavite globalno v SDK vira:

```swift
sdk.globalFeedToolbarButtons = [HashtagButton()]
```

---
---