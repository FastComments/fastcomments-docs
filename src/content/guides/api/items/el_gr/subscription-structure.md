Ένα αντικείμενο `Subscription` αντιπροσωπεύει μια συνδρομή για έναν χρήστη.

Τα αντικείμενα `Subscription` δημιουργούνται όταν ένας χρήστης κάνει κλικ στο καμπανάκι ειδοποίησης στο widget σχολίων και κάνει κλικ στο "Εγγραφή σε αυτή τη σελίδα".

Οι συνδρομές μπορούν επίσης να δημιουργηθούν μέσω του API.

Η ύπαρξη ενός αντικειμένου `Subscription` προκαλεί τη δημιουργία αντικειμένων `Notification`, και την αποστολή emails, όταν νέα σχόλια αφήνονται στη ρίζα της σχετικής σελίδας
για την οποία είναι η `Subscription`. Η αποστολή emails εξαρτάται από τον τύπο του χρήστη. Για κανονικούς χρήστες αυτό εξαρτάται από το `optedInNotifications`. Για Χρήστες SSO αυτό εξαρτάται από το `optedInSubscriptionNotifications`. Σημειώστε ότι ορισμένες εφαρμογές μπορεί να μην έχουν την έννοια μιας σελίδας προσβάσιμης από το web, οπότε απλά ορίστε το `urlId` στο
id του στοιχείου στο οποίο εγγράφεστε (ίδια τιμή για το `urlId` που θα περνούσατε στο widget σχολίων).

Η δομή για το αντικείμενο `Subscription` είναι η ακόλουθη:

[inline-code-attrs-start title = 'Δομή Συνδρομής'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface Subscription {
    id: string
    tenantId: string
    /** With SSO, the user id is in the format `<tenant id>:<user id>`. **/
    userId: string
    anonUserId?: string
    urlId: string
    url?: string
    pageTitle?: string
    createdAt: string // date string
}
[inline-code-end]
