An **AI Agent** είναι ένας αυτόνομος εργαζόμενος, περιορισμένος στον ενοικιαστή FastComments σας, που παρακολουθεί τα γεγονότα στην κοινότητά σας και ενεργεί εκ μέρους σας.

Each agent has three things you control:

1. **Μια προσωπικότητα.** Ένα ελεύθερο κείμενο αρχικής προτροπής που ορίζει τον τόνο, το ρόλο και το στυλ λήψης αποφάσεων («Είστε ένας ζεστός χαιρετιστής της κοινότητας», «Εφαρμόζετε τους κανόνες της κοινότητας αλλά προτιμάτε την προειδοποίηση αντί της απαγόρευσης», κ.λπ.).
2. **Ένα ή περισσότερα ερεθίσματα.** Μια λίστα γεγονότων που ξυπνούν τον πράκτορα - ένα νέο σχόλιο, ένα σχόλιο που ξεπερνά ένα όριο ψήφων ή σημαδιών, μια ενέργεια συντονιστή, το πρώτο σχόλιο ενός χρήστη στον ιστότοπο, και άλλα. Η πλήρης λίστα βρίσκεται στο [Trigger Events Overview](#triggers-overview).
3. **Μια λίστα επιτρεπόμενων εργαλείων.** Τι επιτρέπεται να κάνει ο πράκτορας - να δημοσιεύει ένα σχόλιο, να ψηφίζει, να καρφιτσωθεί, να κλειδώνει, να σημειώνει ως spam, να απαγορεύει έναν χρήστη, να προειδοποιεί μέσω DM, να απονέμει έμβλημα, να στέλνει email, να αποθηκεύει και να αναζητά κοινή μνήμη. Η πλήρης λίστα βρίσκεται στο [Allowed Tool Calls Overview](#tools-overview).

When a trigger fires, the agent receives a context message describing what happened (the comment, the page, optional thread/user/page context) and is prompted with its initial prompt and your community guidelines. It then calls tools to act, recording a justification and a confidence score with every call.

### Agents run asynchronously

Agents **never block the user action that triggered them**. A reader posts a comment, the comment is saved and shown to the thread, the response is returned, and only *then* does the agent run on it - either immediately or after a configured delay (see [Deferred Triggers](#trigger-deferred-delay)). Nothing the agent does adds latency to the user-facing experience.

### Why use them

- **Μέτρηση σε κλίμακα.** Σημειώστε προφανές spam και απαγορεύστε επαναλαμβανόμενους παραβάτες χωρίς να παρακολουθείτε την ουρά όλο το 24ωρο.
- **Καλωσορίστε νέους σχολιαστές.** Απαντήστε σε σχολιαστές που γράφουν για πρώτη φορά με τη φωνή σας.
- **Αναδείξτε το καλύτερο περιεχόμενο.** Καρφιτσώστε ουσιαστικά σχόλια πρώτου επιπέδου μόλις περάσουν ένα όριο ψήφων.
- **Εφαρμόστε τις οδηγίες σας σταθερά.** Εφαρμόστε το ίδιο κείμενο πολιτικής σε κάθε σχόλιο που βρίσκεται στο όριο.
- **Συνοψίστε μακριά νήματα.** Δημοσιεύστε ουδέσες συνοψίσεις πολυ-σελιδωτών συζητήσεων.

### What keeps you in control

- **Λειτουργία δοκιμής (Dry-run).** Κάθε νέος πράκτορας κυκλοφορεί σε **Dry Run**: επεξεργάζεται τα ερεθίσματα, τρέχει το μοντέλο και καταγράφει τι *θα* έκανε, αλλά δεν εκτελεί πραγματικές ενέργειες. Δείτε [Dry-Run Mode](#dry-run-mode).
- **Εγκρίσεις.** Οποιοδήποτε υποσύνολο ενεργειών μπορεί να απαιτεί ανθρώπινη έγκριση. Δείτε [Approval Workflow](#approval-workflow).
- **Προϋπολογισμοί ανά πράκτορα και ανά λογαριασμό.** Σκληρά ημερήσια και μηνιαία όρια. Δείτε [Budgets Overview](#budgets-overview).
- **Λίστα επιτρεπόμενων εργαλείων.** Τα μη επιτρεπόμενα εργαλεία αφαιρούνται από την παλέτα του μοντέλου - ο πράκτορας κυριολεκτικά δεν μπορεί να τα ζητήσει. Δείτε [Allowed Tool Calls Overview](#tools-overview).
- **Πεδία ελέγχου σε κάθε ενέργεια.** Το μοντέλο πρέπει να περιλαμβάνει αιτιολόγηση και σκορ εμπιστοσύνης. Και τα δύο εμφανίζονται στη χρονογραμμή εκτέλεσης και σε κάθε έγκριση. Δείτε [Run Detail View](#run-detail-view).
- **EU DSA Άρθρο 17.** Στην περιοχή της ΕΕ, οι πλήρως αυτοματοποιημένες απαγορεύσεις μπλοκάρονται. Δείτε [EU DSA Article 17 Compliance](#eu-dsa-compliance).
- **Καμία εκπαίδευση στα δεδομένα σας.** Η FastComments χρησιμοποιεί παρόχους που δεν εκπαιδεύονται στα προτροπές ή τα σχόλιά σας.

### Where they fit alongside human moderation

Agents and human moderators share the same comment platform: agents take actions through the same channels (approve, spam, ban, badge, pin, lock, write) and those actions appear in the same [Comment Logs](/guide-moderation.html#comment-logs), the same [Moderation Page](/guide-moderation.html#moderate-comments-page), and the same notification streams. Agents and humans see each other's work and can each react to the other - moderator actions are themselves valid agent triggers (see [Trigger: Moderator Reviewed Comment](#trigger-moderator-reviewed) and friends).

If you need people on the queue as well, FastComments offers [Human Moderation Services](/guide-moderation.html#human-moderation-services): our moderators work alongside your agents, in your dashboard, under your guidelines.