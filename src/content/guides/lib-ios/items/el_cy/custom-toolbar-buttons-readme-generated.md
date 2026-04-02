---
### Κουμπιά γραμμής εργαλείων σχολίων

Υλοποιήστε το `CustomToolbarButton` πρωτόκολλο για να προσθέσετε κουμπιά στη γραμμή εργαλείων εισαγωγής σχολίων:

```swift
struct EmojiButton: CustomToolbarButton {
    let id = "emoji"
    let iconSystemName = "face.smiling"       // Όνομα SF Symbol
    let contentDescription = "Add Emoji"
    let badgeText: String? = nil              // Προαιρετικός αριθμός ένδειξης

    func onClick(text: Binding<String>) {
        text.wrappedValue += "\u{1F44D}"
    }

    // Προαιρετικά (προεπιλεγμένο true)
    func isEnabled() -> Bool { true }
    func isVisible() -> Bool { true }
}
```

Περνάτε τα προσαρμοσμένα κουμπιά κατά τη δημιουργία της προβολής:

```swift
FastCommentsView(
    sdk: sdk,
    customToolbarButtons: [EmojiButton(), CodeBlockButton()]
)
```

Ή προσθέστε τα παγκοσμίως στο SDK (ισχύει για όλες τις περιπτώσεις):

```swift
sdk.addGlobalCustomToolbarButton(EmojiButton())
sdk.removeGlobalCustomToolbarButton(id: "emoji")
sdk.clearGlobalCustomToolbarButtons()
```

### Κουμπιά γραμμής εργαλείων ροής

Υλοποιήστε το `FeedCustomToolbarButton` για τη φόρμα δημιουργίας δημοσίευσης:

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

Περάστε τα στην προβολή δημιουργίας:

```swift
FeedPostCreateView(
    sdk: sdk,
    customToolbarButtons: [HashtagButton()],
    onPostCreated: { _ in },
    onCancelled: { }
)
```

Ή ορίστε τα παγκοσμίως στο SDK της ροής:

```swift
sdk.globalFeedToolbarButtons = [HashtagButton()]
```

---
---