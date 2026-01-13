### 1. Додајте FastCommentsView у ваш изглед

```xml
<com.fastcomments.sdk.FastCommentsView
    android:id="@+id/commentsView"
    android:layout_width="match_parent"
    android:layout_height="match_parent" />
```

### 2. Иницијализујте и конфигуришите SDK

```kotlin
// Конфигуришите SDK
val config = CommentWidgetConfig(
    "your-tenant-id", 
    "page-url-id", 
    "Page Title", 
    "yourdomain.com", 
    "Site Name"
)

// Додатне опције конфигурације
config.voteStyle = VoteStyle.UpDown // или VoteStyle.Heart
config.enableInfiniteScrolling = true
config.hasDarkBackground = true // за подршку тамне теме

// Иницијализујте SDK
val sdk = FastCommentsSDK(config)

// Пронађите приказ коментара у вашем изгледу
val commentsView = findViewById<FastCommentsView>(R.id.commentsView)

// Поставите инстанцу SDK-а за приказ
commentsView.setSDK(sdk)

// Учитајте коментаре
commentsView.load()
```