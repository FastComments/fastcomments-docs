### Yorum Araç Çubuğu Düğmeleri

Yorum giriş araç çubuğuna düğmeler eklemek için `CustomToolbarButton` protokolünü uygulayın:

```swift
struct EmojiButton: CustomToolbarButton {
    let id = "emoji"
    let iconSystemName = "face.smiling"       // SF Symbol name
    let contentDescription = "Add Emoji"
    let badgeText: String? = nil              // İsteğe bağlı rozet sayısı

    func onClick(text: Binding<String>) {
        text.wrappedValue += "\u{1F44D}"
    }

    // Optional overrides (default to true)
    func isEnabled() -> Bool { true }
    func isVisible() -> Bool { true }
}
```

Görünümü oluştururken özel düğmeleri iletin:

```swift
FastCommentsView(
    sdk: sdk,
    customToolbarButtons: [EmojiButton(), CodeBlockButton()]
)
```

Veya SDK üzerinde küresel olarak ekleyin (tüm örneklere uygulanır):

```swift
sdk.addGlobalCustomToolbarButton(EmojiButton())
sdk.removeGlobalCustomToolbarButton(id: "emoji")
sdk.clearGlobalCustomToolbarButtons()
```

### Feed Araç Çubuğu Düğmeleri

Gönderi oluşturma formu için `FeedCustomToolbarButton`'ı uygulayın:

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

Oluşturma görünümüne iletin:

```swift
FeedPostCreateView(
    sdk: sdk,
    customToolbarButtons: [HashtagButton()],
    onPostCreated: { _ in },
    onCancelled: { }
)
```

Veya feed SDK üzerinde bunları küresel olarak ayarlayın:

```swift
sdk.globalFeedToolbarButtons = [HashtagButton()]
```

---
---