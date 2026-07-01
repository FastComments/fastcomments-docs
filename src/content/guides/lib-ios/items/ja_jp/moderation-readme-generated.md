### すべてのユーザーが利用できるアクション

- **Flag/Unflag** -- コメントをレビューのために報告する

```swift
try await sdk.flagComment(commentId: commentId)
try await sdk.unflagComment(commentId: commentId)
```

- **Block/Unblock** -- ユーザーからのすべてのコメントを非表示にする（閲覧者ごと）

```swift
try await sdk.blockUser(commentId: commentId)
try await sdk.unblockUser(commentId: commentId)
```

### 管理者のみ利用できるアクション

- **Pin/Unpin** -- スレッドのトップにコメントを固定する

```swift
try await sdk.pinComment(commentId: commentId)
try await sdk.unpinComment(commentId: commentId)
```

- **Lock/Unlock** -- コメントへの新しい返信を防止し、ロックが解除されるまで編集と削除をブロックする（モデレーターを含む全員に適用）

```swift
try await sdk.lockComment(commentId: commentId)
try await sdk.unlockComment(commentId: commentId)
```

すべてのモデレーションアクションは、UI のコメントコンテキストメニューからも利用できます。管理者アクションは、現在のユーザーがサイト管理者（SSO の `isAdmin` フラグまたはダッシュボード設定で設定）である場合にのみ表示されます。