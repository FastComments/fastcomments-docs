### 1. Düzeninize FastCommentsView ekleyin

```xml
<com.fastcomments.sdk.FastCommentsView
    android:id="@+id/commentsView"
    android:layout_width="match_parent"
    android:layout_height="match_parent" />
```

### 2. SDK'yı başlatın ve yapılandırın

```kotlin
// SDK'yı yapılandırın
val config = CommentWidgetConfig(
    "your-tenant-id", 
    "page-url-id", 
    "Page Title", 
    "yourdomain.com", 
    "Site Name"
)

// Ek yapılandırma seçenekleri
config.voteStyle = VoteStyle.UpDown // veya VoteStyle.Heart
config.enableInfiniteScrolling = true
config.hasDarkBackground = true // koyu mod desteği için

// SDK'yı başlatın
val sdk = FastCommentsSDK(config)

// Düzeninizdeki yorumlar görünümünü bulun
val commentsView = findViewById<FastCommentsView>(R.id.commentsView)

// Görünüm için SDK örneğini ayarlayın
commentsView.setSDK(sdk)

// Yorumları yükleyin
commentsView.load()
```