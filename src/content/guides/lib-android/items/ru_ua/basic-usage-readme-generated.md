### 1. Добавьте FastCommentsView в ваш макет

```xml
<com.fastcomments.sdk.FastCommentsView
    android:id="@+id/commentsView"
    android:layout_width="match_parent"
    android:layout_height="match_parent" />
```

### 2. Инициализируйте и настройте SDK

```kotlin
// Настройка SDK
val config = CommentWidgetConfig(
    "your-tenant-id", 
    "page-url-id", 
    "Page Title", 
    "yourdomain.com", 
    "Site Name"
)

// Дополнительные параметры конфигурации
config.voteStyle = VoteStyle.UpDown // или VoteStyle.Heart
config.enableInfiniteScrolling = true
config.hasDarkBackground = true // для поддержки тёмной темы

// Инициализация SDK
val sdk = FastCommentsSDK(config)

// Найти представление комментариев в вашем макете
val commentsView = findViewById<FastCommentsView>(R.id.commentsView)

// Установить экземпляр SDK для представления
commentsView.setSDK(sdk)

// Загрузить комментарии
commentsView.load()
```