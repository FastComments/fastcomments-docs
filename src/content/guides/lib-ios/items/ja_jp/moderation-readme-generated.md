### すべてのユーザーが利用できる操作

- **フラグ／フラグ解除** -- コメントを審査のために報告する

```swift
try await sdk.flagComment(commentId: commentId)
try await sdk.unflagComment(commentId: commentId)
```

- **ブロック／ブロック解除** -- ユーザーのコメントをすべて非表示にする（閲覧者単位）

```swift
try await sdk.blockUser(commentId: commentId)
try await sdk.unblockUser(commentId: commentId)
```

### 管理者専用の操作

- **ピン留め／ピン解除** -- コメントをスレッドの先頭に固定する

```swift
try await sdk.pinComment(commentId: commentId)
try await sdk.unpinComment(commentId: commentId)
```

- **ロック／ロック解除** -- コメントへの新しい返信を禁止する

```swift
try await sdk.lockComment(commentId: commentId)
try await sdk.unlockComment(commentId: commentId)
```

すべてのモデレーション操作は、UI のコメントコンテキストメニューからも利用できます。管理者向けの操作は、現在のユーザーがサイト管理者である場合にのみ表示されます（SSO の `isAdmin` フラグまたはダッシュボードの設定で指定）。 

---
---