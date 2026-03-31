### 테마 프리셋

네 가지 내장 프리셋이 제공됩니다:

```swift
// 시스템 기본값
sdk.theme = FastCommentsTheme.default

// 그림자와 큰 둥근 모서리가 있는 카드
sdk.theme = FastCommentsTheme.modern

// 플랫(그림자 없음), 작은 모서리 반경, 스레드 라인 없음
sdk.theme = FastCommentsTheme.minimal

// 모든 액션 색상을 단일 브랜드 색상으로 설정
sdk.theme = FastCommentsTheme.allPrimary(.indigo)
```

### 댓글 표시 스타일

```swift
var theme = FastCommentsTheme()
theme.commentStyle = .flat    // 구분선이 있는 플랫 리스트 (기본)
theme.commentStyle = .card    // 그림자가 있는 둥근 카드
theme.commentStyle = .bubble  // 채팅 버블 스타일
```

### 색상

모든 색상 속성은 선택 사항입니다. 설정되지 않은 값은 적절한 시스템 기본값으로 대체됩니다.

```swift
var theme = FastCommentsTheme()

// 브랜드 색상
theme.primaryColor = .indigo
theme.primaryLightColor = .indigo.opacity(0.6)
theme.primaryDarkColor = Color(red: 0.2, green: 0.1, blue: 0.5)

// 배경
theme.commentBackgroundColor = Color(.secondarySystemGroupedBackground)
theme.containerBackgroundColor = Color(.systemGroupedBackground)

// 액션 버튼
theme.actionButtonColor = .indigo
theme.replyButtonColor = .indigo
theme.toggleRepliesButtonColor = .indigo.opacity(0.8)
theme.loadMoreButtonTextColor = .indigo

// 투표
theme.voteActiveColor = .red
theme.voteCountColor = .primary
theme.voteCountZeroColor = .secondary
theme.voteDividerColor = Color(.separator)

// 링크
theme.linkColor = .indigo
theme.linkColorPressed = .indigo.opacity(0.5)

// 다이얼로그
theme.dialogHeaderBackgroundColor = .indigo
theme.dialogHeaderTextColor = .white

// 입력 바
theme.inputBarBackgroundColor = Color(.systemBackground)
theme.inputBarBorderColor = Color(.separator)

// 기타
theme.onlineIndicatorColor = .green
theme.separatorColor = Color(.separator)
theme.badgeBackgroundColor = .gray.opacity(0.2)
theme.threadLineColor = .indigo.opacity(0.15)
```

### 타이포그래피

```swift
theme.commenterNameFont = .subheadline.weight(.bold)
theme.bodyFont = .body
theme.captionFont = .caption
theme.actionFont = .caption.weight(.medium)
```

### 레이아웃 및 간격

```swift
theme.cornerRadius = .large       // .none, .small, .medium, .large
theme.commentSpacing = 4          // 댓글 행 사이의 간격(포인트)
theme.nestingIndent = 20          // 중첩 수준당 들여쓰기(포인트)
theme.avatarSize = 36             // 루트 댓글의 아바타 지름
theme.replyAvatarSize = 28        // 중첩된 답글의 아바타 지름
```

### 시각 효과

```swift
theme.showShadows = true          // 카드에 미묘한 그림자
theme.showThreadLine = true       // 중첩된 답글을 연결하는 수직선
theme.animateVotes = true         // 투표 변경 시 스프링 애니메이션
```

### 테마 적용

두 가지 방법:

```swift
// Via SwiftUI environment (recommended for view hierarchy)
FastCommentsView(sdk: sdk)
    .fastCommentsTheme(theme)

// Directly on the SDK
sdk.theme = theme
```