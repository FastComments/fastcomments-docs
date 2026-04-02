### Boutons de la barre d'outils des commentaires

Implémentez le protocole `CustomToolbarButton` pour ajouter des boutons à la barre d'outils de saisie des commentaires :

```swift
struct EmojiButton: CustomToolbarButton {
    let id = "emoji"
    let iconSystemName = "face.smiling"       // Nom du SF Symbol
    let contentDescription = "Add Emoji"
    let badgeText: String? = nil              // Nombre de badge optionnel

    func onClick(text: Binding<String>) {
        text.wrappedValue += "\u{1F44D}"
    }

    // Remplacements optionnels (par défaut true)
    func isEnabled() -> Bool { true }
    func isVisible() -> Bool { true }
}
```

Passez des boutons personnalisés lors de la création de la vue :

```swift
FastCommentsView(
    sdk: sdk,
    customToolbarButtons: [EmojiButton(), CodeBlockButton()]
)
```

Ou ajoutez-les globalement sur le SDK (s'applique à toutes les instances) :

```swift
sdk.addGlobalCustomToolbarButton(EmojiButton())
sdk.removeGlobalCustomToolbarButton(id: "emoji")
sdk.clearGlobalCustomToolbarButtons()
```

### Boutons de la barre d'outils du fil d'actualité

Implémentez `FeedCustomToolbarButton` pour le formulaire de création de publication :

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

Passez-les à la vue de création :

```swift
FeedPostCreateView(
    sdk: sdk,
    customToolbarButtons: [HashtagButton()],
    onPostCreated: { _ in },
    onCancelled: { }
)
```

Ou configurez-les globalement sur le SDK du fil :

```swift
sdk.globalFeedToolbarButtons = [HashtagButton()]
```

---
---