The feed 系统是一个独立的 SDK (`FastCommentsFeedSDK`)，具有自己的视图。

### 加载和显示 Feed

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
                // 显示分享面板
            }
            .onUserClick { context, userInfo, source in
                // 导航到用户资料
            }
            .onMediaClick { mediaItem, index in
                // 显示全屏图片查看器
            }
            .task {
                try? await sdk.load()
            }
    }
}
```

Feed 视图自动包含下拉刷新和无限滚动。

### 创建帖子

使用 `FeedPostCreateView` 来呈现帖子创建表单：

```swift
@State private var showCreatePost = false

// 在你的视图主体中：
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

### 对帖子进行反应

SDK 使用乐观更新来处理反应：

```swift
try await sdk.reactPost(postId: post.id, reactionType: "l")

// Check reaction state
let hasLiked = sdk.hasUserReacted(postId: post.id, reactType: "l")
let likeCount = sdk.getLikeCount(postId: post.id)
```

### 打开帖子的评论

使用 `CommentsSheet` 来显示某个 feed 帖子的评论。它会使用 feed SDK 的配置在内部创建一个 `FastCommentsSDK` 实例：

```swift
.sheet(item: $commentsPost) { post in
    CommentsSheet(post: post, feedSDK: sdk, onUserClick: { context, userInfo, source in
        // 处理用户点击
    })
}
```

注意：为了使用 `.sheet(item:)`，`FeedPost` 必须遵循 `Identifiable`。添加此扩展：

```swift
extension FeedPost: @retroactive Identifiable {}
```

### 基于标签的 Feed 过滤

实现 `TagSupplier` 协议以按标签过滤 feed 帖子：

```swift
struct TeamTagSupplier: TagSupplier {
    func getTags(currentUser: UserSessionInfo?) -> [String]? {
        guard let user = currentUser else { return nil }
        return ["team:\(user.id ?? "")", "public"]
    }
}

sdk.tagSupplier = TeamTagSupplier()
```

对于未过滤的全局 Feed，返回 `nil`。

### 保存和恢复 Feed 状态

在视图生命周期事件之间保留分页状态：

```swift
let state = sdk.savePaginationState()
// Later...
sdk.restorePaginationState(state)
```

### 删除帖子

```swift
sdk.onPostDeleted = { postId in
    print("Post \(postId) was deleted")
}
```