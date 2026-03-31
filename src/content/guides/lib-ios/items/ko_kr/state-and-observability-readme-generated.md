Both `FastCommentsSDK`와 `FastCommentsFeedSDK`는 `@Published` 속성을 가진 `ObservableObject` 클래스입니다. SwiftUI 뷰에서 이를 관찰하여 반응형 UI 업데이트를 할 수 있습니다.

### FastCommentsSDK Published 속성

| Property | Type | Description |
|----------|------|-------------|
| `commentCountOnServer` | `Int` | 서버의 전체 댓글 수 |
| `newRootCommentCount` | `Int` | 버퍼된 새 댓글들 (`showLiveRightAway`가 false일 때) |
| `currentUser` | `UserSessionInfo?` | 현재 인증된 사용자 |
| `isSiteAdmin` | `Bool` | 현재 사용자가 사이트 관리자인지 여부 |
| `isClosed` | `Bool` | 댓글 스레드가 닫혔는지 여부 |
| `hasBillingIssue` | `Bool` | 청구 관련 문제가 있는지 여부 |
| `isLoading` | `Bool` | 네트워크 요청이 진행 중인지 여부 |
| `hasMore` | `Bool` | 추가 댓글 페이지가 존재하는지 여부 |
| `blockingErrorMessage` | `String?` | UI 작동을 방해하는 오류 |
| `warningMessage` | `String?` | 비차단 경고 메시지 |
| `isDemo` | `Bool` | 데모 모드로 실행 중인지 여부 |
| `commentsVisible` | `Bool` | 댓글 표시 토글 |
| `toolbarEnabled` | `Bool` | 서식 툴바가 표시되는지 여부 |

### FastCommentsFeedSDK Published 속성

| Property | Type | Description |
|----------|------|-------------|
| `feedPosts` | `[FeedPost]` | 현재 로드된 피드 게시물 |
| `hasMore` | `Bool` | 추가 페이지가 존재하는지 여부 |
| `currentUser` | `UserSessionInfo?` | 현재 인증된 사용자 |
| `blockingErrorMessage` | `String?` | 차단 오류 메시지 |
| `isLoading` | `Bool` | 네트워크 요청이 진행 중인지 여부 |
| `newPostsCount` | `Int` | 마지막 로드 이후 새 게시물 수 |

### 댓글 트리

댓글 트리는 `sdk.commentsTree`를 통해 접근할 수 있습니다:

```swift
// 렌더링을 위한 표시된 노드의 평탄(Flat) 목록
sdk.commentsTree.visibleNodes

// ID로 댓글 조회
sdk.commentsTree.commentsById["comment-id"]
```

---
---