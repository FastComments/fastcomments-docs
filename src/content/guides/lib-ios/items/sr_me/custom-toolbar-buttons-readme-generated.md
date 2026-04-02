### Дугмад траке алата за коментаре

Имплементирајте протокол `CustomToolbarButton` да бисте додали дугмад у траку алата за унос коментара:

```swift
struct EmojiButton: CustomToolbarButton {
    let id = "emoji"
    let iconSystemName = "face.smiling"       // Име SF симбола
    let contentDescription = "Add Emoji"
    let badgeText: String? = nil              // Опционални број ознаке

    func onClick(text: Binding<String>) {
        text.wrappedValue += "\u{1F44D}"
    }

    // Опционални методи за промену (подразумевано true)
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

Или их додајте глобално на SDK (важи за све инстанце):

```swift
sdk.addGlobalCustomToolbarButton(EmojiButton())
sdk.removeGlobalCustomToolbarButton(id: "emoji")
sdk.clearGlobalCustomToolbarButtons()
```

### Дугмад траке алата за фид

Имплементирајте `FeedCustomToolbarButton` за формулар за креирање поста:

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

Или их подесите глобално на SDK за фид:

```swift
sdk.globalFeedToolbarButtons = [HashtagButton()]
```

---