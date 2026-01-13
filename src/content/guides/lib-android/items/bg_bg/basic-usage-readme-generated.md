### 1. Добавете FastCommentsView към вашия layout

```xml
<com.fastcomments.sdk.FastCommentsView
    android:id="@+id/commentsView"
    android:layout_width="match_parent"
    android:layout_height="match_parent" />
```

### 2. Инициализирайте и конфигурирайте SDK

```kotlin
// Конфигурирайте SDK
val config = CommentWidgetConfig(
    "your-tenant-id", 
    "page-url-id", 
    "Page Title", 
    "yourdomain.com", 
    "Site Name"
)

// Допълнителни опции за конфигурация
config.voteStyle = VoteStyle.UpDown // или VoteStyle.Heart
config.enableInfiniteScrolling = true
config.hasDarkBackground = true // за поддръжка на тъмен режим

// Инициализирайте SDK
val sdk = FastCommentsSDK(config)

// Намерете изгледа за коментари в оформлението си
val commentsView = findViewById<FastCommentsView>(R.id.commentsView)

// Задайте инстанцията на SDK за изгледа
commentsView.setSDK(sdk)

// Заредете коментарите
commentsView.load()
```