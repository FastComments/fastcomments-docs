---
FastComments παρέχει τέσσερα πρότυπα εκκίνησης ώστε να μην χρειάζεται να γράψετε έναν λειτουργικό agent από το μηδέν. Είναι προσβάσιμα από τη σελίδα [AI Agents](https://fastcomments.com/auth/my-account/ai-agents) κάνοντας κλικ στο **Περιήγηση προτύπων**.

When you pick a template:

1. Ο agent δημιουργείται με **Κατάσταση: Dry Run** και ένα εσωτερικό όνομα βασισμένο στο πρότυπο (`tos_enforcer`, `welcome_greeter`, `top_comment_pinner`, `thread_summarizer`). Αν αυτό το όνομα υπάρχει ήδη στον tenant σας, προστίθεται αριθμητικό επίθημα.
2. Κατευθύνεστε απευθείας στη φόρμα επεξεργασίας με όλα προ-συμπληρωμένα - prompt, triggers, allowed actions, and any thresholds. Μια κορδέλα στο επάνω μέρος αναγράφει "Created from the {templateName} template. Review the settings below, then change status to Enabled when you're ready."
3. Τίποτα δεν είναι ενεργοποιημένο ακόμη. Ο agent δεν θα αναλάβει δράση μέχρι να αποθηκεύσετε και είτε να διατηρήσετε το dry-run ενεργό (για παρακολούθηση) είτε να το αλλάξετε σε Enabled.

### The four templates

- **[Moderator](#template-moderator)** - επανεξετάζει νέα και σημειωμένα σχόλια, προειδοποιεί τους παραβάτες για πρώτη φορά, και κλιμακώνει σε ban μόνο μετά από προειδοποίηση. Triggers on new comments and on flag-threshold crossings (default flag threshold: 3). Επιτρεπόμενα εργαλεία: `mark_comment_approved`, `mark_comment_spam`, `warn_user`, `ban_user`.

- **[Welcome Greeter](#template-welcome-greeter)** - απαντά θερμά σε χρήστες που σχολιάζουν για πρώτη φορά με ένα σύντομο, προσωπικό καλωσόρισμα. Triggers on new-user-first-comment. Επιτρεπόμενο εργαλείο: `write_comment`.

- **[Top Comment Pinner](#template-top-comment-pinner)** - καρφιτσώνει ουσιώδη σχόλια ανώτατου επιπέδου μόλις ξεπεράσουν ένα όριο ψήφων (default: 10), αποκαρφιτσώνοντας πρώτα το προηγουμένως καρφιτσωμένο σχόλιο. Triggers on vote-threshold crossings. Επιτρεπόμενα εργαλεία: `pin_comment`, `unpin_comment`.

- **[Thread Summarizer](#template-thread-summarizer)** - δημοσιεύει μια ουδέτερη, μονοπαραγραφική περίληψη σε μακρές συνομιλίες μετά από καθυστέρηση και στη συνέχεια την καρφιτσώνει. Triggers on new comments with a 30-minute deferral so the thread settles before summarizing. Επιτρεπόμενα εργαλεία: `write_comment`, `pin_comment`, `unpin_comment`.

### Customizing a template

Τα πρότυπα είναι σημεία εκκίνησης, όχι δεσμεύσεις. Αναμένεται να:

- Τροποποιήσετε το **Initial prompt** ώστε να ταιριάζει με τη φωνή της κοινότητάς σας.
- Προσθέσετε ή αφαιρέσετε **Triggers** ώστε να ταιριάζει το πόσο συχνά πρέπει να εκτελείται ο agent.
- Προσθέσετε **Approvals** για οποιαδήποτε ευαίσθητη ενέργεια — συνιστούμε έντονα να βάλετε το `ban_user` πίσω από διαδικασία έγκρισης για πρότυπα τύπου moderator.
- Προσθέσετε **Community guidelines** ώστε ο agent να εφαρμόζει την γραπτή σας πολιτική με συνέπεια. Δείτε [Community Guidelines](#community-guidelines).
- Ορίσετε ανά-agent **Budgets** κατάλληλους για το πόσους triggers αναμένετε.

Το πρότυπο είναι απλώς ένα όχημα που προ-συμπληρώνει λογικές προεπιλογές· μόλις αποθηκευτεί, ο agent είναι δικός σας.

---