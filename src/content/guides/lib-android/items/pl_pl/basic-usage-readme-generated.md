### 1. Dodaj FastCommentsView do swojego układu

```xml
<com.fastcomments.sdk.FastCommentsView
    android:id="@+id/commentsView"
    android:layout_width="match_parent"
    android:layout_height="match_parent" />
```

### 2. Zainicjalizuj i skonfiguruj SDK

```kotlin
// Skonfiguruj SDK
val config = CommentWidgetConfig(
    "your-tenant-id", 
    "page-url-id", 
    "Page Title", 
    "yourdomain.com", 
    "Site Name"
)

// Dodatkowe opcje konfiguracji
config.voteStyle = VoteStyle.UpDown // lub VoteStyle.Heart
config.enableInfiniteScrolling = true
config.hasDarkBackground = true // dla obsługi trybu ciemnego

// Zainicjalizuj SDK
val sdk = FastCommentsSDK(config)

// Znajdź widok komentarzy w układzie
val commentsView = findViewById<FastCommentsView>(R.id.commentsView)

// Ustaw instancję SDK dla widoku
commentsView.setSDK(sdk)

// Załaduj komentarze
commentsView.load()
```