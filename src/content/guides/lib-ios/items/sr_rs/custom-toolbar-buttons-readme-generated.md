### Дугмад траке за коментаре

Имплементирајте протокол `CustomToolbarButton` да бисте додали дугмад у траку за унос коментара:

```swift
struct EmojiButton: CustomToolbarButton {
    let id = "emoji"
    let iconSystemName = "face.smiling"       // Назив SF симбола
    let contentDescription = "Add Emoji"
    let badgeText: String? = nil              // Опционални број значке

    func onClick(text: Binding<String>) {
        text.wrappedValue += "\u{1F44D}"
    }

    // Опционална преписивања (подразумевано true)
    func isEnabled() -> Bool { true }
    func isVisible() -> Bool { true }
}
```

Проследите прилагођена дугмад приликом креирања приказа:

```swift
FastCommentsView(
    sdk: sdk,
    customToolbarButtons: [EmojiButton(), CodeBlockButton()]
)
```

Или их додате глобално у SDK (важи за све инстанце):

```swift
sdk.addGlobalCustomToolbarButton(EmojiButton())
sdk.removeGlobalCustomToolbarButton(id: "emoji")
sdk.clearGlobalCustomToolbarButtons()
```

### Дугмад траке фида

Имплементирајте `FeedCustomToolbarButton` за формулар креирања објаве:

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

Проследите их приказу за креирање:

```swift
FeedPostCreateView(
    sdk: sdk,
    customToolbarButtons: [HashtagButton()],
    onPostCreated: { _ in },
    onCancelled: { }
)
```

Или их поставите глобално на feed SDK:

```swift
sdk.globalFeedToolbarButtons = [HashtagButton()]
```