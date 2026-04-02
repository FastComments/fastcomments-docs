---
Both `FastCommentsSDK` and `FastCommentsFeedSDK` are `ObservableObject` classes with `@Published` properties. You can observe these in your SwiftUI views for reactive UI updates.

### FastCommentsSDK Published Properties

| Property | Type | Description |
|----------|------|-------------|
| `commentCountOnServer` | `Int` | Συνολικός αριθμός σχολίων στον διακομιστή |
| `newRootCommentCount` | `Int` | Προσωρινά αποθηκευμένα νέα σχόλια (όταν το `showLiveRightAway` είναι false) |
| `currentUser` | `UserSessionInfo?` | Τρέχων αυθεντικοποιημένος χρήστης |
| `isSiteAdmin` | `Bool` | Εάν ο τρέχων χρήστης είναι διαχειριστής του ιστότοπου |
| `isClosed` | `Bool` | Εάν το νήμα σχολίων είναι κλειστό |
| `hasBillingIssue` | `Bool` | Εάν υπάρχει πρόβλημα με τη χρέωση |
| `isLoading` | `Bool` | Εάν εκτελείται αίτημα δικτύου |
| `hasMore` | `Bool` | Εάν υπάρχουν περισσότερες σελίδες σχολίων |
| `blockingErrorMessage` | `String?` | Σφάλμα που εμποδίζει τη λειτουργία του UI |
| `warningMessage` | `String?` | Μήνυμα προειδοποίησης που δεν εμποδίζει τη λειτουργία |
| `isDemo` | `Bool` | Εάν εκτελείται σε λειτουργία επίδειξης |
| `commentsVisible` | `Bool` | Εναλλαγή για την ορατότητα των σχολίων |
| `toolbarEnabled` | `Bool` | Εάν η γραμμή εργαλείων μορφοποίησης εμφανίζεται |

### FastCommentsFeedSDK Published Properties

| Property | Type | Description |
|----------|------|-------------|
| `feedPosts` | `[FeedPost]` | Τρέχουσες φορτωμένες αναρτήσεις ροής |
| `hasMore` | `Bool` | Εάν υπάρχουν περισσότερες σελίδες |
| `currentUser` | `UserSessionInfo?` | Τρέχων αυθεντικοποιημένος χρήστης |
| `blockingErrorMessage` | `String?` | Σφάλμα που εμποδίζει τη λειτουργία |
| `isLoading` | `Bool` | Εάν εκτελείται αίτημα δικτύου |
| `newPostsCount` | `Int` | Αριθμός νέων αναρτήσεων από την τελευταία φόρτωση |

### Comment Tree

The comment tree is accessible via `sdk.commentsTree`:

```swift
// Επίπεδη λίστα ορατών κόμβων για απόδοση
sdk.commentsTree.visibleNodes

// Αναζήτηση σχολίου με βάση το αναγνωριστικό
sdk.commentsTree.commentsById["comment-id"]
```

---