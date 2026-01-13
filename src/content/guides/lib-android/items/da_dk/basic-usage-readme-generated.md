### 1. Tilføj FastCommentsView til dit layout

```xml
<com.fastcomments.sdk.FastCommentsView
    android:id="@+id/commentsView"
    android:layout_width="match_parent"
    android:layout_height="match_parent" />
```

### 2. Initialiser og konfigurer SDK'et

```kotlin
// Konfigurer SDK'et
val config = CommentWidgetConfig(
    "your-tenant-id", 
    "page-url-id", 
    "Page Title", 
    "yourdomain.com", 
    "Site Name"
)

// Yderligere konfigurationsmuligheder
config.voteStyle = VoteStyle.UpDown // eller VoteStyle.Heart
config.enableInfiniteScrolling = true
config.hasDarkBackground = true // til understøttelse af mørk tilstand

// Initialiser SDK'et
val sdk = FastCommentsSDK(config)

// Find comment-vinduet i dit layout
val commentsView = findViewById<FastCommentsView>(R.id.commentsView)

// Indstil SDK-instansen for visningen
commentsView.setSDK(sdk)

// Indlæs kommentarer
commentsView.load()
```