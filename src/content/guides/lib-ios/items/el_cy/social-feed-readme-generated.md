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
                // Εμφάνιση φύλλου κοινοποίησης
            }
            .onUserClick { context, userInfo, source in
                // Πλοήγηση στο προφίλ χρήστη
            }
            .onMediaClick { mediaItem, index in
                // Εμφάνιση προβολέα εικόνας σε πλήρη οθόνη
            }
            .task {
                try? await sdk.loadIfNeeded()
            }
    }
}
```

Η προβολή ροής περιλαμβάνει σύρσιμο για ανανέωση (pull-to-refresh) και ατέρμονη κύλιση αυτόματα.
Χρησιμοποιήστε `loadIfNeeded()` για επανείσοδο στο lifecycle της οθόνης ώστε μια υπάρχουσα ή επαναφερθείσα ροή να μην επανέρχεται στην σελίδα 1.

### Δημιουργία Δημοσιεύσεων

Χρησιμοποιήστε `FeedPostCreateView` για να εμφανίσετε μια φόρμα δημιουργίας δημοσίευσης:

```swift
@State private var showCreatePost = false

// Στο σώμα της προβολής σας:
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

Το SDK χειρίζεται τις αντιδράσεις με αισιόδοξες ενημερώσεις (optimistic updates):

```swift
try await sdk.reactPost(postId: post.id, reactionType: "l")

// Check reaction state
let hasLiked = sdk.hasUserReacted(postId: post.id, reactType: "l")
let likeCount = sdk.getLikeCount(postId: post.id)
```

### Άνοιγμα Σχολίων για μια Δημοσίευση

Χρησιμοποιήστε `CommentsSheet` για να εμφανίσετε σχόλια για μια δημοσίευση στη ροή. Δημιουργεί εσωτερικά ένα instance `FastCommentsSDK` χρησιμοποιώντας το config του feed SDK:

```swift
.sheet(item: $commentsPost) { post in
    CommentsSheet(post: post, feedSDK: sdk, onUserClick: { context, userInfo, source in
        // Διαχείριση κλικ χρήστη
    })
}
```

Σημείωση: `FeedPost` πρέπει να συμμορφώνεται με το `Identifiable` για το `.sheet(item:)`. Προσθέστε αυτή την επέκταση:

```swift
extension FeedPost: @retroactive Identifiable {}
```

### Φιλτράρισμα Ροής με Βάση τις Ετικέτες

Υλοποιήστε το πρωτόκολλο `TagSupplier` για να φιλτράρετε δημοσιεύσεις ροής με βάση ετικέτες:

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

Διατηρήστε την κατάσταση σελιδοποίησης (pagination) κατά τα γεγονότα lifecycle της προβολής:

```swift
let state = sdk.savePaginationState()
// Later...
sdk.restorePaginationState(state)
try? await sdk.loadIfNeeded()
```

Αν η οθόνη σας εξαφανιστεί προσωρινά, η προβολή ροής διακόπτει αυτόματα τις ζωντανές ενημερώσεις και τις επαναλαμβάνει όταν επανεμφανιστεί χωρίς να διαγράψει τις φορτωμένες δημοσιεύσεις. Καλέστε `sdk.cleanup()` μόνο όταν έχετε πραγματικά τελειώσει με το instance του SDK.

### Διαγραφή Δημοσιεύσεων

```swift
sdk.onPostDeleted = { postId in
    print("Post \(postId) was deleted")
}
```