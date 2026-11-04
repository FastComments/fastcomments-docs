動態消息系統是一個獨立的 SDK(`FastCommentsFeedSDK`),擁有自己的視圖。

### 載入與顯示動態消息

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
                // 顯示分享面板
            }
            .onUserClick { context, userInfo, source in
                // 導覽至使用者個人資料
            }
            .onMediaClick { mediaItem, index in
                // 顯示全螢幕圖片檢視器
            }
            .task {
                try? await sdk.loadIfNeeded()
            }
    }
}
```

動態消息視圖會自動包含下拉重新整理與無限捲動。
在畫面生命週期重新進入時請使用 `loadIfNeeded()`,這樣已存在或已還原的動態消息就不會重設回第 1 頁。

### 建立貼文

使用 `FeedPostCreateView` 來呈現貼文建立表單:

```swift
@State private var showCreatePost = false

// 在你的視圖 body 中:
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

### 對貼文做出反應

SDK 透過樂觀更新來處理反應:

```swift
try await sdk.reactPost(postId: post.id, reactionType: "l")

// 檢查反應狀態
let hasLiked = sdk.hasUserReacted(postId: post.id, reactType: "l")
let likeCount = sdk.getLikeCount(postId: post.id)
```

### 開啟貼文的留言

使用 `CommentsSheet` 來顯示動態消息貼文的留言。它會在內部使用動態消息 SDK 的設定來建立 `FastCommentsSDK` 實例:

```swift
.sheet(item: $commentsPost) { post in
    CommentsSheet(post: post, feedSDK: sdk, onUserClick: { context, userInfo, source in
        // 處理使用者點擊
    })
}
```

注意:`FeedPost` 必須遵循 `Identifiable` 才能使用 `.sheet(item:)`。請加入此擴充:

```swift
extension FeedPost: @retroactive Identifiable {}
```

### 基於標籤的動態消息篩選

實作 `TagSupplier` 協定以依標籤篩選動態消息貼文:

```swift
struct TeamTagSupplier: TagSupplier {
    func getTags(currentUser: UserSessionInfo?) -> [String]? {
        guard let user = currentUser else { return nil }
        return ["team:\(user.id ?? "")", "public"]
    }
}

sdk.tagSupplier = TeamTagSupplier()
```

若要顯示未篩選的全域動態消息,請回傳 `nil`。

### 儲存與還原動態消息狀態

在視圖生命週期事件之間保留分頁狀態:

```swift
let state = sdk.savePaginationState()
// 稍後...
sdk.restorePaginationState(state)
try? await sdk.loadIfNeeded()
```

如果你的畫面暫時消失,動態消息視圖會自動暫停即時更新,並在重新出現時恢復更新,且不會清除已載入的貼文。只有當你確定不再使用該 SDK 實例時,才呼叫 `sdk.cleanup()`。

### 刪除貼文

```swift
sdk.onPostDeleted = { postId in
    print("Post \(postId) was deleted")
}
```

---