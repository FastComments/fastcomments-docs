### Δράσεις Διαθέσιμες Σε Όλους Τους Χρήστες

- **Flag/Unflag** -- αναφέρετε ένα σχόλιο για έλεγχο

```swift
try await sdk.flagComment(commentId: commentId)
try await sdk.unflagComment(commentId: commentId)
```

- **Block/Unblock** -- απόκρυψη όλων των σχολίων από έναν χρήστη (για κάθε θεατή)

```swift
try await sdk.blockUser(commentId: commentId)
try await sdk.unblockUser(commentId: commentId)
```

### Δράσεις Μόνο για Διαχειριστές

- **Pin/Unpin** -- καρφιτσώστε ένα σχόλιο στην κορυφή της συζήτησης

```swift
try await sdk.pinComment(commentId: commentId)
try await sdk.unpinComment(commentId: commentId)
```

- **Lock/Unlock** -- αποτροπή νέων απαντήσεων σε ένα σχόλιο, και αποκλεισμός επεξεργασιών και διαγραφών μέχρι να ξεκλειδωθεί (εφαρμόζεται σε όλους, συμπεριλαμβανομένων των συντονιστών)

```swift
try await sdk.lockComment(commentId: commentId)
try await sdk.unlockComment(commentId: commentId)
```

Όλες οι ενέργειες συντονισμού διατίθενται επίσης μέσω του μενού περιβάλλοντος του σχολίου στο UI. Οι ενέργειες διαχειριστή εμφανίζονται μόνο όταν ο τρέχων χρήστης είναι διαχειριστής του ιστότοπου (ρυθμίζεται μέσω της σημαίας SSO `isAdmin` ή της διαμόρφωσης πίνακα ελέγχου).