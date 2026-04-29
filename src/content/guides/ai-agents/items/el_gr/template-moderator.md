**Template ID:** `tos_enforcer`

Το πρότυπο Moderator είναι το συνιστώμενο σημείο εκκίνησης εάν ο στόχος σας είναι να μειώσετε το φόρτο χειροκίνητης εποπτείας. Εξετάζει νέα και επισημασμένα σχόλια και εφαρμόζει τους κανόνες της κοινότητάς σας.

Σχεδόν πάντα θα θέλετε να **ενισχύσετε το ενσωματωμένο prompt** με συγκεκριμένα παραδείγματα του τι επιτρέπει και τι δεν επιτρέπει ο ιστότοπός σας. Η ίδια η πολιτική κλιμάκωσης της πλατφόρμας (προειδοποίηση πριν από την απαγόρευση, αναζήτηση στη μνήμη πριν από την απαγόρευση) είναι ήδη ενσωματωμένη στο system prompt που λαμβάνει ο agent, οπότε δεν χρειάζεται να την επαναλάβετε.

### Triggers

- **New comment posted** (`COMMENT_ADD`) - ο agent εξετάζει κάθε νέο σχόλιο.
- **Comment crosses a flag threshold** (`COMMENT_FLAG_THRESHOLD`, default threshold: 3) - ο agent επανεκτιμά ένα σχόλιο που έχουν επισημάνει άλλοι χρήστες.

### Allowed tools

- [`mark_comment_approved`](#tools-overview) - χρήσιμο για tenants με προ-εποπτεία όπου ο agent απελευθερώνει τα καθαρά σχόλια και αποκρύπτει τα υπόλοιπα.
- [`mark_comment_spam`](#tools-overview)
- [`warn_user`](#tool-warn-user)
- [`ban_user`](#tool-ban-user)

Δεν μπορεί να δημοσιεύει σχόλια, να ψηφίζει, να καρφιτσώνει, να κλειδώνει, να απονέμει διακριτικά ή να στέλνει email - το prompt είναι εσκεμμένα περιορισμένο.

### Recommended additions before going live

- **Set [Community Guidelines](#community-guidelines).** Λίγες προτάσεις γραπτής πολιτικής αρκούν· ο agent τις εφαρμόζει σε κάθε εκτέλεση.
- **Gate `ban_user` behind [approval](#approval-workflow).** Αυτό είναι ενεργό από προεπιλογή στην περιοχή της ΕΕ (βλ. [EU DSA Article 17 Compliance](#eu-dsa-compliance)) και συνιστάται παντού.
- **Consider also gating `mark_comment_spam` behind approval** εάν έχετε χαμηλό όγκο αλλά υψηλό ρίσκο περιεχομένου.
- **Gate `mark_comment_approved` behind approval if you run pre-moderation.** Η έγκριση ενός κακού σχολίου το βάζει μπροστά σε αναγνώστες· περιορίστε την μέχρι ο agent να κερδίσει εμπιστοσύνη μέσω dry-run.
- **Tick "Include commenter's trust factor, account age, ban history, and recent comments"** στις [Context Options](#context-options). Το μοντέλο θα προειδοποιεί πολύ λιγότερο επιθετικά όταν μπορεί να δει ότι κάποιος είναι μακροχρόνιος χρήστης με καλή πίστη.

### Recommended dry-run window

Τρέξτε αυτό το πρότυπο σε [dry-run](#dry-run-mode) για τουλάχιστον μία εβδομάδα με την πραγματική σας κίνηση πριν το ενεργοποιήσετε. Χρησιμοποιήστε τα [Test Runs (Replays)](#test-runs-replays) για να προεπισκόπηση και για τις προηγούμενες 30 ημέρες.