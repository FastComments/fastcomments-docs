### 1. Aggiungi FastCommentsView al tuo layout

```xml
<com.fastcomments.sdk.FastCommentsView
    android:id="@+id/commentsView"
    android:layout_width="match_parent"
    android:layout_height="match_parent" />
```

### 2. Inizializza e configura l'SDK

```kotlin
// Configura l'SDK
val config = CommentWidgetConfig(
    "your-tenant-id", 
    "page-url-id", 
    "Page Title", 
    "yourdomain.com", 
    "Site Name"
)

// Opzioni di configurazione aggiuntive
config.voteStyle = VoteStyle.UpDown // oppure VoteStyle.Heart
config.enableInfiniteScrolling = true
config.hasDarkBackground = true // per il supporto della modalità scura

// Inizializza l'SDK
val sdk = FastCommentsSDK(config)

// Trova FastCommentsView nel tuo layout
val commentsView = findViewById<FastCommentsView>(R.id.commentsView)

// Imposta l'istanza SDK per la view
commentsView.setSDK(sdk)

// Carica i commenti
commentsView.load()
```