### 所有用户可用的操作

- **标记/取消标记** -- 报告一条评论以供审核

```swift
try await sdk.flagComment(commentId: commentId)
try await sdk.unflagComment(commentId: commentId)
```

- **屏蔽/取消屏蔽** -- 隐藏来自某用户的所有评论（针对每个查看者）

```swift
try await sdk.blockUser(commentId: commentId)
try await sdk.unblockUser(commentId: commentId)
```

### 仅限管理员的操作

- **置顶/取消置顶** -- 将评论固定到主题顶部

```swift
try await sdk.pinComment(commentId: commentId)
try await sdk.unpinComment(commentId: commentId)
```

- **锁定/解锁** -- 阻止对评论进行新的回复

```swift
try await sdk.lockComment(commentId: commentId)
try await sdk.unlockComment(commentId: commentId)
```

所有的审核操作也可以通过 UI 中的评论上下文菜单使用。管理操作仅在当前用户为站点管理员时显示（通过 SSO `isAdmin` 标志或仪表板配置设置）。

---
---