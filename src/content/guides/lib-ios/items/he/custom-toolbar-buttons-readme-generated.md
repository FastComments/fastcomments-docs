### כפתורי סרגל הכלים של תגובות

ממש את הפרוטוקול `CustomToolbarButton` כדי להוסיף כפתורים לסרגל הכלים של שדה התגובה:

```swift
struct EmojiButton: CustomToolbarButton {
    let id = "emoji"
    let iconSystemName = "face.smiling"       // שם ה-SF Symbol
    let contentDescription = "Add Emoji"
    let badgeText: String? = nil              // ספירת תג אופציונלית

    func onClick(text: Binding<String>) {
        text.wrappedValue += "\u{1F44D}"
    }

    // אפשרויות ברירת מחדל שניתנות להחלפה (ברירת מחדל true)
    func isEnabled() -> Bool { true }
    func isVisible() -> Bool { true }
}
```

העבר כפתורים מותאמים בעת יצירת התצוגה:

```swift
FastCommentsView(
    sdk: sdk,
    customToolbarButtons: [EmojiButton(), CodeBlockButton()]
)
```

או הוסף אותם באופן גלובלי ב-SDK (חל על כל המופעים):

```swift
sdk.addGlobalCustomToolbarButton(EmojiButton())
sdk.removeGlobalCustomToolbarButton(id: "emoji")
sdk.clearGlobalCustomToolbarButtons()
```

### כפתורי סרגל הכלים של הפיד

ממש את `FeedCustomToolbarButton` עבור טופס יצירת הפוסט:

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

העבר אותם לתצוגת היצירה:

```swift
FeedPostCreateView(
    sdk: sdk,
    customToolbarButtons: [HashtagButton()],
    onPostCreated: { _ in },
    onCancelled: { }
)
```

או הגדר אותם גלובלית ב-SDK של הפיד:

```swift
sdk.globalFeedToolbarButtons = [HashtagButton()]
```

---
---