Feed систем је посебан SDK (`FastCommentsFeedSDK`) са својим приказом.

### Учитавање и приказ фида

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
                // Прикажи листу за дељење
            }
            .onUserClick { context, userInfo, source in
                // Отвори кориснички профил
            }
            .onMediaClick { mediaItem, index in
                // Прикажи приказивач слика преко целог екрана
            }
            .task {
                try? await sdk.load()
            }
    }
}
```

Приказ фида аутоматски укључује повлачење за освежавање и бесконачно скроловање.

### Креирање постова

Користите `FeedPostCreateView` да прикажете форму за креирање поста:

```swift
@State private var showCreatePost = false

// Унутар тела вашег приказа:
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

### Реакције на постове

SDK обрађује реакције са оптимистичким ажурирањима:

```swift
try await sdk.reactPost(postId: post.id, reactionType: "l")

// Check reaction state
let hasLiked = sdk.hasUserReacted(postId: post.id, reactType: "l")
let likeCount = sdk.getLikeCount(postId: post.id)
```

### Отварање коментара на посту

Користите `CommentsSheet` за приказ коментара за пост у фиду. Он унутар себе креира инстанцу `FastCommentsSDK` користећи конфигурацију feed SDK-а:

```swift
.sheet(item: $commentsPost) { post in
    CommentsSheet(post: post, feedSDK: sdk, onUserClick: { context, userInfo, source in
        // Обради клик на корисника
    })
}
```

Напомена: `FeedPost` мора да имплементира `Identifiable` за `.sheet(item:)`. Додајте ову екстензију:

```swift
extension FeedPost: @retroactive Identifiable {}
```

### Филтрирање фида по таговима

Имплементирајте протокол `TagSupplier` да бисте филтрирали постове у фиду по таговима:

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

Сачувајте стање пагинације кроз животни циклус приказа:

```swift
let state = sdk.savePaginationState()
// Later...
sdk.restorePaginationState(state)
```

### Брисање постова

```swift
sdk.onPostDeleted = { postId in
    print("Post \(postId) was deleted")
}
```

---
---