### 1. 将 FastCommentsView 添加到您的布局

```xml
<com.fastcomments.sdk.FastCommentsView
    android:id="@+id/commentsView"
    android:layout_width="match_parent"
    android:layout_height="match_parent" />
```

### 2. 初始化并配置 SDK

```kotlin
// 配置 SDK
val config = CommentWidgetConfig(
    "your-tenant-id", 
    "page-url-id", 
    "Page Title", 
    "yourdomain.com", 
    "Site Name"
)

// 其他配置选项
config.voteStyle = VoteStyle.UpDown // 或 VoteStyle.Heart
config.enableInfiniteScrolling = true
config.hasDarkBackground = true // 用于暗色模式支持

// 初始化 SDK
val sdk = FastCommentsSDK(config)

// 在布局中查找评论视图
val commentsView = findViewById<FastCommentsView>(R.id.commentsView)

// 为视图设置 SDK 实例
commentsView.setSDK(sdk)

// 加载评论
commentsView.load()
```