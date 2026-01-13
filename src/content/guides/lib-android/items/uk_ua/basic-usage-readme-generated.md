### 1. Додайте FastCommentsView до вашого макета

```xml
<com.fastcomments.sdk.FastCommentsView
    android:id="@+id/commentsView"
    android:layout_width="match_parent"
    android:layout_height="match_parent" />
```

### 2. Ініціалізуйте та налаштуйте SDK

```kotlin
// Налаштуйте SDK
val config = CommentWidgetConfig(
    "your-tenant-id", 
    "page-url-id", 
    "Page Title", 
    "yourdomain.com", 
    "Site Name"
)

// Додаткові параметри конфігурації
config.voteStyle = VoteStyle.UpDown // або VoteStyle.Heart
config.enableInfiniteScrolling = true
config.hasDarkBackground = true // для підтримки темного режиму

// Ініціалізуйте SDK
val sdk = FastCommentsSDK(config)

// Знайдіть FastCommentsView у вашому макеті
val commentsView = findViewById<FastCommentsView>(R.id.commentsView)

// Встановіть екземпляр SDK для view
commentsView.setSDK(sdk)

// Завантажте коментарі
commentsView.load()
```