The feed sistemi kendi görünümüne sahip ayrı bir SDK'dır (`FastCommentsFeedSDK`).

### Feed'i Yükleme ve Görüntüleme

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
                // Kullanıcı profiline git
            }
            .onMediaClick { mediaItem, index in
                // Tam ekran resim görüntüleyiciyi göster
            }
            .task {
                try? await sdk.load()
            }
    }
}
```

Feed görünümü otomatik olarak çekerek yenileme (pull-to-refresh) ve sonsuz kaydırma (infinite scroll) içerir.

### Gönderi Oluşturma

Gönderi oluşturma formunu göstermek için `FeedPostCreateView`'i kullanın:

```swift
@State private var showCreatePost = false

// Görünüm gövdenizde:
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

SDK reaksiyonları iyimser güncellemelerle yönetir:

```swift
try await sdk.reactPost(postId: post.id, reactionType: "l")

// Reaksiyon durumunu kontrol et
let hasLiked = sdk.hasUserReacted(postId: post.id, reactType: "l")
let likeCount = sdk.getLikeCount(postId: post.id)
```

### Bir Gönderide Yorumları Açma

Bir feed gönderisi için yorumları göstermek üzere `CommentsSheet`'i kullanın. İçeride feed SDK'nın konfigürasyonunu kullanarak bir `FastCommentsSDK` örneği oluşturur:

```swift
.sheet(item: $commentsPost) { post in
    CommentsSheet(post: post, feedSDK: sdk, onUserClick: { context, userInfo, source in
        // Kullanıcı tıklamasını işle
    })
}
```

Not: `.sheet(item:)` için `FeedPost`'un `Identifiable` protokolüne uyması gerekir. Bu uzantıyı ekleyin:

```swift
extension FeedPost: @retroactive Identifiable {}
```

### Etiket Tabanlı Feed Filtreleme

Feed gönderilerini etiketlere göre filtrelemek için `TagSupplier` protokolünü uygulayın:

```swift
struct TeamTagSupplier: TagSupplier {
    func getTags(currentUser: UserSessionInfo?) -> [String]? {
        guard let user = currentUser else { return nil }
        return ["team:\(user.id ?? "")", "public"]
    }
}

sdk.tagSupplier = TeamTagSupplier()
```

Filtrelenmemiş küresel feed için `nil` döndürün.

### Feed Durumunu Kaydetme ve Geri Yükleme

Sayfalandırma (pagination) durumunu görünüm yaşam döngüsü olayları boyunca koruyun:

```swift
let state = sdk.savePaginationState()
// Later...
sdk.restorePaginationState(state)
```

### Gönderileri Silme

```swift
sdk.onPostDeleted = { postId in
    print("Post \(postId) was deleted")
}
```

---
---