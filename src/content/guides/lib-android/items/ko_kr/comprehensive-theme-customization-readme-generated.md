FastComments SDK의 모든 버튼 및 UI 요소는 테마 설정이 가능합니다. 앱 브랜딩을 완전히 제어하려면 `FastCommentsTheme.Builder`를 사용하세요.

### 프로그래밍 방식 테마 적용 (권장)

```kotlin
val theme = FastCommentsTheme.Builder()
    // 작업 버튼: 전송, 투표, 메뉴, 좋아요/공유 버튼
    .setActionButtonColor(Color.parseColor("#FF1976D2"))
    
    // 답글 버튼: 댓글 답글 버튼  
    .setReplyButtonColor(Color.parseColor("#FF4CAF50"))
    
    // 토글 버튼: 답글 표시/숨기기 버튼
    .setToggleRepliesButtonColor(Color.parseColor("#FFFF5722"))
    
    // 더 보기 버튼: 페이지 매김 버튼
    .setLoadMoreButtonTextColor(Color.parseColor("#FF9C27B0"))
    
    .setPrimaryColor(Color.parseColor("#FF6200EE"))
    .setLinkColor(Color.parseColor("#FF1976D2"))
    .setDialogHeaderBackgroundColor(Color.parseColor("#FF333333"))
    .build()

// 테마 적용
sdk.setTheme(theme)
```

### 빠른 색상 재정의

간단한 브랜딩을 위해 `colors.xml`의 색상 리소스를 오버라이드하세요:

```xml
<!-- 앱의 res/values/colors.xml에서 -->
<resources>
    <!-- 모든 주요 UI 요소 변경 -->
    <color name="primary">#FF1976D2</color>
    
    <!-- 또는 특정 버튼 유형을 사용자화 -->
    <color name="fastcomments_action_button_color">#FF1976D2</color>
    <color name="fastcomments_reply_button_color">#FF4CAF50</color>
    <color name="fastcomments_toggle_replies_button_color">#FFFF5722</color>
    <color name="fastcomments_load_more_button_text_color">#FF9C27B0</color>
</resources>
```

### 테마 적용 버튼 범위

**SDK의 모든 버튼은 테마를 지원합니다:**
- 전송 버튼, 투표 버튼, 메뉴 버튼, 답글 버튼
- 답글 표시/숨기기 버튼, 더 보기 버튼  
- 피드 액션 버튼(좋아요, 댓글, 공유)
- 다이얼로그 버튼(제출, 취소, 저장)
- 피드 게시물의 동적 작업 버튼

자세한 테마 문서는 [THEMING.md](https://github.com/FastComments/fastcomments-android/blob/main/THEMING.md)를 참조하세요.