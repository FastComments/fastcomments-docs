### 테마 프리셋

네 가지 내장 프리셋을 사용할 수 있습니다:

```swift
// 시스템 기본값
sdk.theme = FastCommentsTheme.default

// 그림자와 큰 둥근 모서리가 있는 카드
sdk.theme = FastCommentsTheme.modern

// 평면, 그림자 없음, 작은 모서리 반경, 스레드 라인 없음
sdk.theme = FastCommentsTheme.minimal

// 모든 액션 색상을 단일 브랜드 색상으로 설정
sdk.theme = FastCommentsTheme.allPrimary(.indigo)
```

### 댓글 표시 스타일

```swift
var theme = FastCommentsTheme()
theme.commentStyle = .flat    // 구분선이 있는 평면 목록 (기본값)
theme.commentStyle = .card    // 그림자가 있는 둥근 카드
theme.commentStyle = .bubble  // 채팅 버블 스타일
```

### 색상

모든 색상 속성은 선택 사항입니다. 설정하지 않은 값은 적절한 시스템 기본값으로 대체됩니다.

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

// 대화상자
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
theme.commentSpacing = 4          // 댓글 행 사이의 포인트 간격
theme.nestingIndent = 20          // 중첩 레벨당 들여쓰기 포인트
theme.avatarSize = 36             // 루트 댓글의 아바타 지름
theme.replyAvatarSize = 28        // 중첩된 답글의 아바타 지름
```

### 시각 효과

```swift
theme.showShadows = true          // 카드에 은은한 그림자
theme.showThreadLine = true       // 중첩된 답글을 연결하는 세로선
theme.animateVotes = true         // 투표 변경 시 스프링 애니메이션
```

### 테마 적용

두 가지 방법:

```swift
// SwiftUI 환경을 통한 방법 (뷰 계층 구조에 권장)
FastCommentsView(sdk: sdk)
    .fastCommentsTheme(theme)

// SDK에 직접 설정
sdk.theme = theme
```

---


### 투표 색상

투표는 라이트 및 다크 모드에서 웹 위젯의 투표 카드 색상을 기본값으로 사용합니다. `primaryColor`를 설정하면 강조 색상과 선택한 옵션의 바도 함께 변경됩니다. 직접 설정하려면:

```swift
var theme = FastCommentsTheme()
theme.pollAccentColor = .indigo
```

| 속성 | 용도 |
|----------|----------|
| `pollAccentColor` | 선택된 옵션 테두리 및 체크, 선택한 기간, "Your vote is in" |
| `pollAccentTextColor` | 강조 색상 위에 그려지는 체크 표시 및 텍스트 |
| `pollOptionBackgroundColor` | 옵션 칩 배경 |
| `pollOptionPressedBackgroundColor` | 눌렀을 때의 옵션 칩 배경 |
| `pollBarColor` | 선택하지 않은 옵션의 결과 바 |
| `pollSelectedBarColor` | 선택한 옵션의 결과 바 |
| `pollBorderColor` | 체크박스 표시 및 외곽선 필 |
| `pollDangerColor` | 오류 및 "Remove poll" |