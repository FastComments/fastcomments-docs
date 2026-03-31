### Schaltflächen der Kommentar-Symbolleiste

Implementieren Sie das Protokoll `CustomToolbarButton`, um Schaltflächen zur Kommentar-Eingabesymbolleiste hinzuzufügen:

```swift
struct EmojiButton: CustomToolbarButton {
    let id = "emoji"
    let iconSystemName = "face.smiling"       // Name des SF-Symbols
    let contentDescription = "Add Emoji"
    let badgeText: String? = nil              // Optionale Badge-Anzahl

    func onClick(text: Binding<String>) {
        text.wrappedValue += "\u{1F44D}"
    }

    // Optionale Überschreibungen (Standard: true)
    func isEnabled() -> Bool { true }
    func isVisible() -> Bool { true }
}
```

Übergeben Sie benutzerdefinierte Schaltflächen beim Erstellen der Ansicht:

```swift
FastCommentsView(
    sdk: sdk,
    customToolbarButtons: [EmojiButton(), CodeBlockButton()]
)
```

Oder fügen Sie sie global im SDK hinzu (gilt für alle Instanzen):

```swift
sdk.addGlobalCustomToolbarButton(EmojiButton())
sdk.removeGlobalCustomToolbarButton(id: "emoji")
sdk.clearGlobalCustomToolbarButtons()
```

### Schaltflächen der Feed-Symbolleiste

Implementieren Sie `FeedCustomToolbarButton` für das Beitragserstellungsformular:

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

Übergeben Sie sie der Erstellungsansicht:

```swift
FeedPostCreateView(
    sdk: sdk,
    customToolbarButtons: [HashtagButton()],
    onPostCreated: { _ in },
    onCancelled: { }
)
```

Oder setzen Sie sie global im Feed-SDK:

```swift
sdk.globalFeedToolbarButtons = [HashtagButton()]
```

---