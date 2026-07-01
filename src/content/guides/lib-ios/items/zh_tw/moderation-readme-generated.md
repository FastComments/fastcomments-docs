### 所有使用者可執行的操作

- **Flag/Unflag** -- 檢舉評論以供審核

```swift
try await sdk.flagComment(commentId: commentId)
try await sdk.unflagComment(commentId: commentId)
```

- **Block/Unblock** -- 對特定檢視者隱藏該使用者的所有評論

```swift
try await sdk.blockUser(commentId: commentId)
try await sdk.unblockUser(commentId: commentId)
```

### 僅限管理員的操作

- **Pin/Unpin** -- 將評論固定在討論串頂部

```swift
try await sdk.pinComment(commentId: commentId)
try await sdk.unpinComment(commentId: commentId)
```

- **Lock/Unlock** -- 阻止對評論的回覆，並在解除鎖定前禁止編輯與刪除（適用於所有人，包括版主）

```swift
try await sdk.lockComment(commentId: commentId)
try await sdk.unlockComment(commentId: commentId)
```

所有的審查動作也可在 UI 中的評論上下文功能表取得。僅當目前使用者是站台管理員時（透過 SSO 的 `isAdmin` 標誌或儀表板設定），才會顯示管理員動作。