### 編集

```swift
try await sdk.editComment(commentId: commentId, newText: "Updated text")
```

サーバーがHTMLを再レンダリングします。ローカルのコメントは自動的に更新されます。

### 削除

```swift
try await sdk.deleteComment(commentId: commentId)
```

コメントを削除すると、その子孫もローカルツリーから削除されます。

これらの操作は、現在のユーザーがコメントの作成者（またはサイト管理者）の場合、UIのコメントコンテキストメニューから利用できます。

---
---