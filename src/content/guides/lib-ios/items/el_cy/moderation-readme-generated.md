### Ενέργειες Διαθέσιμες σε Όλους τους Χρήστες

- **Επισήμανση/Αφαίρεση επισήμανσης** -- αναφέρετε ένα σχόλιο για επανεξέταση

```swift
try await sdk.flagComment(commentId: commentId)
try await sdk.unflagComment(commentId: commentId)
```

- **Αποκλεισμός/Κατάργηση αποκλεισμού** -- απόκρυψη όλων των σχολίων από έναν χρήστη (ανά θεατή)

```swift
try await sdk.blockUser(commentId: commentId)
try await sdk.unblockUser(commentId: commentId)
```

### Ενέργειες Μόνο για Διαχειριστές

- **Καρφίτσωμα/Αποκαρφίτσωμα** -- καρφώστε ένα σχόλιο στην κορυφή του νήματος

```swift
try await sdk.pinComment(commentId: commentId)
try await sdk.unpinComment(commentId: commentId)
```

- **Κλείδωμα/Ξεκλείδωμα** -- αποτρέψτε νέες απαντήσεις σε ένα σχόλιο

```swift
try await sdk.lockComment(commentId: commentId)
try await sdk.unlockComment(commentId: commentId)
```

Όλες οι ενέργειες διαχείρισης είναι επίσης διαθέσιμες μέσω του μενού περιβάλλοντος σχολίου στο UI. Οι ενέργειες για διαχειριστές εμφανίζονται μόνο όταν ο τρέχων χρήστης είναι διαχειριστής του ιστότοπου (ορίζεται μέσω του SSO `isAdmin` flag ή μέσω ρυθμίσεων του πίνακα ελέγχου).

---
---