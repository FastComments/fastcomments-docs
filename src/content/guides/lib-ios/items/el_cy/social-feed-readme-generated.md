---
Το σύστημα ροής είναι ένα ξεχωριστό SDK (`FastCommentsFeedSDK`) με το δικό του view.

### Φόρτωση και Εμφάνιση της Ροής

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
                // Present share sheet
            }
            .onUserClick { context, userInfo, source in
                // Navigate to user profile
            }
            .onMediaClick { mediaItem, index in
                // Present full-screen image viewer
            }
            .task {
                try? await sdk.load()
            }
    }
}
```

Το view της ροής περιλαμβάνει αυτόματα λειτουργία pull-to-refresh και infinite scroll.

### Δημιουργία Δημοσιεύσεων

Χρησιμοποιήστε το `FeedPostCreateView` για να εμφανίσετε μια φόρμα δημιουργίας δημοσίευσης:

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

### Αντίδραση σε Δημοσιεύσεις

Το SDK διαχειρίζεται τις αντιδράσεις με οπτιμιστικές ενημερώσεις:

```swift
try await sdk.reactPost(postId: post.id, reactionType: "l")

// Check reaction state
let hasLiked = sdk.hasUserReacted(postId: post.id, reactType: "l")
let likeCount = sdk.getLikeCount(postId: post.id)
```

### Άνοιγμα σχολίων σε μια δημοσίευση

Χρησιμοποιήστε το `CommentsSheet` για να εμφανίσετε τα σχόλια μιας δημοσίευσης στη ροή. Δημιουργεί εσωτερικά ένα στιγμιότυπο `FastCommentsSDK` χρησιμοποιώντας τη διαμόρφωση (config) του feed SDK:

```swift
.sheet(item: $commentsPost) { post in
    CommentsSheet(post: post, feedSDK: sdk, onUserClick: { context, userInfo, source in
        // Handle user click
    })
}
```

Σημείωση: `FeedPost` πρέπει να συμμορφώνεται με `Identifiable` για `.sheet(item:)`. Προσθέστε αυτήν την επέκταση:

```swift
extension FeedPost: @retroactive Identifiable {}
```

### Φιλτράρισμα ροής με βάση τις ετικέτες

Υλοποιήστε το πρωτόκολλο `TagSupplier` για να φιλτράρετε τις δημοσιεύσεις της ροής κατά ετικέτες:

```swift
struct TeamTagSupplier: TagSupplier {
    func getTags(currentUser: UserSessionInfo?) -> [String]? {
        guard let user = currentUser else { return nil }
        return ["team:\(user.id ?? "")", "public"]
    }
}

sdk.tagSupplier = TeamTagSupplier()
```

Επιστρέψτε `nil` για μια παγκόσμια ροή χωρίς φίλτρο.

### Αποθήκευση και Επαναφορά Κατάστασης της Ροής

Διατηρήστε την κατάσταση σελιδοποίησης (pagination) ανάμεσα σε συμβάντα κύκλου ζωής του view:

```swift
let state = sdk.savePaginationState()
// Later...
sdk.restorePaginationState(state)
```

### Διαγραφή Δημοσιεύσεων

```swift
sdk.onPostDeleted = { postId in
    print("Post \(postId) was deleted")
}
```

---
---