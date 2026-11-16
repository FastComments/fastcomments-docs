Οι δημοσκοπήσεις λειτουργούν όπως στο web widget και ενεργοποιούνται από τις ίδιες ρυθμίσεις του widget στον πίνακα ελέγχου FastComments (ή με τα `pollCreation` / `pollVoting` παραπάνω).

- Τα σχόλια με δημοσκόπηση την εμφανίζουν κάτω από το κείμενό τους. Οι αναγνώστες ψηφίζουν πατώντας μια επιλογή, μπορούν να δείξουν ή να κρύψουν τα αποτελέσματα πριν ψηφίσουν και μπορούν να αλλάξουν την ψήφο τους. Οι μετρήσεις ενημερώνονται ζωντανά καθώς ψηφίζουν άλλοι.
- Οι δημοσκοπήσεις μπορούν να κλείσουν σε μια ημερομηνία, να κρύψουν τα αποτελέσματα μέχρι να ψηφίσετε και να κάνουν τους ψηφοφόρους ορατούς στους διαχειριστές ή σε όλους («Δείτε ποιοι ψήφισαν»).
- Όταν ο θεατής μπορεί να δημιουργήσει δημοσκοπήσεις, το πεδίο σχολίου του `FastCommentsView` εμφανίζει ένα κουμπί δημοσκόπησης που ανοίγει τον δημιουργό δημοσκοπήσεων. Η γραμμή εργαλείων εμφανίζεται γι' αυτό ακόμα κι αν δεν έχετε ενεργοποιήσει ποτέ τη γραμμή εργαλείων· το `sdk.setCommentToolbarEnabled(false)` την κρύβει, συμπεριλαμβανομένου του κουμπιού δημοσκόπησης, και το `setPollCreationEnabled(false)` στο πεδίο αφαιρεί μόνο το κουμπί δημοσκόπησης. Καλέστε το `setCommentToolbarEnabled` πριν δημιουργήσετε το `FastCommentsView`, αφού το πεδίο το διαβάζει όταν λαμβάνει το SDK· για μια προβολή που βρίσκεται ήδη στην οθόνη, καλέστε αντ' αυτού το `getBottomCommentInput().setToolbarVisible(false)`. Το πεδίο της live συνομιλίας το παραλείπει, όπως το web live chat.

Για να δημοσιεύσετε μια δημοσκόπηση από το δικό σας UI, περάστε ένα `CommentPollInput` στο `postComment`:

```java
CommentPollInput poll = new CommentPollInput()
        .question("Where should we eat?")
        .options(Arrays.asList("Tacos", "Ramen"))
        // Optional: an ISO 8601 close date, PollPrivacy value, and results hidden until you vote.
        .closesAt(Instant.now().plus(1, ChronoUnit.DAYS).toString())
        .privacy(PollPrivacy.NUMBER_2.getValue())
        .requireVoteToSeeResults(true);
sdk.postComment("Lunch vote", null, null, poll, callback);
```

Τα `sdk.votePoll(commentId, optionId, callback)` και `sdk.getPollVoters(commentId, page, search, callback)` καλύπτουν την ψηφοφορία και τη λίστα ψηφοφόρων. Τα χρώματα των δημοσκοπήσεων αναφέρονται στο THEMING.md.