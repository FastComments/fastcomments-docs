Ένα **trigger** είναι ένα γεγονός που ενεργοποιεί έναν agent. Κάθε agent μπορεί να έχει έναν ή περισσότερους triggers ορισμένους.

### Η πλήρης λίστα

| Trigger | When it fires |
|---|---|
| [Comment Added](#trigger-comment-add) | Δημοσιεύεται ένα νέο σχόλιο. |
| [Comment Edited](#trigger-comment-edit) | Ένα σχόλιο επεξεργάζεται. Το προηγούμενο κείμενο περιλαμβάνεται στο context του agent. |
| [Comment Deleted](#trigger-comment-delete) | Ένα σχόλιο διαγράφεται. |
| [Comment Pinned](#trigger-comment-pin) | Ένα σχόλιο καρφιτσώνεται (από οποιονδήποτε, συμπεριλαμβανομένου ενός moderator ή άλλου agent). |
| [Comment Unpinned](#trigger-comment-unpin) | Ένα σχόλιο ξεκαρφιτσώνεται. |
| [Comment Locked](#trigger-comment-lock) | Ένα σχόλιο κλειδώνεται (δεν επιτρέπονται επιπλέον απαντήσεις). |
| [Comment Unlocked](#trigger-comment-unlock) | Ένα σχόλιο ξεκλειδώνεται. |
| [Comment Crosses Vote Threshold](#trigger-comment-vote-threshold) | Οι καθαρές ψήφοι ενός σχολίου φτάνουν το ρυθμισμένο όριο. |
| [Comment Crosses Flag Threshold](#trigger-comment-flag-threshold) | Ο αριθμός σημαδιών ενός σχολίου φτάνει ακριβώς το ρυθμισμένο όριο. |
| [User Posts First Comment](#trigger-new-user-first-comment) | Ένας χρήστης δημοσιεύει το πρώτο του σχόλιο σε αυτόν τον ιστότοπο. |
| [Comment Auto-Spammed](#trigger-comment-auto-spammed) | Ένα σχόλιο επισημαίνεται αυτόματα ως spam από τη μηχανή spam. |
| [Moderator Reviews Comment](#trigger-moderator-reviewed) | Ένας moderator επισημαίνει ένα σχόλιο ως ελεγμένο. |
| [Moderator Approves Comment](#trigger-moderator-approved) | Ένας moderator εγκρίνει ένα σχόλιο. |
| [Moderator Marks Spam](#trigger-moderator-spammed) | Ένας moderator επισημαίνει ένα σχόλιο ως spam. |
| [Moderator Awards Badge](#trigger-moderator-awarded-badge) | Ένας moderator απονέμει ένα badge σε έναν χρήστη. |

### Multiple triggers per agent

Ένας agent μπορεί να εγγραφεί σε οποιονδήποτε συνδυασμό triggers - το [Πρότυπο Moderator](#template-moderator) εγγράφεται σε παράδειγμα τόσο στα `COMMENT_ADD` όσο και `COMMENT_FLAG_THRESHOLD`. Κάθε γεγονός ενεργοποιεί τον agent μία φορά με το context του γεγονότος.

### Τι εμποδίζει την ενεργοποίηση ενός agent

Ένα εγγεγραμμένο trigger γεγονός **δεν** ενεργοποιεί τον agent αν ισχύει οποιοδήποτε από τα παρακάτω:

- Η [κατάσταση](#status-states) του agent είναι **Disabled**.
- Το [URL ή το πεδίο locale](#scope-url-locale) του agent δεν ταιριάζει με το σχόλιο που προκάλεσε το γεγονός.
- Ο [ημερήσιος, μηνιαίος ή προϋπολογισμός ορίου ρυθμού](#budgets-overview) του agent έχει εξαντληθεί - το trigger καταγράφεται ως **dropped** με αιτία. Δείτε [Drop Reasons](#drop-reasons).
- Η παραλληλία για αυτόν τον agent είναι κορεσμένη (όριο ανά agent).
- Ο tenant του agent έχει μη έγκυρη χρέωση.
- Η ενέργεια που προκάλεσε το trigger έγινε από bot ή άλλο agent (πρόληψη βρόχου).
- Το trigger αφορούσε σχόλιο που έχει ήδη επεξεργαστεί αυτός ο agent εντός του παραθύρου αφαίρεσης διπλοτύπων (deduplication window).

Όταν ένα εγγεγραμμένο trigger ενεργοποιηθεί επιτυχώς, το [Run History](#run-history) του agent εμφανίζει μια γραμμή με κατάσταση **Started** που μεταβαίνει σε **Success** ή **Error** όταν η εκτέλεση ολοκληρωθεί.

### Vote and flag thresholds

Δύο triggers - **Comment Crosses Vote Threshold** και **Comment Crosses Flag Threshold** - απαιτούν έναν αριθμητικό ουδό στη φόρμα επεξεργασίας. Το trigger ενεργοποιείται τη στιγμή που το πλήθος ξεπερνά την ρυθμισμένη τιμή (συγκεκριμένα, το flag-threshold trigger ενεργοποιείται όταν `flagCount === flagThreshold`, οπότε η επιλογή 1 σημαίνει "ενεργοποίηση με την πρώτη σημαία", και η επιλογή 5 σημαίνει "ενεργοποίηση όταν φτάσει η πέμπτη σημαία").

### Αναβαλλόμενα triggers

Οποιοδήποτε trigger μπορεί να αναβληθεί ώστε ο agent να εκτελεστεί αργότερα, για παράδειγμα αφού οι ψήφοι/σημαίες/απαντήσεις έχουν χρόνο να ομαλοποιηθούν. Δείτε [Deferred Triggers](#trigger-deferred-delay).

### Loop prevention

Για να αποφευχθούν άπειροι βρόχοι, τα σχόλια **γραφόμενα από έναν agent** φέρουν ένα `botId`. Οι trigger για νέα σχόλια αγνοούν σχόλια με `botId`.

Το καθαρό αποτέλεσμα: οι agents μπορούν να ενεργούν σε απάντηση σε *ανθρώπινες* ενέργειες στον tenant σας, αλλά ενέργειες που προέρχονται από agent ποτέ δεν ενεργοποιούν triggers agent. Αυτό ισχύει για όλους τους τύπους triggers.

### REPLAY: the internal trigger

Υπάρχει επίσης ένας εσωτερικός τύπος trigger `REPLAY` που χρησιμοποιείται από τη λειτουργία [Test Runs (Replays)](#test-runs-replays). Δεν μπορείτε να τον επιλέξετε στη φόρμα επεξεργασίας - υπάρχει ώστε οι επαναλήψεις να επισημαίνονται ξεχωριστά στο ιστορικό εκτελέσεων και να εξαιρούνται από τις προβολές ζωντανών εκτελέσεων.