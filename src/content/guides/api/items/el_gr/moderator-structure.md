Ένα αντικείμενο `Moderator` αντιπροσωπεύει τη διαμόρφωση για έναν συντονιστή.

Υπάρχουν τρεις τύποι συντονιστών:

1. Χρήστες διαχειριστές που έχουν τη σημαία `isCommentModeratorAdmin`.
2. Χρήστες SSO με τη σημαία `isCommentModeratorAdmin`.
3. Κανονικοί σχολιαστές, ή χρήστες FastComments.com, που προσκαλούνται ως Συντονιστές.

Η δομή `Moderator` χρησιμοποιείται για να αναπαραστήσει την Κατάσταση Συντονισμού της περίπτωσης χρήσης `3`.

Αν θέλετε να προσκαλέσετε έναν χρήστη να γίνει συντονιστής, μέσω του API, χρησιμοποιήστε το API `Moderator` δημιουργώντας έναν `Moderator` και `προσκαλώντας` τον.

Αν ο χρήστης δεν έχει λογαριασμό FastComments.com, το email πρόσκλησης θα τον βοηθήσει να ρυθμίσει. Αν έχει ήδη λογαριασμό, θα του
δοθεί πρόσβαση συντονισμού στον ενοικιαστή σας και το `userId` του αντικειμένου `Moderator` θα ενημερωθεί για να δείχνει στον χρήστη του. Δεν θα έχετε πρόσβαση API
στον χρήστη τους, καθώς σε αυτή την περίπτωση ανήκει στους ίδιους και διαχειρίζεται από το FastComments.com.

Αν απαιτείτε πλήρη διαχείριση του λογαριασμού του χρήστη, συνιστούμε είτε τη χρήση SSO, είτε την προσθήκη τους ως [Χρήστη Ενοικιαστή](https://fastcomments.com/auth/my-account/users) και
μετά την προσθήκη ενός αντικειμένου `Moderator` για την παρακολούθηση των στατιστικών τους.

Η δομή `Moderator` μπορεί να χρησιμοποιηθεί ως μηχανισμός παρακολούθησης στατιστικών για τις περιπτώσεις χρήσης `1` και `2`. Μετά τη δημιουργία του χρήστη, προσθέστε ένα αντικείμενο `Moderator`
με το `userId` τους ορισμένο και τα στατιστικά τους θα παρακολουθούνται στη [Σελίδα Συντονιστών Σχολίων](https://fastcomments.com/auth/my-account/moderate-comments/moderators).

Η δομή για το αντικείμενο `Moderator` είναι η ακόλουθη:

[inline-code-attrs-start title = 'Δομή Συντονιστή'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface Moderator {
    name: string
    email: string
    tenantId: string
    userId?: string|null
    acceptedInvite?: boolean
    markReviewedCount?: number
    deletedCount?: number
    markedSpamCount?: number
    approvedCount?: number
    editedCount?: number
    bannedCount?: number
    createdAt: string
    moderationGroupIds?: string[]|null
}
[inline-code-end]
