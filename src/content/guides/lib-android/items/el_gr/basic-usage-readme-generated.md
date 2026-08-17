### 1. Προσθήκη FastCommentsView στη διάταξή σας

```xml
<com.fastcomments.sdk.FastCommentsView
    android:id="@+id/commentsView"
    android:layout_width="match_parent"
    android:layout_height="match_parent" />
```

### 2. Αρχικοποίηση και διαμόρφωση του SDK

```kotlin
// Διαμόρφωση του SDK
val config = CommentWidgetConfig(
    "your-tenant-id", 
    "page-url-id", 
    "Page Title", 
    "yourdomain.com", 
    "Site Name"
)

// Επιπλέον επιλογές διαμόρφωσης
config.voteStyle = VoteStyle.UpDown // or VoteStyle.Heart
config.enableInfiniteScrolling = true
config.hasDarkBackground = true // for dark mode support

// Αρχικοποίηση του SDK
val sdk = FastCommentsSDK(config)

// Εύρεση της προβολής σχολίων στη διάταξή σας
val commentsView = findViewById<FastCommentsView>(R.id.commentsView)

// Ορισμός του αντικειμένου SDK για την προβολή
commentsView.setSDK(sdk)

// Φόρτωση σχολίων
commentsView.load()
```