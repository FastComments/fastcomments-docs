### 1. Voeg FastCommentsView toe aan je layout

```xml
<com.fastcomments.sdk.FastCommentsView
    android:id="@+id/commentsView"
    android:layout_width="match_parent"
    android:layout_height="match_parent" />
```

### 2. Initialiseer en configureer de SDK

```kotlin
// Configureer de SDK
val config = CommentWidgetConfig(
    "your-tenant-id", 
    "page-url-id", 
    "Page Title", 
    "yourdomain.com", 
    "Site Name"
)

// Aanvullende configuratieopties
config.voteStyle = VoteStyle.UpDown // of VoteStyle.Heart
config.enableInfiniteScrolling = true
config.hasDarkBackground = true // voor ondersteuning van donkere modus

// Initialiseer de SDK
val sdk = FastCommentsSDK(config)

// Zoek de commentsweergave in je layout
val commentsView = findViewById<FastCommentsView>(R.id.commentsView)

// Stel de SDK-instantie in voor de view
commentsView.setSDK(sdk)

// Laad reacties
commentsView.load()
```