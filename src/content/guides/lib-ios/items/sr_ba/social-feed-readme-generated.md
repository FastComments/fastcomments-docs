Систем фида је одвојен SDK (`FastCommentsFeedSDK`) са својим приказом.

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
                // Прikaжи дијалог за дијељење
            }
            .onUserClick { context, userInfo, source in
                // Отвори кориснички профил
            }
            .onMediaClick { mediaItem, index in
                // Прikaжи преглед слике преко цијелог екрана
            }
            .task {
                try? await sdk.loadIfNeeded()
            }
    }
}
```

Приказ фида укључује повлачење за освежавање и неограничено скроловање аутоматски.
Користите `loadIfNeeded()` при поновном уласку у циклус живота екрана тако да постојећи или враћени фид не буде ресетован назад на страницу 1.

### Креирање постова

Користите `FeedPostCreateView` да прикажете форму за креирање поста:

```swift
@State private var showCreatePost = false

// У тијелу вашег приказа:
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

SDK рукује реакцијама са оптимистичким ажурирањима:

```swift
try await sdk.reactPost(postId: post.id, reactionType: "l")

// Check reaction state
let hasLiked = sdk.hasUserReacted(postId: post.id, reactType: "l")
let likeCount = sdk.getLikeCount(postId: post.id)
```

### Отварање коментара на пост

Користите `CommentsSheet` да прикажете коментаре за пост из фида. Он креира инстанцу `FastCommentsSDK` интерно користећи конфигурацију feed SDK-а:

```swift
.sheet(item: $commentsPost) { post in
    CommentsSheet(post: post, feedSDK: sdk, onUserClick: { context, userInfo, source in
        // Обрадите клик корисника
    })
}
```

Напомена: `FeedPost` мора да реализује `Identifiable` за `.sheet(item:)`. Додајте ово проширење:

```swift
extension FeedPost: @retroactive Identifiable {}
```

### Филтрирање фида на основу тагова

Имплементирајте протокол `TagSupplier` да филтрирате постове фида по таговима:

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

Сачувајте стање пагинације преко догађаја животног циклуса приказа:

```swift
let state = sdk.savePaginationState()
// Later...
sdk.restorePaginationState(state)
try? await sdk.loadIfNeeded()
```

Ако ваш екран привремено не буде видљив, приказ фида аутоматски паузира ажурирања уживо и наставља их при поновном појављивању без брисања учитаних постова. Позивајте `sdk.cleanup()` само када сте заиста завршили са инстанцом SDK-а.

### Брисање постова

```swift
sdk.onPostDeleted = { postId in
    print("Post \(postId) was deleted")
}
```