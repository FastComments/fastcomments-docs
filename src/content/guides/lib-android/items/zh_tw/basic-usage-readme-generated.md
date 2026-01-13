### 1. 將 FastCommentsView 加入您的佈局

```xml
<com.fastcomments.sdk.FastCommentsView
    android:id="@+id/commentsView"
    android:layout_width="match_parent"
    android:layout_height="match_parent" />
```

### 2. 初始化並設定 SDK

```kotlin
// 設定 SDK
val config = CommentWidgetConfig(
    "your-tenant-id", 
    "page-url-id", 
    "Page Title", 
    "yourdomain.com", 
    "Site Name"
)

// 其他設定選項
config.voteStyle = VoteStyle.UpDown // or VoteStyle.Heart
config.enableInfiniteScrolling = true
config.hasDarkBackground = true // 支援深色模式

// 初始化 SDK
val sdk = FastCommentsSDK(config)

// 在佈局中找到 comments view
val commentsView = findViewById<FastCommentsView>(R.id.commentsView)

// 為該 view 設定 SDK 實例
commentsView.setSDK(sdk)

// 載入留言
commentsView.load()
```