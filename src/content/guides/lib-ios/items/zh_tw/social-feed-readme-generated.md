The feed system is a separate SDK (`FastCommentsFeedSDK`) with its own view.

### 載入與顯示動態

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
                // 導覽到使用者個人頁面
            }
            .onMediaClick { mediaItem, index in
                // 顯示全螢幕影像檢視器
            }
            .task {
                try? await sdk.loadIfNeeded()
            }
    }
}
```

動態視圖會自動包含下拉重新整理與無限滾動功能。
在畫面生命週期重新進入時，使用 `loadIfNeeded()`，以避免既有或還原的動態重設回第 1 頁。

### 建立貼文

使用 `FeedPostCreateView` 顯示貼文建立表單：

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

### 對貼文的反應

SDK 使用樂觀更新處理反應：

```swift
try await sdk.reactPost(postId: post.id, reactionType: "l")

// Check reaction state
let hasLiked = sdk.hasUserReacted(postId: post.id, reactType: "l")
let likeCount = sdk.getLikeCount(postId: post.id)
```

### 在貼文上開啟留言

使用 `CommentsSheet` 顯示指定動態貼文的留言。它會內部使用 feed SDK 的設定建立一個 `FastCommentsSDK` 實例：

```swift
.sheet(item: $commentsPost) { post in
    CommentsSheet(post: post, feedSDK: sdk, onUserClick: { context, userInfo, source in
        // 處理使用者點擊
    })
}
```

注意：`FeedPost` 必須符合 `Identifiable`，以用於 `.sheet(item:)`。加入此擴充：

```swift
extension FeedPost: @retroactive Identifiable {}
```

### 基於標籤的動態過濾

實作 `TagSupplier` 協定以依標籤過濾動態貼文：

```swift
struct TeamTagSupplier: TagSupplier {
    func getTags(currentUser: UserSessionInfo?) -> [String]? {
        guard let user = currentUser else { return nil }
        return ["team:\(user.id ?? "")", "public"]
    }
}

sdk.tagSupplier = TeamTagSupplier()
```

對於不過濾的全域動態，回傳 `nil`。

### 儲存與還原動態狀態

在視圖生命週期事件中保留分頁狀態：

```swift
let state = sdk.savePaginationState()
// Later...
sdk.restorePaginationState(state)
try? await sdk.loadIfNeeded()
```

如果你的畫面暫時消失，動態視圖會自動暫停即時更新，並在重新出現時恢復，而不會清除已載入的貼文。僅在確實不再使用該 SDK 實例時才呼叫 `sdk.cleanup()`。

### 刪除貼文

```swift
sdk.onPostDeleted = { postId in
    print("Post \(postId) was deleted")
}
```

---
---