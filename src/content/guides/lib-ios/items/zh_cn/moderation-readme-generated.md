### 所有用户可用的操作

- **Flag/Unflag** -- 将评论举报以供审查

```swift
try await sdk.flagComment(commentId: commentId)
try await sdk.unflagComment(commentId: commentId)
```

- **Block/Unblock** -- 对观看者隐藏某用户的所有评论（仅对当前观看者）

```swift
try await sdk.blockUser(commentId: commentId)
try await sdk.unblockUser(commentId: commentId)
```

### 仅管理员可用的操作

- **Pin/Unpin** -- 将评论置顶

```swift
try await sdk.pinComment(commentId: commentId)
try await sdk.unpinComment(commentId: commentId)
```

- **Lock/Unlock** -- 阻止对评论的新的回复，并在解锁前阻止编辑和删除（适用于所有人，包括版主）

```swift
try await sdk.lockComment(commentId: commentId)
try await sdk.unlockComment(commentId: commentId)
```

所有的审核操作也可以通过 UI 中的评论上下文菜单使用。管理员操作仅在当前用户是站点管理员时出现（通过 SSO `isAdmin` 标志或仪表盘配置设置）。