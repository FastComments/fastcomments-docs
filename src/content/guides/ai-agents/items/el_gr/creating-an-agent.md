Από τη σελίδα [AI Agents](https://fastcomments.com/auth/my-account/ai-agents) μπορείτε να δημιουργήσετε ένα agent με δύο τρόπους:

- **Από ένα πρότυπο.** Κάντε κλικ στο **Browse templates** και επιλέξτε ένα από τα τέσσερα ενσωματωμένα starter agents. Η φόρμα ανοίγει με προ-συμπληρωμένα πεδία και η κατάσταση του agent είναι **Dry Run**. Δείτε τα [Starter Templates](#starter-templates).
- **Από την αρχή.** Κάντε κλικ στο **Create new agent**. Η φόρμα ανοίγει κενή.

Σε κάθε περίπτωση, η ίδια φόρμα επεξεργασίας είναι αυτή που αποθηκεύετε και επεξεργάζεστε στη συνέχεια. Αυτή η σελίδα περιγράφει τη φόρμα από πάνω προς τα κάτω.

### Basics

- **Internal name.** Ένας σύντομος αναγνωριστικός που χρησιμοποιείται μόνο στα admin dashboards (run history, analytics, audit logs). Τα πεζά γράμματα με underscores λειτουργούν καλά: `moderator`, `welcome_greeter`. Αν το internal name ενός template έχει ήδη καταληφθεί, η φόρμα προσθέτει αυτόματα suffix (`tos_enforcer_2`, κ.λπ.).
- **Display name.** Εμφανίζεται δημόσια κάθε φορά που ο agent δημοσιεύει ένα σχόλιο. Αυτό βλέπουν οι αναγνώστες σας.
- **Status.** Disabled, Dry Run, or Enabled. Τα νέα agents έχουν πάντα προεπιλογή Dry Run. Δείτε [Status States](#status-states).

### Model

Επιλέξτε το LLM. Δείτε [Choosing a Model](#choosing-a-model).

### Budget

Προαιρετικά ημερήσια και μηνιαία όρια στο νόμισμα του λογαριασμού σας, καθώς και μια λίστα ελέγχου **Alert thresholds** (προεπιλογή 80% και 100%). Δείτε [Budgets Overview](#budgets-overview) και [Budget Alerts](#budget-alerts).

### Personality

Το **Initial prompt** είναι το system prompt που ορίζει τον τόνο, τον ρόλο και τους κανόνες απόφασης. Απλό κείμενο, χωρίς σύνταξη template. Δείτε [Personality and the Initial Prompt](#personality-prompt).

### Context

Το πεδίο Context περιλαμβάνει τρία checkboxes, ένα πλαίσιο κειμένου με οδηγίες και τα πεδία εύρους:

- Συμπερίληψη του γονικού σχολίου και προηγούμενων απαντήσεων στο ίδιο νήμα.
- Συμπερίληψη του trust factor του σχολιαστή, ηλικίας λογαριασμού, ιστορικού αποκλεισμών και πρόσφατων σχολίων.
- Συμπερίληψη τίτλου σελίδας, υπότιτλου, περιγραφής και meta tags.
- Ένα προαιρετικό μπλοκ κειμένου **Community guidelines** που προπορεύεται κάθε prompt.
- **Restrict to specific pages** - λίστα επιτρεπόμενων προτύπων URL (ένα ανά γραμμή). Κενό σημαίνει tenant-wide.
- **Restrict to specific locales** - λίστα επιτρεπόμενων τοπικών ρυθμίσεων μέσω επιλογέα διπλής λίστας. Κενό σημαίνει κάθε locale.

Περισσότερο πλαίσιο παράγει καλύτερες αποφάσεις αλλά αυξάνει το κόστος σε tokens ανά εκτέλεση. Δείτε [Context Options](#context-options), [Community Guidelines](#community-guidelines), και [Scope: URL and Locale Filters](#scope-url-locale).

### Triggers

Επιλέξτε τουλάχιστον ένα γεγονός από τη λίστα. Για τα triggers τύπου vote-threshold και flag-threshold πρέπει επίσης να ορίσετε το threshold. Το προαιρετικό πεδίο **Delay before running** αναβάλλει την εκτέλεση μετά την ενεργοποίηση ενός trigger (χρήσιμο για flag thresholds όπου οι ψήφοι μπορεί να έχουν ακόμη οριστεί). Δείτε [Trigger Events Overview](#triggers-overview) και [Deferred Triggers](#trigger-deferred-delay).

### Allowed tool calls

Τικάρετε **Allow any tool calls** για να εκθέσετε όλη την παλέτα εργαλείων. Διαφορετικά, τικάρετε τα συγκεκριμένα εργαλεία που επιτρέπεται να χρησιμοποιεί ο agent — τα μη επιτρεπόμενα εργαλεία αφαιρούνται από την παλέτα του μοντέλου και απορρίπτονται κατά το dispatch. Η υποενότητα **Ban options** περιορίζει τις καταστροφικές παραλλαγές ban (delete-all-comments, ban-by-IP) πίσω από ρητές opt-ins. Δείτε [Allowed Tool Calls Overview](#tools-overview) και [Tool: ban_user](#tool-ban-user).

### Approvals

Τικάρετε τις ενέργειες που πρέπει να εγκριθούν από άνθρωπο πριν τις εκτελέσει ο agent. Οι εγκρίσεις εφαρμόζονται μόνο σε εργαλεία που έχει δικαίωμα να επικαλεστεί ο agent· τα μη επιτρεπόμενα εργαλεία απορρίπτονται εξολοκλήρου. Στην περιοχή της ΕΕ, το **ban_user** είναι ενεργοποιημένο λόγω του Άρθρου 17 του Digital Services Act. Δείτε [Approval Workflow](#approval-workflow) και [EU DSA Article 17 Compliance](#eu-dsa-compliance).

### Approval notifications

Εάν οι εγκρίσεις είναι ενεργοποιημένες, επιλέξτε σε ποιον αποστέλλεται email:

- **All admins and moderators** - ιδιοκτήτες λογαριασμού, super admins και comment moderator admins.
- **Specific users** - επιλεγμένοι χειροκίνητα από επιλογέα διπλής λίστας.

Η μεμονωμένη συχνότητα παράδοσης κάθε reviewer (immediate, hourly digest, daily digest) ρυθμίζεται στο δικό του προφίλ. Δείτε [Approval Notifications](#approval-notifications).

### Stats

Μόνο για ανάγνωση. Συνολικές εκτελέσεις, χρονική σήμανση τελευταίας εκτέλεσης και το ID του πιο πρόσφατου σχολίου που έγραψε ο agent (εάν υπάρχει).

### Save

Κάντε κλικ στο **Save agent**. Η σελίδα ανακατευθύνει πίσω στη λίστα agents. Τα νέα agents είναι άμεσα επιλέξιμα για να λαμβάνουν triggers σε dry-run.

### Editing later

Κάθε σειρά στη σελίδα λίστας agents εμφανίζει ενέργειες ανά agent: **Edit**, **Clone**, **Runs**, **Replays**, **Test run**, **Analytics**, **Approvals**, και **Delete**. Η επεξεργασία ενός agent δεν επηρεάζει εκ των υστέρων ήδη καταγεγραμμένες εκτελέσεις — το history διατηρείται. Τα replay snapshots επίσης παγώνουν τη ρύθμιση του agent στο σημείο που ξεκίνησε το replay, έτσι ώστε τα αποτελέσματα ενός αποθηκευμένου replay να παραμένουν αναπαραγώγιμα ακόμα και μετά την επεξεργασία του prompt.