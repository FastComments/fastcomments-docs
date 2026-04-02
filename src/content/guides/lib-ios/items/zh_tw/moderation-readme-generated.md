### 所有使用者可用的操作

- **檢舉/取消檢舉** -- 回報評論以供審查

```swift
try await sdk.flagComment(commentId: commentId)
try await sdk.unflagComment(commentId: commentId)
```

- **封鎖/取消封鎖** -- 隱藏該使用者的所有評論（以檢視者為單位）

```swift
try await sdk.blockUser(commentId: commentId)
try await sdk.unblockUser(commentId: commentId)
```

### 僅限管理員的操作

- **置頂/取消置頂** -- 將評論釘選於討論串頂端

```swift
try await sdk.pinComment(commentId: commentId)
try await sdk.unpinComment(commentId: commentId)
```

- **鎖定/解鎖** -- 防止在該評論上新增回覆

```swift
try await sdk.lockComment(commentId: commentId)
try await sdk.unlockComment(commentId: commentId)
```

所有管理操作也可以透過 UI 中的評論快顯選單執行。僅當目前使用者為站點管理員（透過 SSO `isAdmin` 標誌或在儀表板設定）時，才會顯示管理員操作。

---
---