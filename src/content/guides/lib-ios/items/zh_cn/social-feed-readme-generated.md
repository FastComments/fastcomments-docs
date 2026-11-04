信息流系统是一个独立的 SDK(`FastCommentsFeedSDK`),拥有自己的视图。

### 加载和显示信息流

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
                // 跳转到用户资料页
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

信息流视图自动包含下拉刷新和无限滚动。
在屏幕生命周期重新进入时使用 `loadIfNeeded()`,这样已存在或已恢复的信息流不会重置回第 1 页。

### 创建帖子

使用 `FeedPostCreateView` 来展示帖子创建表单:

```swift
@State private var showCreatePost = false

// 在你的视图 body 中:
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

### 对帖子做出反应

SDK 通过乐观更新来处理反应:

```swift
try await sdk.reactPost(postId: post.id, reactionType: "l")

// 检查反应状态
let hasLiked = sdk.hasUserReacted(postId: post.id, reactType: "l")
let likeCount = sdk.getLikeCount(postId: post.id)
```

### 在帖子上打开评论

使用 `CommentsSheet` 来显示信息流帖子的评论。它会在内部使用信息流 SDK 的配置创建一个 `FastCommentsSDK` 实例:

```swift
.sheet(item: $commentsPost) { post in
    CommentsSheet(post: post, feedSDK: sdk, onUserClick: { context, userInfo, source in
        // 处理用户点击
    })
}
```

注意:`FeedPost` 必须遵循 `Identifiable` 才能使用 `.sheet(item:)`。添加以下扩展:

```swift
extension FeedPost: @retroactive Identifiable {}
```

### 基于标签的信息流过滤

实现 `TagSupplier` 协议以按标签过滤信息流帖子:

```swift
struct TeamTagSupplier: TagSupplier {
    func getTags(currentUser: UserSessionInfo?) -> [String]? {
        guard let user = currentUser else { return nil }
        return ["team:\(user.id ?? "")", "public"]
    }
}

sdk.tagSupplier = TeamTagSupplier()
```

返回 `nil` 表示不进行过滤的全局信息流。

### 保存和恢复信息流状态

在视图生命周期事件中保留分页状态:

```swift
let state = sdk.savePaginationState()
// 稍后...
sdk.restorePaginationState(state)
try? await sdk.loadIfNeeded()
```

如果你的屏幕暂时消失,信息流视图会自动暂停实时更新,并在重新出现时恢复更新,而不会清除已加载的帖子。只有当你真正不再使用该 SDK 实例时才调用 `sdk.cleanup()`。

### 删除帖子

```swift
sdk.onPostDeleted = { postId in
    print("Post \(postId) was deleted")
}
```