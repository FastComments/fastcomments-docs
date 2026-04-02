Both `FastCommentsSDK` and `FastCommentsFeedSDK` are `ObservableObject` classes with `@Published` properties. You can observe these in your SwiftUI views for reactive UI updates.

### FastCommentsSDK 已发布的属性

| Property | Type | Description |
|----------|------|-------------|
| `commentCountOnServer` | `Int` | 服务器上的评论总数 |
| `newRootCommentCount` | `Int` | 缓冲的新根评论数（当 `showLiveRightAway` 为 false 时） |
| `currentUser` | `UserSessionInfo?` | 当前已认证用户 |
| `isSiteAdmin` | `Bool` | 当前用户是否为站点管理员 |
| `isClosed` | `Bool` | 评论线程是否已关闭 |
| `hasBillingIssue` | `Bool` | 是否存在计费问题 |
| `isLoading` | `Bool` | 是否有网络请求正在进行 |
| `hasMore` | `Bool` | 是否存在更多评论页 |
| `blockingErrorMessage` | `String?` | 阻止 UI 正常工作的错误 |
| `warningMessage` | `String?` | 非阻塞的警告信息 |
| `isDemo` | `Bool` | 是否处于演示模式 |
| `commentsVisible` | `Bool` | 评论可见性的切换 |
| `toolbarEnabled` | `Bool` | 是否显示格式化工具栏 |

### FastCommentsFeedSDK 已发布的属性

| Property | Type | Description |
|----------|------|-------------|
| `feedPosts` | `[FeedPost]` | 当前加载的 Feed 帖子 |
| `hasMore` | `Bool` | 是否存在更多页 |
| `currentUser` | `UserSessionInfo?` | 当前已认证用户 |
| `blockingErrorMessage` | `String?` | 阻塞性错误信息 |
| `isLoading` | `Bool` | 是否有网络请求正在进行 |
| `newPostsCount` | `Int` | 自上次加载以来的新帖子数量 |

### 评论树

可以通过 `sdk.commentsTree` 访问评论树：

```swift
// Flat list of visible nodes for rendering
sdk.commentsTree.visibleNodes

// Lookup a comment by ID
sdk.commentsTree.commentsById["comment-id"]
```

---
---