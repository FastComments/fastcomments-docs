### Werkbalkknoppen voor opmerkingen

Implementeer het `CustomToolbarButton`-protocol om knoppen toe te voegen aan de invoerwerkbalk voor opmerkingen:

```swift
struct EmojiButton: CustomToolbarButton {
    let id = "emoji"
    let iconSystemName = "face.smiling"       // SF-symboolnaam
    let contentDescription = "Add Emoji"
    let badgeText: String? = nil              // Optioneel badge-aantal

    func onClick(text: Binding<String>) {
        text.wrappedValue += "\u{1F44D}"
    }

    // Optionele overschrijvingen (standaard true)
    func isEnabled() -> Bool { true }
    func isVisible() -> Bool { true }
}
```

Geef aangepaste knoppen door bij het aanmaken van de view:

```swift
FastCommentsView(
    sdk: sdk,
    customToolbarButtons: [EmojiButton(), CodeBlockButton()]
)
```

Of voeg ze globaal toe aan de SDK (geldt voor alle instanties):

```swift
sdk.addGlobalCustomToolbarButton(EmojiButton())
sdk.removeGlobalCustomToolbarButton(id: "emoji")
sdk.clearGlobalCustomToolbarButtons()
```

### Werkbalkknoppen voor de feed

Implementeer `FeedCustomToolbarButton` voor het formulier om een bericht te maken:

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

Geef ze door aan de aanmaakweergave:

```swift
FeedPostCreateView(
    sdk: sdk,
    customToolbarButtons: [HashtagButton()],
    onPostCreated: { _ in },
    onCancelled: { }
)
```

Of stel ze globaal in op de feed SDK:

```swift
sdk.globalFeedToolbarButtons = [HashtagButton()]
```