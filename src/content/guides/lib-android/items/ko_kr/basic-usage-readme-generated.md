---
### 1. 레이아웃에 FastCommentsView 추가

```xml
<com.fastcomments.sdk.FastCommentsView
    android:id="@+id/commentsView"
    android:layout_width="match_parent"
    android:layout_height="match_parent" />
```

### 2. SDK 초기화 및 구성

```kotlin
// SDK 구성
val config = CommentWidgetConfig(
    "your-tenant-id", 
    "page-url-id", 
    "Page Title", 
    "yourdomain.com", 
    "Site Name"
)

// 추가 구성 옵션
config.voteStyle = VoteStyle.UpDown // 또는 VoteStyle.Heart
config.enableInfiniteScrolling = true
config.hasDarkBackground = true // 다크 모드 지원용

// SDK 초기화
val sdk = FastCommentsSDK(config)

// 레이아웃에서 댓글 뷰 찾기
val commentsView = findViewById<FastCommentsView>(R.id.commentsView)

// 뷰에 SDK 인스턴스 설정
commentsView.setSDK(sdk)

// 댓글 로드
commentsView.load()
```
---