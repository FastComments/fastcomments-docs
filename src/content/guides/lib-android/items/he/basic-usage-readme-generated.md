### 1. הוסף FastCommentsView לפריסה שלך

```xml
<com.fastcomments.sdk.FastCommentsView
    android:id="@+id/commentsView"
    android:layout_width="match_parent"
    android:layout_height="match_parent" />
```

### 2. אתחול וקונפיגורציה של ה‑SDK

```kotlin
// קבע את תצורת ה‑SDK
val config = CommentWidgetConfig(
    "your-tenant-id", 
    "page-url-id", 
    "Page Title", 
    "yourdomain.com", 
    "Site Name"
)

// אפשרויות קונפיגורציה נוספות
config.voteStyle = VoteStyle.UpDown // או VoteStyle.Heart
config.enableInfiniteScrolling = true
config.hasDarkBackground = true // עבור תמיכה במצב כהה

// אתחל את ה‑SDK
val sdk = FastCommentsSDK(config)

// מצא את תצוגת התגובות בפריסה שלך
val commentsView = findViewById<FastCommentsView>(R.id.commentsView)

// הגדר את מופע ה‑SDK לתצוגה
commentsView.setSDK(sdk)

// טען תגובות
commentsView.load()
```