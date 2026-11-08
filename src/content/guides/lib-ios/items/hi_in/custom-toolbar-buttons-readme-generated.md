### कमेंट टूलबार बटन

कमेंट इनपुट टूलबार में बटन जोड़ने के लिए `CustomToolbarButton` प्रोटोकॉल लागू करें:

```swift
struct EmojiButton: CustomToolbarButton {
    let id = "emoji"
    let iconSystemName = "face.smiling"       // SF Symbol का नाम
    let contentDescription = "Add Emoji"
    let badgeText: String? = nil              // वैकल्पिक बैज काउंट

    func onClick(text: Binding<String>) {
        text.wrappedValue += "\u{1F44D}"
    }

    // वैकल्पिक ओवरराइड (डिफ़ॉल्ट true है)
    func isEnabled() -> Bool { true }
    func isVisible() -> Bool { true }
}
```

व्यू बनाते समय कस्टम बटन पास करें:

```swift
FastCommentsView(
    sdk: sdk,
    customToolbarButtons: [EmojiButton(), CodeBlockButton()]
)
```

या SDK पर उन्हें ग्लोबली जोड़ें (सभी इंस्टेंस पर लागू होता है):

```swift
sdk.addGlobalCustomToolbarButton(EmojiButton())
sdk.removeGlobalCustomToolbarButton(id: "emoji")
sdk.clearGlobalCustomToolbarButtons()
```

### फ़ीड टूलबार बटन

पोस्ट बनाने के फ़ॉर्म के लिए `FeedCustomToolbarButton` लागू करें:

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

उन्हें क्रिएशन व्यू को पास करें:

```swift
FeedPostCreateView(
    sdk: sdk,
    customToolbarButtons: [HashtagButton()],
    onPostCreated: { _ in },
    onCancelled: { }
)
```

या फ़ीड SDK पर उन्हें ग्लोबली सेट करें:

```swift
sdk.globalFeedToolbarButtons = [HashtagButton()]
```

---
---