### 1. レイアウトに FastCommentsView を追加する

```xml
<com.fastcomments.sdk.FastCommentsView
    android:id="@+id/commentsView"
    android:layout_width="match_parent"
    android:layout_height="match_parent" />
```

### 2. SDK を初期化して設定する

```kotlin
// SDK を構成
val config = CommentWidgetConfig(
    "your-tenant-id", 
    "page-url-id", 
    "Page Title", 
    "yourdomain.com", 
    "Site Name"
)

// 追加の設定オプション
config.voteStyle = VoteStyle.UpDown // または VoteStyle.Heart
config.enableInfiniteScrolling = true
config.hasDarkBackground = true // ダークモード対応

// SDK を初期化
val sdk = FastCommentsSDK(config)

// レイアウト内のコメントビューを取得
val commentsView = findViewById<FastCommentsView>(R.id.commentsView)

// ビューに SDK インスタンスを設定
commentsView.setSDK(sdk)

// コメントを読み込む
commentsView.load()
```