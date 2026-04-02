---
### Edit

```swift
try await sdk.editComment(commentId: commentId, newText: "Updated text")
```

伺服器會重新渲染 HTML。本地留言會自動更新。

### Delete

```swift
try await sdk.deleteComment(commentId: commentId)
```

刪除留言也會從本地樹狀結構中移除其子孫。

兩個操作在 UI 的留言快顯選單中可用，當前使用者為留言作者（或網站管理員）時。

---
---