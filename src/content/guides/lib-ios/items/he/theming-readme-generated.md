### תבניות נושא

ארבע תבניות מובנות זמינות:

```swift
// ברירות מחדל של המערכת
sdk.theme = FastCommentsTheme.default

// קלפים עם צללים ופינות מעוגלות גדולות
sdk.theme = FastCommentsTheme.modern

// שטוח, ללא צללים, רדיוס פינות קטן, ללא קווי שרשור
sdk.theme = FastCommentsTheme.minimal

// קבע את כל צבעי הפעולות לצבע מותג יחיד
sdk.theme = FastCommentsTheme.allPrimary(.indigo)
```

### סגנונות תצוגת תגובות

```swift
var theme = FastCommentsTheme()
theme.commentStyle = .flat    // רשימה שטוחה עם מפרידים (ברירת מחדל)
theme.commentStyle = .card    // קלפים מעוגלים עם צללים
theme.commentStyle = .bubble  // סגנון בועת שיחה
```

### צבעים

כל מאפייני הצבע אופציונליים. ערכים שלא הוגדרו יחזרו לברירות המחדל הסבירות של המערכת.

```swift
var theme = FastCommentsTheme()

// צבעי מותג
theme.primaryColor = .indigo
theme.primaryLightColor = .indigo.opacity(0.6)
theme.primaryDarkColor = Color(red: 0.2, green: 0.1, blue: 0.5)

// רקעים
theme.commentBackgroundColor = Color(.secondarySystemGroupedBackground)
theme.containerBackgroundColor = Color(.systemGroupedBackground)

// כפתורי פעולה
theme.actionButtonColor = .indigo
theme.replyButtonColor = .indigo
theme.toggleRepliesButtonColor = .indigo.opacity(0.8)
theme.loadMoreButtonTextColor = .indigo

// הצבעות
theme.voteActiveColor = .red
theme.voteCountColor = .primary
theme.voteCountZeroColor = .secondary
theme.voteDividerColor = Color(.separator)

// קישורים
theme.linkColor = .indigo
theme.linkColorPressed = .indigo.opacity(0.5)

// תיבות דו-שיח
theme.dialogHeaderBackgroundColor = .indigo
theme.dialogHeaderTextColor = .white

// סרגל קלט
theme.inputBarBackgroundColor = Color(.systemBackground)
theme.inputBarBorderColor = Color(.separator)

// אחרים
theme.onlineIndicatorColor = .green
theme.separatorColor = Color(.separator)
theme.badgeBackgroundColor = .gray.opacity(0.2)
theme.threadLineColor = .indigo.opacity(0.15)
```

### טיפוגרפיה

```swift
theme.commenterNameFont = .subheadline.weight(.bold)
theme.bodyFont = .body
theme.captionFont = .caption
theme.actionFont = .caption.weight(.medium)
```

### פריסה וריווח

```swift
theme.cornerRadius = .large       // אפשרויות: .none, .small, .medium, .large
theme.commentSpacing = 4          // נקודות בין שורות תגובות
theme.nestingIndent = 20          // נקודות הזחה לכל רמת שרשור
theme.avatarSize = 36             // קוטר האוואטר לתגובות שורש
theme.replyAvatarSize = 28        // קוטר האוואטר לתגובות מקוננות
```

### אפקטים ויזואליים

```swift
theme.showShadows = true          // צללים עדינים על קלפים
theme.showThreadLine = true       // קו אנכי המחבר תגובות מקוננות
theme.animateVotes = true         // אנימציית קפיץ בעת שינויי הצבעה
```

### החלת ערכות נושא

שתי גישות:

```swift
// Via SwiftUI environment (recommended for view hierarchy)
FastCommentsView(sdk: sdk)
    .fastCommentsTheme(theme)

// Directly on the SDK
sdk.theme = theme
```

---
---