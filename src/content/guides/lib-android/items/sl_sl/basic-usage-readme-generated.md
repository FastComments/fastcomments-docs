### 1. Dodajte FastCommentsView v vašo postavitev

```xml
<com.fastcomments.sdk.FastCommentsView
    android:id="@+id/commentsView"
    android:layout_width="match_parent"
    android:layout_height="match_parent" />
```

### 2. Inicializirajte in konfigurirajte SDK

```kotlin
// Konfigurirajte SDK
val config = CommentWidgetConfig(
    "your-tenant-id", 
    "page-url-id", 
    "Page Title", 
    "yourdomain.com", 
    "Site Name"
)

// Dodatne možnosti konfiguracije
config.voteStyle = VoteStyle.UpDown // ali VoteStyle.Heart
config.enableInfiniteScrolling = true
config.hasDarkBackground = true // za podporo temnemu načinu

// Inicializirajte SDK
val sdk = FastCommentsSDK(config)

// Poiščite pogled komentarjev v vaši postavitvi
val commentsView = findViewById<FastCommentsView>(R.id.commentsView)

// Nastavite instanco SDK za pogled
commentsView.setSDK(sdk)

// Naložite komentarje
commentsView.load()
```