### Botones de la barra de herramientas de comentarios

Implemente el protocolo `CustomToolbarButton` para agregar botones a la barra de herramientas del campo de entrada de comentarios:

```swift
struct EmojiButton: CustomToolbarButton {
    let id = "emoji"
    let iconSystemName = "face.smiling"       // Nombre del SF Symbol
    let contentDescription = "Add Emoji"
    let badgeText: String? = nil              // Conteo de insignia opcional

    func onClick(text: Binding<String>) {
        text.wrappedValue += "\u{1F44D}"
    }

    // Sobrescrituras opcionales (predeterminado true)
    func isEnabled() -> Bool { true }
    func isVisible() -> Bool { true }
}
```

Pase los botones personalizados al crear la vista:

```swift
FastCommentsView(
    sdk: sdk,
    customToolbarButtons: [EmojiButton(), CodeBlockButton()]
)
```

O agréguelos globalmente al SDK (se aplica a todas las instancias):

```swift
sdk.addGlobalCustomToolbarButton(EmojiButton())
sdk.removeGlobalCustomToolbarButton(id: "emoji")
sdk.clearGlobalCustomToolbarButtons()
```

### Botones de la barra de herramientas del feed

Implemente `FeedCustomToolbarButton` para el formulario de creación de publicaciones:

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

Páselos a la vista de creación:

```swift
FeedPostCreateView(
    sdk: sdk,
    customToolbarButtons: [HashtagButton()],
    onPostCreated: { _ in },
    onCancelled: { }
)
```

O configúrelos globalmente en el SDK del feed:

```swift
sdk.globalFeedToolbarButtons = [HashtagButton()]
```

---
---