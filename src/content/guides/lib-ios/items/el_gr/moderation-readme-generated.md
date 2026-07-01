### Δράσεις Διαθέσιμες σε Όλους τους Χρήστες

- **Flag/Unflag** -- αναφορά σχολίου για έλεγχο

```swift
try await sdk.flagComment(commentId: commentId)
try await sdk.unflagComment(commentId: commentId)
```

- **Block/Unblock** -- απόκρυψη όλων των σχολίων από έναν χρήστη (ανά θεατή)

```swift
try await sdk.blockUser(commentId: commentId)
try await sdk.unblockUser(commentId: commentId)
```

### Δράσεις Μόνο για Διαχειριστές

- **Pin/Unpin** -- καρφίτσωμα ενός σχολίου στην κορυφή της συζήτησης

```swift
try await sdk.pinComment(commentId: commentId)
try await sdk.unpinComment(commentId: commentId)
```

- **Lock/Unlock** -- αποτροπή νέων απαντήσεων σε ένα σχόλιο, καθώς και φραγή επεξεργασιών και διαγραφών μέχρι να ξεκλειδωθεί (εφαρμόζεται σε όλους, συμπεριλαμβανομένων των συντονιστών)

```swift
try await sdk.lockComment(commentId: commentId)
try await sdk.unlockComment(commentId: commentId)
```

Όλες οι ενέργειες διαχείρισης είναι επίσης διαθέσιμες μέσω του μενού περιβάλλοντος σχολίου στη διεπαφή χρήστη. Οι ενέργειες διαχειριστή εμφανίζονται μόνο όταν ο τρέχων χρήστης είναι διαχειριστής του site (ορίζεται μέσω της σημαίας SSO `isAdmin` ή της διαμόρφωσης πίνακα ελέγχου).