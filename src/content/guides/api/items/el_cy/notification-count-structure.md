Ένα αντικείμενο `NotificationCount` αντιπροσωπεύει τον αριθμό μη αναγνωσμένων ειδοποιήσεων και τα μεταδεδομένα για έναν χρήστη.

Αν δεν υπάρχουν μη αναγνωσμένες ειδοποιήσεις, δεν θα υπάρχει `NotificationCount` για τον χρήστη.

Τα αντικείμενα `NotificationCount` δημιουργούνται αυτόματα και δεν μπορούν να δημιουργηθούν μέσω του API. Επίσης λήγουν μετά από ένα έτος.

Μπορείτε να καθαρίσετε τον αριθμό μη αναγνωσμένων ειδοποιήσεων ενός χρήστη διαγράφοντας το `NotificationCount` του.

Η δομή για το αντικείμενο `NotificationCount` είναι η ακόλουθη:

[inline-code-attrs-start title = 'Δομή NotificationCount'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationCount {
    id: string // user id
    count: number
    createdAt: string // date string
    expireAt: string // date string
}
[inline-code-end]
