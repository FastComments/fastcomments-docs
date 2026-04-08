The feed system is a separate SDK (`FastCommentsFeedSDK`) with its own view.

### 加载与显示 Feed

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
                // 显示分享表单
            }
            .onUserClick { context, userInfo, source in
                // 跳转到用户资料
            }
            .onMediaClick { mediaItem, index in
                // 显示全屏图片查看器
            }
            .task {
                try? await sdk.loadIfNeeded()
            }
    }
}
```

Feed 视图会自动包含下拉刷新和无限滚动。  
在屏幕生命周期重新进入时使用 `loadIfNeeded()`，以便已有或恢复的 Feed 不会重置回第 1 页。

### 创建帖子

使用 `FeedPostCreateView` 来展示发帖表单：

```swift
@State private var showCreatePost = false

// 在你的视图主体中:
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

### 对帖子作出反应

SDK 使用乐观更新来处理反应：

```swift
try await sdk.reactPost(postId: post.id, reactionType: "l")

// Check reaction state
let hasLiked = sdk.hasUserReacted(postId: post.id, reactType: "l")
let likeCount = sdk.getLikeCount(postId: post.id)
```

### 打开帖子的评论

使用 `CommentsSheet` 来显示某个 Feed 帖子的评论。它会内部使用 Feed SDK 的配置创建一个 `FastCommentsSDK` 实例：

```swift
.sheet(item: $commentsPost) { post in
    CommentsSheet(post: post, feedSDK: sdk, onUserClick: { context, userInfo, source in
        // 处理用户点击
    })
}
```

注意：对于 `.sheet(item:)`，`FeedPost` 必须遵循 `Identifiable`。添加如下扩展：

```swift
extension FeedPost: @retroactive Identifiable {}
```

### 基于标签的 Feed 过滤

实现 `TagSupplier` 协议以按标签过滤 Feed 帖子：

```swift
struct TeamTagSupplier: TagSupplier {
    func getTags(currentUser: UserSessionInfo?) -> [String]? {
        guard let user = currentUser else { return nil }
        return ["team:\(user.id ?? "")", "public"]
    }
}

sdk.tagSupplier = TeamTagSupplier()
```

若要获得不带过滤的全局 Feed，请返回 `nil`。

### 保存与恢复 Feed 状态

在视图生命周期事件中保留分页状态：

```swift
let state = sdk.savePaginationState()
// Later...
sdk.restorePaginationState(state)
try? await sdk.loadIfNeeded()
```

如果你的屏幕短暂消失，Feed 视图会自动暂停实时更新，并在重新出现时恢复，而不会清除已加载的帖子。仅在真正不再使用该 SDK 实例时才调用 `sdk.cleanup()`。

### 删除帖子

```swift
sdk.onPostDeleted = { postId in
    print("Post \(postId) was deleted")
}
```

---
---