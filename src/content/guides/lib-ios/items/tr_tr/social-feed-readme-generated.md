Akış sistemi, kendi görünümüne sahip ayrı bir SDK'dır (`FastCommentsFeedSDK`).

### Akışı Yükleme ve Görüntüleme

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
                // Tam ekran görüntüleyiciyi göster
            }
            .task {
                try? await sdk.loadIfNeeded()
            }
    }
}
```

Akış görünümü, çek-yenile ve sonsuz kaydırma özelliklerini otomatik olarak içerir.
Ekran yaşam döngüsüne yeniden girişlerde `loadIfNeeded()` kullanın; böylece mevcut veya geri yüklenmiş bir akış 1. sayfaya sıfırlanmaz.

### Gönderi Oluşturma

Gönderi oluşturma formunu göstermek için `FeedPostCreateView` kullanın:

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

SDK, iyimser güncellemelerle tepkileri yönetir:

```swift
try await sdk.reactPost(postId: post.id, reactionType: "l")

// Tepki durumunu kontrol et
let hasLiked = sdk.hasUserReacted(postId: post.id, reactType: "l")
let likeCount = sdk.getLikeCount(postId: post.id)
```

### Bir Gönderideki Yorumları Açma

Bir akış gönderisinin yorumlarını görüntülemek için `CommentsSheet` kullanın. Akış SDK'sının yapılandırmasını kullanarak dahili olarak bir `FastCommentsSDK` örneği oluşturur:

```swift
.sheet(item: $commentsPost) { post in
    CommentsSheet(post: post, feedSDK: sdk, onUserClick: { context, userInfo, source in
        // Kullanıcı tıklamasını işle
    })
}
```

Not: `.sheet(item:)` için `FeedPost` öğesinin `Identifiable` uyumlu olması gerekir. Şu uzantıyı ekleyin:

```swift
extension FeedPost: @retroactive Identifiable {}
```

### Etiket Tabanlı Akış Filtreleme

Akış gönderilerini etiketlere göre filtrelemek için `TagSupplier` protokolünü uygulayın:

```swift
struct TeamTagSupplier: TagSupplier {
    func getTags(currentUser: UserSessionInfo?) -> [String]? {
        guard let user = currentUser else { return nil }
        return ["team:\(user.id ?? "")", "public"]
    }
}

sdk.tagSupplier = TeamTagSupplier()
```

Filtrelenmemiş genel bir akış için `nil` döndürün.

### Akış Durumunu Kaydetme ve Geri Yükleme

Sayfalama durumunu görünüm yaşam döngüsü olayları boyunca koruyun:

```swift
let state = sdk.savePaginationState()
// Daha sonra...
sdk.restorePaginationState(state)
try? await sdk.loadIfNeeded()
```

Ekranınız geçici olarak kaybolursa, akış görünümü canlı güncellemeleri otomatik olarak duraklatır ve yüklenmiş gönderileri temizlemeden yeniden göründüğünde devam ettirir. `sdk.cleanup()` çağrısını yalnızca SDK örneğiyle gerçekten işiniz bittiğinde yapın.

### Gönderi Silme

```swift
sdk.onPostDeleted = { postId in
    print("Post \(postId) was deleted")
}
```

---
---