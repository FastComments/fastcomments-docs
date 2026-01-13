### 1. Fügen Sie FastCommentsView zu Ihrem Layout hinzu

```xml
<com.fastcomments.sdk.FastCommentsView
    android:id="@+id/commentsView"
    android:layout_width="match_parent"
    android:layout_height="match_parent" />
```

### 2. Initialisieren und konfigurieren Sie das SDK

```kotlin
// SDK konfigurieren
val config = CommentWidgetConfig(
    "your-tenant-id", 
    "page-url-id", 
    "Page Title", 
    "yourdomain.com", 
    "Site Name"
)

// Zusätzliche Konfigurationsoptionen
config.voteStyle = VoteStyle.UpDown // oder VoteStyle.Heart
config.enableInfiniteScrolling = true
config.hasDarkBackground = true // zur Unterstützung des Dunkelmodus

// SDK initialisieren
val sdk = FastCommentsSDK(config)

// Kommentaransicht in Ihrem Layout finden
val commentsView = findViewById<FastCommentsView>(R.id.commentsView)

// SDK-Instanz für die Ansicht setzen
commentsView.setSDK(sdk)

// Kommentare laden
commentsView.load()
```