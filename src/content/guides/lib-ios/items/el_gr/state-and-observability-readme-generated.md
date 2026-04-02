Και τα `FastCommentsSDK` και `FastCommentsFeedSDK` είναι κλάσεις `ObservableObject` με ιδιότητες `@Published`. Μπορείτε να παρακολουθείτε αυτές στις προβολές SwiftUI σας για αντιδραστικές ενημερώσεις του UI.

### FastCommentsSDK Δημοσιευμένες Ιδιότητες

| Ιδιότητα | Τύπος | Περιγραφή |
|----------|------|-------------|
| `commentCountOnServer` | `Int` | Συνολικός αριθμός σχολίων στον διακομιστή |
| `newRootCommentCount` | `Int` | Νέα σχόλια σε προσωρινή αποθήκευση (όταν το `showLiveRightAway` είναι false) |
| `currentUser` | `UserSessionInfo?` | Τρέχων αυθεντικοποιημένος χρήστης |
| `isSiteAdmin` | `Bool` | Εάν ο τρέχων χρήστης είναι διαχειριστής του ιστότοπου |
| `isClosed` | `Bool` | Εάν το νήμα σχολίων είναι κλειστό |
| `hasBillingIssue` | `Bool` | Εάν υπάρχει πρόβλημα χρέωσης |
| `isLoading` | `Bool` | Εάν βρίσκεται σε εξέλιξη ένα αίτημα δικτύου |
| `hasMore` | `Bool` | Εάν υπάρχουν περισσότερες σελίδες σχολίων |
| `blockingErrorMessage` | `String?` | Σφάλμα που εμποδίζει τη λειτουργία της διεπαφής χρήστη |
| `warningMessage` | `String?` | Μη μπλοκαριστικό μήνυμα προειδοποίησης |
| `isDemo` | `Bool` | Εάν τρέχει σε λειτουργία demo |
| `commentsVisible` | `Bool` | Εναλλαγή ορατότητας σχολίων |
| `toolbarEnabled` | `Bool` | Εάν η γραμμή εργαλείων μορφοποίησης εμφανίζεται |

### FastCommentsFeedSDK Δημοσιευμένες Ιδιότητες

| Ιδιότητα | Τύπος | Περιγραφή |
|----------|------|-------------|
| `feedPosts` | `[FeedPost]` | Αναρτήσεις feed που έχουν φορτωθεί |
| `hasMore` | `Bool` | Εάν υπάρχουν περισσότερες σελίδες |
| `currentUser` | `UserSessionInfo?` | Τρέχων αυθεντικοποιημένος χρήστης |
| `blockingErrorMessage` | `String?` | Μπλοκαριστικό μήνυμα σφάλματος |
| `isLoading` | `Bool` | Εάν βρίσκεται σε εξέλιξη ένα αίτημα δικτύου |
| `newPostsCount` | `Int` | Αριθμός νέων αναρτήσεων από την τελευταία φόρτωση |

### Δέντρο Σχολίων

Το δέντρο σχολίων είναι προσβάσιμο μέσω του `sdk.commentsTree`:

```swift
// Επίπεδη λίστα ορατών κόμβων για απόδοση
sdk.commentsTree.visibleNodes

// Εύρεση σχολίου κατά ID
sdk.commentsTree.commentsById["comment-id"]
```

---
---