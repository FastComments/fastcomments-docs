Εκτελεί τον agent κάθε φορά που δημοσιεύεται ένα νέο σχόλιο σε μια σελίδα που καλύπτεται από το [scope](#scope-url-locale) του agent.

### Context the agent receives

- Το νέο σχόλιο ολόκληρο - κείμενο, author, votes, parent ID, page URL ID.
- Προαιρετικά: parent comment και προηγούμενες απαντήσεις στο ίδιο thread, εάν το [thread context](#context-options) είναι ενεργό.
- Προαιρετικά: ο trust factor του σχολιαστή, η ηλικία του λογαριασμού, ιστορικό αποκλεισμών και πρόσφατα σχόλια, εάν το [user history context](#context-options) είναι ενεργό.
- Προαιρετικά: μεταδεδομένα σελίδας, εάν το [page context](#context-options) είναι ενεργό.

### Notable

- Ο trigger εκτελείται **μετά** το σχόλιο να έχει αποθηκευτεί. Ο agent μπορεί να αναφερθεί σε αυτό απευθείας σε κλήσεις εργαλείων.
- Δεn εκτελείται για σχόλια που έχουν συγγραφεί από άλλο agent στον ίδιο tenant.
- Εκτελείται για τόσο επαληθευμένα όσο και μη επαληθευμένα σχόλια. Εάν ο tenant σας απαιτεί έγκριση από moderator πριν ένα σχόλιο γίνει ορατό (βλ. [How Approvals Work](/guide-moderation.html#moderation-approvals) στον οδηγό moderation), ο trigger εκτελείται όταν το σχόλιο δημιουργείται, όχι όταν εγκριθεί αργότερα. Το moderator bot μπορεί να λάβει εντολή να εγκρίνει σχόλια για εσάς μετά από έλεγχο.

### Common uses

- **Moderation** - ελέγξτε το σχόλιο σε σχέση με τις οδηγίες της κοινότητας, επισημάνετε ως spam ή προειδοποιήστε τους πρωτοεμφανιζόμενους.
- **Welcome greeting** - αν και το [Trigger: New User First Comment](#trigger-new-user-first-comment) συνήθως είναι καταλληλότερο για χαιρετισμούς καθώς εκτελείται μία φορά ανά χρήστη.
- **Thread summarization** - συνήθως σε συνδυασμό με ένα [trigger delay](#trigger-deferred-delay) ώστε το thread να σταθεροποιηθεί πριν εκτελεστεί ο agent.