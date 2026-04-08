Feed систем је одвојен SDK (`FastCommentsFeedSDK`) са својим властим приказом.

### Учитавање и приказ фид-а

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
                // Прикажи sheet за дељење
            }
            .onUserClick { context, userInfo, source in
                // Навигирај до корисничког профила
            }
            .onMediaClick { mediaItem, index in
                // Прикажи прегледач слике преко целог екрана
            }
            .task {
                try? await sdk.loadIfNeeded()
            }
    }
}
```

Фид приказ укључује pull-to-refresh и бесконачно скроловање аутоматски.
Користите `loadIfNeeded()` за поновни улазак у животни циклус екрана тако да постојећи или враћени фид не буде враћен на страницу 1.

### Креирање постова

Користите `FeedPostCreateView` да прикажете форму за креирање поста:

```swift
@State private var showCreatePost = false

// У телу вашег приказа:
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

SDK управља реакцијама уз оптимистичка ажурирања:

```swift
try await sdk.reactPost(postId: post.id, reactionType: "l")

// Check reaction state
let hasLiked = sdk.hasUserReacted(postId: post.id, reactType: "l")
let likeCount = sdk.getLikeCount(postId: post.id)
```

### Отварање коментара на пост

Користите `CommentsSheet` да прикажете коментаре за пост у фид-у. Он креира `FastCommentsSDK` инстанцу интерно користећи конфигурацију фид SDK-а:

```swift
.sheet(item: $commentsPost) { post in
    CommentsSheet(post: post, feedSDK: sdk, onUserClick: { context, userInfo, source in
        // Обрадити клик на корисника
    })
}
```

Напомена: `FeedPost` мора да се придржава `Identifiable` за `.sheet(item:)`. Додајте ово проширење:

```swift
extension FeedPost: @retroactive Identifiable {}
```

### Филтрирање фид-а по таговима

Имплементирајте протокол `TagSupplier` да бисте филтрирали постове фид-а по таговима:

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

### Чување и враћање стања фид-а

Сачувајте стање пагинације преко догађаја животног циклуса приказа:

```swift
let state = sdk.savePaginationState()
// Later...
sdk.restorePaginationState(state)
try? await sdk.loadIfNeeded()
```

Ако ваш екран привремено нестане, фид приказ аутоматски паузира live ажурирања и наставља их при поновном појављивању без брисања учитаних постова. Позовите `sdk.cleanup()` само када сте заиста завршили са инстанцом SDK-а.

### Брисање постова

```swift
sdk.onPostDeleted = { postId in
    print("Post \(postId) was deleted")
}
```