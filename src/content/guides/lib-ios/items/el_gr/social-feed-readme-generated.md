Το σύστημα ροής είναι ένα ξεχωριστό SDK (`FastCommentsFeedSDK`) με τη δική του προβολή.

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
                // Εμφάνιση φύλλου κοινής χρήσης
            }
            .onUserClick { context, userInfo, source in
                // Μετάβαση στο προφίλ χρήστη
            }
            .onMediaClick { mediaItem, index in
                // Εμφάνιση προβολέα εικόνων σε πλήρη οθόνη
            }
            .task {
                try? await sdk.load()
            }
    }
}
```

Η προβολή ροής περιλαμβάνει αυτόματα σύρσιμο για ανανέωση (pull-to-refresh) και απεριόριστη κύλιση.

### Δημιουργία Αναρτήσεων

Χρησιμοποιήστε το `FeedPostCreateView` για να εμφανίσετε μια φόρμα δημιουργίας ανάρτησης:

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

### Αντιδράσεις σε Αναρτήσεις

Το SDK χειρίζεται τις αντιδράσεις με αισιόδοξες ενημερώσεις (optimistic updates):

```swift
try await sdk.reactPost(postId: post.id, reactionType: "l")

// Check reaction state
let hasLiked = sdk.hasUserReacted(postId: post.id, reactType: "l")
let likeCount = sdk.getLikeCount(postId: post.id)
```

### Άνοιγμα Σχολίων σε Ανάρτηση

Χρησιμοποιήστε το `CommentsSheet` για να εμφανίσετε σχόλια για μια ανάρτηση ροής. Αυτό δημιουργεί εσωτερικά ένα στιγμιότυπο `FastCommentsSDK` χρησιμοποιώντας τη διαμόρφωση (config) του feed SDK:

```swift
.sheet(item: $commentsPost) { post in
    CommentsSheet(post: post, feedSDK: sdk, onUserClick: { context, userInfo, source in
        // Χειρισμός κλικ χρήστη
    })
}
```

Σημείωση: `FeedPost` πρέπει να συμμορφώνεται με `Identifiable` για `.sheet(item:)`. Προσθέστε αυτήν την επέκταση:

```swift
extension FeedPost: @retroactive Identifiable {}
```

### Φιλτράρισμα ροής βάσει ετικετών

Υλοποιήστε το πρωτόκολλο `TagSupplier` για να φιλτράρετε τις αναρτήσεις της ροής βάσει ετικετών:

```swift
struct TeamTagSupplier: TagSupplier {
    func getTags(currentUser: UserSessionInfo?) -> [String]? {
        guard let user = currentUser else { return nil }
        return ["team:\(user.id ?? "")", "public"]
    }
}

sdk.tagSupplier = TeamTagSupplier()
```

Επιστρέψτε `nil` για μια μη φιλτραρισμένη παγκόσμια ροή.

### Αποθήκευση και Επαναφορά Κατάστασης Ροής

Διατηρήστε την κατάσταση σελιδοποίησης κατά τη διάρκεια των γεγονότων του κύκλου ζωής της προβολής:

```swift
let state = sdk.savePaginationState()
// Later...
sdk.restorePaginationState(state)
```

### Διαγραφή Αναρτήσεων

```swift
sdk.onPostDeleted = { postId in
    print("Post \(postId) was deleted")
}
```

---
---