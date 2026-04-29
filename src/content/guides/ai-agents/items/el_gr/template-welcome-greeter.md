**ID Προτύπου:** `welcome_greeter`

Το Welcome Greeter απαντά θερμά σε χρήστες που σχολιάζουν για πρώτη φορά. Είναι το πρότυπο με τον πιο χαμηλό κίνδυνο (χωρίς καταστροφικά εργαλεία) και ένας καλός πρώτος agent για να βγάλεις live.

### Ενσωματωμένη αρχική προτροπή

[inline-code-attrs-start title = 'Αρχική Προτροπή Προτύπου Welcome Greeter'; type='text' inline-code-attrs-end]
[inline-code-start]
You are a warm community greeter. Reply to first-time commenters with a short, personal welcome. Mention one specific thing from their comment so it does not read as a template. Keep replies to 1-2 sentences. Never reply to accounts more than 24 hours old.
[inline-code-end]

### Ενεργοποιητές

- **New user posts their first comment on this site** (`NEW_USER_FIRST_COMMENT`).

Αυτό το συμβάν ενεργοποιείται ακριβώς μία φορά ανά χρήστη, επομένως ο agent δεν μπορεί να κάνει loop. Δείτε [Trigger: New User First Comment](#trigger-new-user-first-comment).

### Επιτρεπόμενα εργαλεία

- [`write_comment`](#tools-overview)

Αυτό είναι το μόνο εργαλείο — ο agent κυριολεκτικά δεν μπορεί να μετριάσει, να ψηφίσει, να αποκλείσει ή να στείλει DM.

### Προτεινόμενες προσθήκες πριν την ενεργοποίηση

- **Ορίστε το Display name** σε κάτι φιλικό - "Community Bot", τη μασκότ του ιστότοπού σας, ή το όνομα του brand σας. Το display name είναι αυτό που βλέπουν οι αναγνώστες συνδεδεμένο με την απάντηση καλωσορίσματος.
- **Επιλέξτε το "Include page title, subtitle, description, and meta tags"** στις [Context Options](#context-options). Οι απαντήσεις του χαιρετιστή βελτιώνονται αισθητά όταν μπορεί να αναφερθεί σε αυτό για το οποίο αφορά πραγματικά η σελίδα.
- **Σκεφτείτε περιορισμούς locale** αν λειτουργείτε σε πολλές γλώσσες. Μια απάντηση καλωσορίσματος στη λάθος γλώσσα είναι πιο ενοχλητική από το να μην απαντήσετε καθόλου. Δείτε [Scope: URL and Locale Filters](#scope-url-locale).

### Γιατί δεν χρειάζονται εγκρίσεις

Ο agent γράφει μόνο νέα σχόλια και μόνο σε ένα one-shot trigger. Στην χειρότερη περίπτωση: ένας αμήχανος χαιρετισμός. Δεν υπάρχει καμία καταστροφική ενέργεια που να απαιτεί έλεγχο. Οι περισσότεροι διαχειριστές το τρέχουν χωρίς καθόλου εγκρίσεις μόλις το dry-run φαίνεται καθαρό.