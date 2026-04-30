FastComments παρέχει πέντε πρότυπα εκκίνησης ώστε να μην χρειάζεται να γράψετε έναν λειτουργικό agent από την αρχή. Είναι προσβάσιμα από τη [AI Agents page](https://fastcomments.com/auth/my-account/ai-agents) κάνοντας κλικ στο **Browse templates**.

When you pick a template:

1. Ο πράκτορας δημιουργείται με **Κατάσταση: Dry Run** και ένα εσωτερικό όνομα βασισμένο στο πρότυπο (`tos_enforcer`, `welcome_greeter`, `top_comment_pinner`, `thread_summarizer`, `gaslight_detector`). Αν αυτό το όνομα χρησιμοποιείται ήδη στο tenant σας, προστίθεται ένας αριθμητικός επίθημα.
2. Προσγειώνεστε απευθείας στη φόρμα επεξεργασίας με όλα προ-συμπληρωμένα - prompt, triggers, allowed actions, και τυχόν thresholds. Μια ειδοποίηση στην κορυφή αναγράφει "Created from the {templateName} template. Review the settings below, then change status to Enabled when you're ready."
3. Τίποτα δεν είναι ακόμα ενεργοποιημένο. Ο πράκτορας δεν θα δράσει μέχρι να αποθηκεύσετε και είτε να κρατήσετε το dry-run ενεργό (για παρατήρηση) είτε να το αλλάξετε σε Enabled.

### The five templates

- **[Moderator](#template-moderator)** - εξετάζει νέα και σημασμένα σχόλια, προειδοποιεί παραβάτες για πρώτη φορά, κλιμακώνει σε ban μόνο μετά από προειδοποίηση. Triggers on new comments and on flag-threshold crossings (default flag threshold: 3). Allowed tools: `mark_comment_approved`, `mark_comment_spam`, `warn_user`, `ban_user`.

- **[Welcome Greeter](#template-welcome-greeter)** - απαντά θερμά σε σχολιαστές που γράφουν για πρώτη φορά με ένα σύντομο, προσωπικό καλωσόρισμα. Triggers on new-user-first-comment. Allowed tool: `write_comment`.

- **[Top Comment Pinner](#template-top-comment-pinner)** - καρφιτσώνει ουσιαστικά σχόλια κορυφαίου επιπέδου όταν ξεπερνούν ένα όριο ψήφων (default: 10), αφαιρώντας πρώτα την προηγουμένως καρφιτσωμένη απάντηση. Triggers on vote-threshold crossings. Allowed tools: `pin_comment`, `unpin_comment`.

- **[Thread Summarizer](#template-thread-summarizer)** - δημοσιεύει μια ουδέτερη, μονοπαραγραφο σύνοψη σε μεγάλες συζητήσεις μετά από καθυστέρηση, και στη συνέχεια τη καρφιτσώνει. Triggers on new comments with a 30-minute deferral so the thread settles before summarizing. Allowed tools: `write_comment`, `pin_comment`, `unpin_comment`.

- **[Gaslight Detector](#template-gaslight-detector)** - παρακολουθεί επεξεργασίες σχολίων για ενδιάμεσες αναθεωρήσεις που αλλοιώνουν τις απαντήσεις, αποκαθιστά το αρχικό κείμενο και στέλνει DM στον συγγραφέα. Triggers on comment edits. Allowed tools: `edit_comment`, `warn_user`, `send_dm`.

### Customizing a template

Τα πρότυπα είναι σημεία εκκίνησης, όχι δεσμεύσεις. Αναμένεται να:

- Τροποποιήσετε το **Initial prompt** ώστε να ταιριάζει με τη φωνή της κοινότητάς σας.
- Προσθέσετε ή αφαιρέσετε **Triggers** ώστε να ταιριάζουν με το πόσο συχνά πρέπει να ενεργοποιείται ο πράκτορας.
- Προσθέσετε **Approvals** για οποιαδήποτε ευαίσθητη ενέργεια - συστήνουμε έντονα να βάλετε `ban_user` πίσω από διαδικασία έγκρισης για πρότυπα τύπου moderator.
- Προσθέσετε **Community guidelines** ώστε ο πράκτορας να εφαρμόζει συνεπώς την γραπτή σας πολιτική. Δείτε [Community Guidelines](#community-guidelines).
- Ορίσετε ανά-πράκτορα **Budgets** κατάλληλα για τον αναμενόμενο αριθμό triggers.

Το πρότυπο είναι απλά ένα όχημα που προ-συμπληρώνει λογικές προεπιλογές· μόλις αποθηκευτεί, ο πράκτορας είναι δικός σας.