### Edit

```swift
try await sdk.editComment(commentId: commentId, newText: "Updated text")
```

服务器会重新渲染 HTML。本地评论会自动更新。

### Delete

```swift
try await sdk.deleteComment(commentId: commentId)
```

删除评论也会从本地树中移除其子代。

当当前用户是评论作者（或网站管理员）时，这两项操作可通过 UI 中的评论上下文菜单使用。

---
---