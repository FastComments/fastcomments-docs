### ערכות נושא מוכנות

ארבע ערכות נושא מובנות זמינות:

```swift
// ברירות מחדל של המערכת
sdk.theme = FastCommentsTheme.default

// כרטיסים עם צללים ופינות מעוגלות גדולות
sdk.theme = FastCommentsTheme.modern

// שטוח, ללא צללים, רדיוס פינות קטן, ללא קווי שרשור
sdk.theme = FastCommentsTheme.minimal

// הגדרת כל צבעי הפעולה לצבע מותג אחד
sdk.theme = FastCommentsTheme.allPrimary(.indigo)
```

### סגנונות תצוגת תגובות

```swift
var theme = FastCommentsTheme()
theme.commentStyle = .flat    // רשימה שטוחה עם מפרידים (ברירת מחדל)
theme.commentStyle = .card    // כרטיסים מעוגלים עם צללים
theme.commentStyle = .bubble  // סגנון בועת צ'אט
```

### צבעים

כל מאפייני הצבע אופציונליים. ערכים שלא הוגדרו חוזרים לברירות מחדל סבירות של המערכת.

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

// דיאלוגים
theme.dialogHeaderBackgroundColor = .indigo
theme.dialogHeaderTextColor = .white

// סרגל קלט
theme.inputBarBackgroundColor = Color(.systemBackground)
theme.inputBarBorderColor = Color(.separator)

// אחר
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

### פריסה ומרווחים

```swift
theme.cornerRadius = .large       // .none, .small, .medium, .large
theme.commentSpacing = 4          // נקודות בין שורות תגובות
theme.nestingIndent = 20          // נקודות הזחה לכל רמת קינון
theme.avatarSize = 36             // קוטר האווטאר לתגובות שורש
theme.replyAvatarSize = 28        // קוטר האווטאר לתשובות מקוננות
```

### אפקטים חזותיים

```swift
theme.showShadows = true          // צללים עדינים על כרטיסים
theme.showThreadLine = true       // קו אנכי המחבר תשובות מקוננות
theme.animateVotes = true         // אנימציית קפיץ בשינויי הצבעות
```

### החלת ערכות נושא

שתי גישות:

```swift
// דרך סביבת SwiftUI (מומלץ עבור היררכיית תצוגות)
FastCommentsView(sdk: sdk)
    .fastCommentsTheme(theme)

// ישירות על ה-SDK
sdk.theme = theme
```

---


### צבעי סקרים

סקרים משתמשים כברירת מחדל בצבעי כרטיס הסקר של ווידג'ט האינטרנט, במצב בהיר וכהה. הגדרת `primaryColor` משנה גם את צבע ההדגשה ואת הסרגל של האפשרות שבחרת. כדי להגדיר אותם ישירות:

```swift
var theme = FastCommentsTheme()
theme.pollAccentColor = .indigo
```

| מאפיין | משמש עבור |
|----------|----------|
| `pollAccentColor` | גבול וסימון האפשרות הנבחרת, משך הבחירה, "ההצבעה שלך נרשמה" |
| `pollAccentTextColor` | סימן הוי וטקסט המצויר על צבע ההדגשה |
| `pollOptionBackgroundColor` | רקע צ'יפ האפשרות |
| `pollOptionPressedBackgroundColor` | רקע צ'יפ האפשרות בזמן לחיצה |
| `pollBarColor` | סרגל תוצאות לאפשרויות שלא בחרת |
| `pollSelectedBarColor` | סרגל תוצאות לאפשרות שבחרת |
| `pollBorderColor` | סימון תיבת סימון וגלולות מתוחמות |
| `pollDangerColor` | שגיאות ו"הסרת סקר" |