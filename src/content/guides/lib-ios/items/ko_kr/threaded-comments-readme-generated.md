### 기본 사용법

```swift
struct CommentsPage: View {
    @StateObject private var sdk = FastCommentsSDK(
        config: FastCommentsWidgetConfig(
            tenantId: "YOUR_TENANT_ID",
            urlId: "article-42",
            url: "https://example.com/article/42",
            pageTitle: "Article Title"
        )
    )

    var body: some View {
        FastCommentsView(sdk: sdk)
            .task {
                try? await sdk.load()
            }
    }
}
```

### 투표 스타일

기본 투표 스타일은 상/하 화살표를 표시합니다. 하트 스타일 투표를 사용하려면 `._1` 을 전달하세요:

```swift
FastCommentsView(sdk: sdk, voteStyle: ._1)
```

| Style | Appearance |
|-------|------------|
| `._0` | 상/하 화살표 버튼과 합계 카운트 |
| `._1` | 하나의 하트 버튼과 카운트 |

### 이벤트 콜백

모디파이어 스타일의 콜백을 사용하여 사용자 상호작용을 처리하세요:

```swift
FastCommentsView(sdk: sdk)
    .onCommentPosted { comment in
        print("New comment: \(comment.commentHTML)")
    }
    .onReplyClick { renderableComment in
        print("Replying to: \(renderableComment.comment.id)")
    }
    .onUserClick { context, userInfo, source in
        // source는 .name 또는 .avatar 입니다
        print("Tapped \(userInfo.displayName)")
    }
```

### 테마 적용

SwiftUI 환경을 통해 테마를 전달하세요:

```swift
FastCommentsView(sdk: sdk)
    .fastCommentsTheme(myTheme)
    .task { try? await sdk.load() }
```

또는 SDK에 직접 설정하세요:

```swift
sdk.theme = FastCommentsTheme.modern
```

### 정렬 방향

```swift
sdk.defaultSortDirection = .nf  // 최신순 (기본값)
sdk.defaultSortDirection = .of  // 오래된 순
sdk.defaultSortDirection = .mr  // 관련성 높은 순
```

---
---