---
Систем за фид је посебан SDK (`FastCommentsFeedSDK`) са сопственим приказом.

### Учитавање и приказивање фида

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
                // Прикажи лист за дељење
            }
            .onUserClick { context, userInfo, source in
                // Иди на профил корисника
            }
            .onMediaClick { mediaItem, index in
                // Прикажи прегледач слика у пуном екрану
            }
            .task {
                try? await sdk.loadIfNeeded()
            }
    }
}
```

Приказ фида аутоматски укључује повуци‑за‑освежавање и бесконачно скроловање.  
Користите `loadIfNeeded()` за поновни улаз у животни циклус екрана тако да постојећи или обновљени фид не ресетује на страницу 1.

### Креирање постова

Користите `FeedPostCreateView` за приказ форме за креирање поста:

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

### Реаговање на постове

SDK управља реакцијама са оптимистичким ажурирањима:

```swift
try await sdk.reactPost(postId: post.id, reactionType: "l")

// Check reaction state
let hasLiked = sdk.hasUserReacted(postId: post.id, reactType: "l")
let likeCount = sdk.getLikeCount(postId: post.id)
```

### Отварање коментара на посту

Користите `CommentsSheet` за приказ коментара за пост у фиду. Он унутар креира `FastCommentsSDK` инстанцу користећи конфигурацију feed SDK‑а:

```swift
.sheet(item: $commentsPost) { post in
    CommentsSheet(post: post, feedSDK: sdk, onUserClick: { context, userInfo, source in
        // Обради клик корисника
    })
}
```

Напомена: `FeedPost` мора да имплементира `Identifiable` за `.sheet(item:)`. Додајте ову екстензију:

```swift
extension FeedPost: @retroactive Identifiable {}
```

### Филтрирање фида по ознакама

Имплементирајте протокол `TagSupplier` за филтрирање постова у фиду по ознакама:

```swift
struct TeamTagSupplier: TagSupplier {
    func getTags(currentUser: UserSessionInfo?) -> [String]? {
        guard let user = currentUser else { return nil }
        return ["team:\(user.id ?? "")", "public"]
    }
}

sdk.tagSupplier = TeamTagSupplier()
```

Вратите `nil` за нефилтрирани глобални фид.

### Чување и враћање стања фида

Сачувајте стање пагинације током догађаја животног циклуса приказа:

```swift
let state = sdk.savePaginationState()
// Later...
sdk.restorePaginationState(state)
try? await sdk.loadIfNeeded()
```

Ако ваш екран привремено нестане, приказ фида аутоматски паузира живе ажурирања и наставља их по поновном појављивању без брисања учитаних постова. Позовите `sdk.cleanup()` само када заиста завршите са SDK инстанцом.

### Брисање постова

```swift
sdk.onPostDeleted = { postId in
    print("Post \(postId) was deleted")
}
```
---