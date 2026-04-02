### Pulsanti della barra degli strumenti dei commenti

Implementa il protocollo `CustomToolbarButton` per aggiungere pulsanti alla barra degli strumenti per l'inserimento dei commenti:

```swift
struct EmojiButton: CustomToolbarButton {
    let id = "emoji"
    let iconSystemName = "face.smiling"       // Nome SF Symbol
    let contentDescription = "Add Emoji"
    let badgeText: String? = nil              // Conteggio badge opzionale

    func onClick(text: Binding<String>) {
        text.wrappedValue += "\u{1F44D}"
    }

    // Override opzionali (di default true)
    func isEnabled() -> Bool { true }
    func isVisible() -> Bool { true }
}
```

Passa pulsanti personalizzati quando crei la view:

```swift
FastCommentsView(
    sdk: sdk,
    customToolbarButtons: [EmojiButton(), CodeBlockButton()]
)
```

Oppure aggiungili globalmente allo SDK (si applica a tutte le istanze):

```swift
sdk.addGlobalCustomToolbarButton(EmojiButton())
sdk.removeGlobalCustomToolbarButton(id: "emoji")
sdk.clearGlobalCustomToolbarButtons()
```

### Pulsanti della barra degli strumenti del feed

Implementa `FeedCustomToolbarButton` per il form di creazione del post:

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

Passali alla view di creazione:

```swift
FeedPostCreateView(
    sdk: sdk,
    customToolbarButtons: [HashtagButton()],
    onPostCreated: { _ in },
    onCancelled: { }
)
```

Oppure impostali globalmente sull'SDK del feed:

```swift
sdk.globalFeedToolbarButtons = [HashtagButton()]
```