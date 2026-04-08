Besleme sistemi kendi görünümüne sahip ayrı bir SDKdır (`FastCommentsFeedSDK`).

### Beslemeyi Yükleme ve Gösterme

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
                // Paylaşım sayfasını göster
            }
            .onUserClick { context, userInfo, source in
                // Kullanıcı profiline yönlendir
            }
            .onMediaClick { mediaItem, index in
                // Tam ekran resim görüntüleyicisini göster
            }
            .task {
                try? await sdk.loadIfNeeded()
            }
    }
}
```

Besleme görünümü otomatik olarak pull-to-refresh ve sonsuz kaydırma içerir.
Var olan veya geri yüklenmiş bir beslemenin sayfa 1'e sıfırlanmaması için ekran yaşam döngüsüne yeniden girişte `loadIfNeeded()` kullanın.

### Gönderi Oluşturma

Use `FeedPostCreateView` to present a post creation form:

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

### Gönderilere Tepki Verme

The SDK handles reactions with optimistic updates:

```swift
try await sdk.reactPost(postId: post.id, reactionType: "l")

// Tepki durumunu kontrol edin
let hasLiked = sdk.hasUserReacted(postId: post.id, reactType: "l")
let likeCount = sdk.getLikeCount(postId: post.id)
```

### Bir Gönderide Yorumları Açma

Use `CommentsSheet` to display comments for a feed post. It creates a `FastCommentsSDK` instance internally using the feed SDK's config:

```swift
.sheet(item: $commentsPost) { post in
    CommentsSheet(post: post, feedSDK: sdk, onUserClick: { context, userInfo, source in
        // Kullanıcı tıklamasını işle
    })
}
```

Not: `.sheet(item:)` için `FeedPost` `Identifiable` protokolüne uymalıdır. Bu uzantıyı ekleyin:

```swift
extension FeedPost: @retroactive Identifiable {}
```

### Etiket Tabanlı Besleme Filtreleme

Implement the `TagSupplier` protocol to filter feed posts by tags:

```swift
struct TeamTagSupplier: TagSupplier {
    func getTags(currentUser: UserSessionInfo?) -> [String]? {
        guard let user = currentUser else { return nil }
        return ["team:\(user.id ?? "")", "public"]
    }
}

sdk.tagSupplier = TeamTagSupplier()
```

Filtrelenmemiş global bir besleme için `nil` döndürün.

### Besleme Durumunu Kaydetme ve Geri Yükleme

Preserve pagination state across view lifecycle events:

```swift
let state = sdk.savePaginationState()
// Later...
sdk.restorePaginationState(state)
try? await sdk.loadIfNeeded()
```

Ekranınız geçici olarak kaybolursa, besleme görünümü canlı güncellemeleri otomatik olarak duraklatır ve tekrar görünüşte yüklü gönderileri temizlemeden devam ettirir. SDK örneğiyle işiniz gerçekten bittiğinde yalnızca `sdk.cleanup()` çağırın.

### Gönderileri Silme

```swift
sdk.onPostDeleted = { postId in
    print("Post \(postId) was deleted")
}
```