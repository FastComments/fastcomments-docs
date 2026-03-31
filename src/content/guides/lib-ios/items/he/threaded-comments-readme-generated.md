### שימוש בסיסי

```swift
struct CommentsPage: View {
    @StateObject private var sdk = FastCommentsSDK(
        config: FastCommentsWidgetConfig(
            tenantId: "YOUR_TENANT_ID",
            urlId: "article-42",
            url: "https://example.com/article/42",
            pageTitle: "Article Title"
        )
    )

    var body: some View {
        FastCommentsView(sdk: sdk)
            .task {
                try? await sdk.load()
            }
    }
}
```

### סגנונות הצבעה

הסגנון ברירת המחדל מציג חצים למעלה/למטה. העבר את `._1` כדי לקבל הצבעות בסגנון לב:

```swift
FastCommentsView(sdk: sdk, voteStyle: ._1)
```

| Style | Appearance |
|-------|------------|
| `._0` | Up/down arrow buttons with net count |
| `._1` | Single heart button with count |

### קריאות חזרה של אירועים

השתמש בקריאות חזרה בסגנון modifier כדי לטפל באינטראקציות משתמש:

```swift
FastCommentsView(sdk: sdk)
    .onCommentPosted { comment in
        print("New comment: \(comment.commentHTML)")
    }
    .onReplyClick { renderableComment in
        print("Replying to: \(renderableComment.comment.id)")
    }
    .onUserClick { context, userInfo, source in
        // המקור הוא .name או .avatar
        print("Tapped \(userInfo.displayName)")
    }
```

### החלת ערכת נושא

העבר ערכת נושא דרך סביבת SwiftUI:

```swift
FastCommentsView(sdk: sdk)
    .fastCommentsTheme(myTheme)
    .task { try? await sdk.load() }
```

או קבע אותה ישירות ב-SDK:

```swift
sdk.theme = FastCommentsTheme.modern
```

### כיוון המיון

```swift
sdk.defaultSortDirection = .nf  // החדשים ביותר תחילה (ברירת מחדל)
sdk.defaultSortDirection = .of  // הישנים ביותר תחילה
sdk.defaultSortDirection = .mr  // הרלוונטיים ביותר
```

---
---