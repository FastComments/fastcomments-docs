피드 시스템은 자체 뷰를 가진 별도의 SDK (`FastCommentsFeedSDK`)입니다.

### 피드 로드 및 표시

```swift
struct FeedPage: View {
    @StateObject private var sdk: FastCommentsFeedSDK = {
        let config = FastCommentsWidgetConfig(
            tenantId: "YOUR_TENANT_ID",
            urlId: "my-feed",
            sso: ssoToken
        )
        return FastCommentsFeedSDK(config: config)
    }()

    @State private var commentsPost: FeedPost?

    var body: some View {
        FastCommentsFeedView(sdk: sdk)
            .onPostSelected { post in
                commentsPost = post
            }
            .onCommentsRequested { post in
                commentsPost = post
            }
            .onSharePost { post in
                // 공유 시트 표시
            }
            .onUserClick { context, userInfo, source in
                // 사용자 프로필로 이동
            }
            .onMediaClick { mediaItem, index in
                // 전체 화면 이미지 뷰어 표시
            }
            .task {
                try? await sdk.load()
            }
    }
}
```

피드 뷰는 자동으로 풀 투 리프레시(pull-to-refresh)와 무한 스크롤을 포함합니다.

### 게시물 생성

게시물 작성 폼을 표시하려면 `FeedPostCreateView`를 사용하세요:

```swift
@State private var showCreatePost = false

// In your view body:
.sheet(isPresented: $showCreatePost) {
    FeedPostCreateView(
        sdk: sdk,
        onPostCreated: { post in
            showCreatePost = false
            Task { try? await sdk.refresh() }
        },
        onCancelled: {
            showCreatePost = false
        }
    )
}
```

### 게시물에 반응하기

SDK는 낙관적 업데이트(optimistic updates)로 반응을 처리합니다:

```swift
try await sdk.reactPost(postId: post.id, reactionType: "l")

// 반응 상태 확인
let hasLiked = sdk.hasUserReacted(postId: post.id, reactType: "l")
let likeCount = sdk.getLikeCount(postId: post.id)
```

### 게시물의 댓글 열기

피드 게시물의 댓글을 표시하려면 `CommentsSheet`를 사용하세요. 이 컴포넌트는 내부적으로 feed SDK의 구성(config)을 사용하여 `FastCommentsSDK` 인스턴스를 생성합니다:

```swift
.sheet(item: $commentsPost) { post in
    CommentsSheet(post: post, feedSDK: sdk, onUserClick: { context, userInfo, source in
        // 사용자 클릭 처리
    })
}
```

참고: `FeedPost`가 `.sheet(item:)`에 사용되려면 `Identifiable`을 준수해야 합니다. 다음 익스텐션을 추가하세요:

```swift
extension FeedPost: @retroactive Identifiable {}
```

### 태그 기반 피드 필터링

태그로 피드 게시물을 필터링하려면 `TagSupplier` 프로토콜을 구현하세요:

```swift
struct TeamTagSupplier: TagSupplier {
    func getTags(currentUser: UserSessionInfo?) -> [String]? {
        guard let user = currentUser else { return nil }
        return ["team:\(user.id ?? "")", "public"]
    }
}

sdk.tagSupplier = TeamTagSupplier()
```

필터링되지 않은 전역 피드의 경우 `nil`을 반환하세요.

### 피드 상태 저장 및 복원

뷰 수명 주기 이벤트 전반에 걸쳐 페이지네이션 상태를 보존하세요:

```swift
let state = sdk.savePaginationState()
// Later...
sdk.restorePaginationState(state)
```

### 게시물 삭제

```swift
sdk.onPostDeleted = { postId in
    print("Post \(postId) was deleted")
}
```