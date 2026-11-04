`FastCommentsSDK` 和 `FastCommentsFeedSDK` 都是带有 `@Published` 属性的 `ObservableObject` 类。你可以在 SwiftUI 视图中观察这些属性，以实现响应式 UI 更新。

### FastCommentsSDK 的 Published 属性

| 属性 | 类型 | 描述 |
|----------|------|-------------|
| `commentCountOnServer` | `Int` | 服务器上的评论总数 |
| `newRootCommentCount` | `Int` | 缓冲的新评论（当 `showLiveRightAway` 为 false 时） |
| `currentUser` | `UserSessionInfo?` | 当前已认证的用户 |
| `isSiteAdmin` | `Bool` | 当前用户是否为站点管理员 |
| `isClosed` | `Bool` | 评论线程是否已关闭 |
| `hasBillingIssue` | `Bool` | 是否存在计费问题 |
| `isLoading` | `Bool` | 是否正在进行网络请求 |
| `hasMore` | `Bool` | 是否存在更多评论页面 |
| `blockingErrorMessage` | `String?` | 阻止 UI 正常运行的错误 |
| `warningMessage` | `String?` | 非阻塞性警告消息 |
| `isDemo` | `Bool` | 是否在演示模式下运行 |
| `commentsVisible` | `Bool` | 评论可见性的开关 |
| `toolbarEnabled` | `Bool` | 是否显示格式化工具栏 |

### FastCommentsFeedSDK 的 Published 属性

| 属性 | 类型 | 描述 |
|----------|------|-------------|
| `feedPosts` | `[FeedPost]` | 当前已加载的动态帖子 |
| `hasMore` | `Bool` | 是否存在更多页面 |
| `currentUser` | `UserSessionInfo?` | 当前已认证的用户 |
| `blockingErrorMessage` | `String?` | 阻塞性错误消息 |
| `isLoading` | `Bool` | 是否正在进行网络请求 |
| `newPostsCount` | `Int` | 自上次加载以来的新帖子数量 |

### 评论树

评论树可通过 `sdk.commentsTree` 访问：

```swift
// 用于渲染的可见节点扁平列表
sdk.commentsTree.visibleNodes

// 按 ID 查找评论
sdk.commentsTree.commentsById["comment-id"]
```