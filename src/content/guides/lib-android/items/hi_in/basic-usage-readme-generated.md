### 1. अपने लेआउट में FastCommentsView जोड़ें

```xml
<com.fastcomments.sdk.FastCommentsView
    android:id="@+id/commentsView"
    android:layout_width="match_parent"
    android:layout_height="match_parent" />
```

### 2. SDK को इनिशियलाइज़ और कॉन्फ़िगर करें

```kotlin
// SDK कॉन्फ़िगर करें
val config = CommentWidgetConfig(
    "your-tenant-id", 
    "page-url-id", 
    "Page Title", 
    "yourdomain.com", 
    "Site Name"
)

// अतिरिक्त कॉन्फ़िगरेशन विकल्प
config.voteStyle = VoteStyle.UpDown // या VoteStyle.Heart
config.enableInfiniteScrolling = true
config.hasDarkBackground = true // डार्क मोड सपोर्ट के लिए

// SDK को इनिशियलाइज़ करें
val sdk = FastCommentsSDK(config)

// अपने लेआउट में कमेंट्स व्यू खोजें
val commentsView = findViewById<FastCommentsView>(R.id.commentsView)

// व्यू के लिए SDK इंस्टेंस सेट करें
commentsView.setSDK(sdk)

// कमेंट्स लोड करें
commentsView.load()
```