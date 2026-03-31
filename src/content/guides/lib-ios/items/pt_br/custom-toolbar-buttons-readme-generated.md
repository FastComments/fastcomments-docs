### Botões da Barra de Ferramentas de Comentários

Implemente o protocolo `CustomToolbarButton` para adicionar botões à barra de ferramentas do campo de entrada de comentários:

```swift
struct EmojiButton: CustomToolbarButton {
    let id = "emoji"
    let iconSystemName = "face.smiling"       // nome do SF Symbol
    let contentDescription = "Add Emoji"
    let badgeText: String? = nil              // Contador de badge opcional

    func onClick(text: Binding<String>) {
        text.wrappedValue += "\u{1F44D}"
    }

    // Sobrescritas opcionais (padrão true)
    func isEnabled() -> Bool { true }
    func isVisible() -> Bool { true }
}
```

Passe botões customizados ao criar a view:

```swift
FastCommentsView(
    sdk: sdk,
    customToolbarButtons: [EmojiButton(), CodeBlockButton()]
)
```

Ou adicione-os globalmente no SDK (aplica-se a todas as instâncias):

```swift
sdk.addGlobalCustomToolbarButton(EmojiButton())
sdk.removeGlobalCustomToolbarButton(id: "emoji")
sdk.clearGlobalCustomToolbarButtons()
```

### Botões da Barra de Ferramentas do Feed

Implemente `FeedCustomToolbarButton` para o formulário de criação de post:

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

Passe-os para a view de criação:

```swift
FeedPostCreateView(
    sdk: sdk,
    customToolbarButtons: [HashtagButton()],
    onPostCreated: { _ in },
    onCancelled: { }
)
```

Ou defina-os globalmente no SDK do feed:

```swift
sdk.globalFeedToolbarButtons = [HashtagButton()]
```

---
---