### Comment Toolbar Buttons

Implement the `CustomToolbarButton` protocol to add buttons to the comment input toolbar:

```swift
struct EmojiButton: CustomToolbarButton {
    let id = "emoji"
    let iconSystemName = "face.smiling"       // SF Symbol name
    let contentDescription = "Add Emoji"
    let badgeText: String? = nil              // Optional badge count

    func onClick(text: Binding<String>) {
        text.wrappedValue += "\u{1F44D}"
    }

    // Optional overrides (default to true)
    func isEnabled() -> Bool { true }
    func isVisible() -> Bool { true }
}
```

Pass custom buttons when creating the view:

```swift
FastCommentsView(
    sdk: sdk,
    customToolbarButtons: [EmojiButton(), CodeBlockButton()]
)
```

Or add them globally on the SDK (applies to all instances):

```swift
sdk.addGlobalCustomToolbarButton(EmojiButton())
sdk.removeGlobalCustomToolbarButton(id: "emoji")
sdk.clearGlobalCustomToolbarButtons()
```

### Feed Toolbar Buttons

Implement `FeedCustomToolbarButton` for the post creation form:

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

Pass them to the creation view:

```swift
FeedPostCreateView(
    sdk: sdk,
    customToolbarButtons: [HashtagButton()],
    onPostCreated: { _ in },
    onCancelled: { }
)
```

Or set them globally on the feed SDK:

```swift
sdk.globalFeedToolbarButtons = [HashtagButton()]
```

---