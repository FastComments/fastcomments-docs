### Comment Toolbar Buttons

Implementirajte `CustomToolbarButton` protokol da biste dodali dugmad u traku za unos komentara:

```swift
struct EmojiButton: CustomToolbarButton {
    let id = "emoji"
    let iconSystemName = "face.smiling"       // Ime SF simbola
    let contentDescription = "Add Emoji"
    let badgeText: String? = nil              // Opcioni broj na bedžu

    func onClick(text: Binding<String>) {
        text.wrappedValue += "\u{1F44D}"
    }

    // Opcione override metode (podrazumevano: true)
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

Ili ih dodajte globalno na SDK (važi za sve instance):

```swift
sdk.addGlobalCustomToolbarButton(EmojiButton())
sdk.removeGlobalCustomToolbarButton(id: "emoji")
sdk.clearGlobalCustomToolbarButtons()
```

### Feed Toolbar Buttons

Implementirajte `FeedCustomToolbarButton` za formu za kreiranje objave:

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

Prosledite ih prikazu za kreiranje objave:

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