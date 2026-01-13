### 1. Add FastCommentsView to your layout

```xml
<com.fastcomments.sdk.FastCommentsView
    android:id="@+id/commentsView"
    android:layout_width="match_parent"
    android:layout_height="match_parent" />
```

### 2. Initialize and configure the SDK

```kotlin
// Διαμορφώστε το SDK
val config = CommentWidgetConfig(
    "your-tenant-id", 
    "page-url-id", 
    "Page Title", 
    "yourdomain.com", 
    "Site Name"
)

// Επιπρόσθετες επιλογές διαμόρφωσης
config.voteStyle = VoteStyle.UpDown // ή VoteStyle.Heart
config.enableInfiniteScrolling = true
config.hasDarkBackground = true // για υποστήριξη σκοτεινής λειτουργίας

// Αρχικοποιήστε το SDK
val sdk = FastCommentsSDK(config)

// Βρείτε την προβολή σχολίων στο layout σας
val commentsView = findViewById<FastCommentsView>(R.id.commentsView)

// Ορίστε το instance του SDK για την προβολή
commentsView.setSDK(sdk)

// Φορτώστε τα σχόλια
commentsView.load()
```