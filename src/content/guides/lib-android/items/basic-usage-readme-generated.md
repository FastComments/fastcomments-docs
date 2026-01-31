### 1. Add FastCommentsView to your layout

```xml
<com.fastcomments.sdk.FastCommentsView
    android:id="@+id/commentsView"
    android:layout_width="match_parent"
    android:layout_height="match_parent" />
```

### 2. Initialize and configure the SDK

```kotlin
// Configure the SDK
val config = CommentWidgetConfig(
    "your-tenant-id", 
    "page-url-id", 
    "Page Title", 
    "yourdomain.com", 
    "Site Name"
)

// Additional configuration options
config.voteStyle = VoteStyle.UpDown // or VoteStyle.Heart
config.enableInfiniteScrolling = true
config.hasDarkBackground = true // for dark mode support

// Initialize the SDK
val sdk = FastCommentsSDK(config)

// Find the comments view in your layout
val commentsView = findViewById<FastCommentsView>(R.id.commentsView)

// Set the SDK instance for the view
commentsView.setSDK(sdk)

// Load comments
commentsView.load()
```