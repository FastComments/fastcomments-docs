`FastCommentsSDK` と `FastCommentsFeedSDK` はどちらも `ObservableObject` クラスで、`@Published` プロパティを持ちます。これらは SwiftUI のビューで監視してリアクティブな UI 更新に利用できます。

### FastCommentsSDK の @Published プロパティ

| プロパティ | 型 | 説明 |
|----------|------|-------------|
| `commentCountOnServer` | `Int` | サーバー上のコメントの総数 |
| `newRootCommentCount` | `Int` | バッファされた新しいコメント（`showLiveRightAway` が false のとき） |
| `currentUser` | `UserSessionInfo?` | 現在認証されているユーザー |
| `isSiteAdmin` | `Bool` | 現在のユーザーがサイト管理者かどうか |
| `isClosed` | `Bool` | コメントスレッドが閉じられているかどうか |
| `hasBillingIssue` | `Bool` | 課金に問題があるかどうか |
| `isLoading` | `Bool` | ネットワークリクエストが進行中かどうか |
| `hasMore` | `Bool` | さらにページが存在するかどうか |
| `blockingErrorMessage` | `String?` | UI の動作を妨げるエラー |
| `warningMessage` | `String?` | ブロッキングしない警告メッセージ |
| `isDemo` | `Bool` | デモモードで実行されているかどうか |
| `commentsVisible` | `Bool` | コメント表示の切り替え |
| `toolbarEnabled` | `Bool` | 書式ツールバーが表示されているかどうか |

### FastCommentsFeedSDK の @Published プロパティ

| プロパティ | 型 | 説明 |
|----------|------|-------------|
| `feedPosts` | `[FeedPost]` | 現在読み込まれているフィード投稿 |
| `hasMore` | `Bool` | さらにページが存在するかどうか |
| `currentUser` | `UserSessionInfo?` | 現在認証されているユーザー |
| `blockingErrorMessage` | `String?` | ブロッキングエラーのメッセージ |
| `isLoading` | `Bool` | ネットワークリクエストが進行中かどうか |
| `newPostsCount` | `Int` | 前回の読み込み以降の新しい投稿数 |

### コメントツリー

コメントツリーは `sdk.commentsTree` でアクセスできます:

```swift
// レンダリング用の表示されているノードのフラットな一覧
sdk.commentsTree.visibleNodes

// IDでコメントを検索
sdk.commentsTree.commentsById["comment-id"]
```

---
---