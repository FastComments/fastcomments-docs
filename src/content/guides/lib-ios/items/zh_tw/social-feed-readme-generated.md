Feed 系統是獨立的 SDK（`FastCommentsFeedSDK`），並具有自己的檢視。

### 載入並顯示 Feed

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
                // 顯示分享畫面
            }
            .onUserClick { context, userInfo, source in
                // 導覽至使用者檔案
            }
            .onMediaClick { mediaItem, index in
                // 顯示全螢幕圖像檢視器
            }
            .task {
                try? await sdk.load()
            }
    }
}
```

Feed 檢視會自動包含下拉重新整理與無限滾動。

### 建立文章

使用 `FeedPostCreateView` 顯示文章建立表單：

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

### 對文章做出反應

SDK 使用樂觀更新來處理反應：

```swift
try await sdk.reactPost(postId: post.id, reactionType: "l")

// Check reaction state
let hasLiked = sdk.hasUserReacted(postId: post.id, reactType: "l")
let likeCount = sdk.getLikeCount(postId: post.id)
```

### 在文章上開啟留言

使用 `CommentsSheet` 顯示 Feed 文章的留言。它會使用 Feed SDK 的設定在內部建立一個 `FastCommentsSDK` 實例：

```swift
.sheet(item: $commentsPost) { post in
    CommentsSheet(post: post, feedSDK: sdk, onUserClick: { context, userInfo, source in
        // 處理使用者點擊
    })
}
```

注意：`FeedPost` 必須遵從 `Identifiable`，以便使用 `.sheet(item:)`。新增此延伸：

```swift
extension FeedPost: @retroactive Identifiable {}
```

### 根據標籤篩選 Feed

實作 `TagSupplier` 協定以依標籤篩選 Feed 文章：

```swift
struct TeamTagSupplier: TagSupplier {
    func getTags(currentUser: UserSessionInfo?) -> [String]? {
        guard let user = currentUser else { return nil }
        return ["team:\(user.id ?? "")", "public"]
    }
}

sdk.tagSupplier = TeamTagSupplier()
```

對於未篩選的全域 Feed，回傳 `nil`。

### 儲存與還原 Feed 狀態

在檢視生命週期事件之間保存分頁狀態：

```swift
let state = sdk.savePaginationState()
// Later...
sdk.restorePaginationState(state)
```

### 刪除文章

```swift
sdk.onPostDeleted = { postId in
    print("Post \(postId) was deleted")
}
```