フィードシステムは独立した SDK (`FastCommentsFeedSDK`) で、専用のビューを持ちます。

### フィードの読み込みと表示

```swift
struct FeedPage: View {
    @StateObject private var sdk: FastCommentsFeedSDK = {
        let config = FastCommentsWidgetConfig(
            tenantId: "YOUR_TENANT_ID",
            urlId: "my-feed",
            sso: ssoToken
        )
        return FastCommentsFeedSDK(config: config)
    }()

    @State private var commentsPost: FeedPost?

    var body: some View {
        FastCommentsFeedView(sdk: sdk)
            .onPostSelected { post in
                commentsPost = post
            }
            .onCommentsRequested { post in
                commentsPost = post
            }
            .onSharePost { post in
                // 共有シートを表示する
            }
            .onUserClick { context, userInfo, source in
                // ユーザープロファイルへ遷移する
            }
            .onMediaClick { mediaItem, index in
                // フルスクリーンの画像ビューアを表示する
            }
            .task {
                try? await sdk.loadIfNeeded()
            }
    }
}
```

フィードビューにはプル・トゥ・リフレッシュと無限スクロールが自動的に含まれます。
画面のライフサイクルに戻る場合は `loadIfNeeded()` を使用してください。既存または復元されたフィードがページ1にリセットされるのを防ぎます。

### 投稿の作成

投稿作成フォームを表示するには `FeedPostCreateView` を使用します:

```swift
@State private var showCreatePost = false

// In your view body:
.sheet(isPresented: $showCreatePost) {
    FeedPostCreateView(
        sdk: sdk,
        onPostCreated: { post in
            showCreatePost = false
            Task { try? await sdk.refresh() }
        },
        onCancelled: {
            showCreatePost = false
        }
    )
}
```

### 投稿へのリアクション

SDKは楽観的更新でリアクションを処理します:

```swift
try await sdk.reactPost(postId: post.id, reactionType: "l")

// Check reaction state
let hasLiked = sdk.hasUserReacted(postId: post.id, reactType: "l")
let likeCount = sdk.getLikeCount(postId: post.id)
```

### 投稿のコメントを開く

フィード投稿のコメントを表示するには `CommentsSheet` を使用します。これはフィードSDKの設定を使用して内部で `FastCommentsSDK` インスタンスを作成します:

```swift
.sheet(item: $commentsPost) { post in
    CommentsSheet(post: post, feedSDK: sdk, onUserClick: { context, userInfo, source in
        // ユーザークリックを処理する
    })
}
```

注意: `.sheet(item:)` には `FeedPost` が `Identifiable` に準拠している必要があります。この拡張を追加してください:

```swift
extension FeedPost: @retroactive Identifiable {}
```

### タグベースのフィードフィルタリング

タグでフィード投稿をフィルタリングするには `TagSupplier` プロトコルを実装します:

```swift
struct TeamTagSupplier: TagSupplier {
    func getTags(currentUser: UserSessionInfo?) -> [String]? {
        guard let user = currentUser else { return nil }
        return ["team:\(user.id ?? "")", "public"]
    }
}

sdk.tagSupplier = TeamTagSupplier()
```

フィルタリングされていないグローバルフィードの場合は `nil` を返してください。

### フィード状態の保存と復元

ビューのライフサイクルイベント間でページネーション状態を保持します:

```swift
let state = sdk.savePaginationState()
// Later...
sdk.restorePaginationState(state)
try? await sdk.loadIfNeeded()
```

画面が一時的に非表示になる場合、フィードビューはライブ更新を自動的に一時停止し、再表示時に読み込んだ投稿をクリアせずに再開します。SDKインスタンスを本当に使い終わったときだけ `sdk.cleanup()` を呼び出してください。

### 投稿の削除

```swift
sdk.onPostDeleted = { postId in
    print("Post \(postId) was deleted")
}
```