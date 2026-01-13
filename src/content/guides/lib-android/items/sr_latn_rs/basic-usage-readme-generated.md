### 1. Dodajte FastCommentsView u svoj layout

```xml
<com.fastcomments.sdk.FastCommentsView
    android:id="@+id/commentsView"
    android:layout_width="match_parent"
    android:layout_height="match_parent" />
```

### 2. Inicijalizujte i konfigurišite SDK

```kotlin
// Konfigurišite SDK
val config = CommentWidgetConfig(
    "your-tenant-id", 
    "page-url-id", 
    "Page Title", 
    "yourdomain.com", 
    "Site Name"
)

// Dodatne opcije konfiguracije
config.voteStyle = VoteStyle.UpDown // ili VoteStyle.Heart
config.enableInfiniteScrolling = true
config.hasDarkBackground = true // za podršku tamnom režimu

// Inicijalizujte SDK
val sdk = FastCommentsSDK(config)

// Pronađite comments view u svom layoutu
val commentsView = findViewById<FastCommentsView>(R.id.commentsView)

// Postavite instancu SDK-a za view
commentsView.setSDK(sdk)

// Učitajte komentare
commentsView.load()
```