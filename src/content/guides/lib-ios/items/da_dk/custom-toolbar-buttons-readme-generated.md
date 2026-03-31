### Kommentarværktøjslinje-knapper

Implementer `CustomToolbarButton`-protokollen for at tilføje knapper til kommentarfeltets værktøjslinje:

```swift
struct EmojiButton: CustomToolbarButton {
    let id = "emoji"
    let iconSystemName = "face.smiling"       // SF Symbol-navn
    let contentDescription = "Add Emoji"
    let badgeText: String? = nil              // Valgfri badge-tæller

    func onClick(text: Binding<String>) {
        text.wrappedValue += "\u{1F44D}"
    }

    // Valgfrie overrides (standard er true)
    func isEnabled() -> Bool { true }
    func isVisible() -> Bool { true }
}
```

Videregiv brugerdefinerede knapper, når du opretter visningen:

```swift
FastCommentsView(
    sdk: sdk,
    customToolbarButtons: [EmojiButton(), CodeBlockButton()]
)
```

Eller tilføj dem globalt på SDK'et (gælder for alle forekomster):

```swift
sdk.addGlobalCustomToolbarButton(EmojiButton())
sdk.removeGlobalCustomToolbarButton(id: "emoji")
sdk.clearGlobalCustomToolbarButtons()
```

### Feed-værktøjslinjeknapper

Implementer `FeedCustomToolbarButton` til indlægsoprettelsesformularen:

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

Videregiv dem til oprettelsesvisningen:

```swift
FeedPostCreateView(
    sdk: sdk,
    customToolbarButtons: [HashtagButton()],
    onPostCreated: { _ in },
    onCancelled: { }
)
```

Eller indstil dem globalt på feed-SDK'et:

```swift
sdk.globalFeedToolbarButtons = [HashtagButton()]
```

---
---