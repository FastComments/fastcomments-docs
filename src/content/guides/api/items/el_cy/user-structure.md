Ο `User` είναι ένα αντικείμενο που αντιπροσωπεύει τον πιο κοινό παρονομαστή όλων των χρηστών.

Λάβετε υπόψη ότι στο FastComments έχουμε πολλές διαφορετικές περιπτώσεις χρήσης για χρήστες:

- Secure SSO
- Simple SSO
- Χρήστες Ενοικιαστή (Για παράδειγμα: Διαχειριστές)
- Σχολιαστές

Αυτό το API είναι για **Σχολιαστές** και χρήστες που δημιουργήθηκαν μέσω **Simple SSO**. Βασικά, οποιοσδήποτε χρήστης που δημιουργήθηκε
μέσω του ιστότοπού σας μπορεί να προσπελαστεί μέσω αυτού του API. Οι Χρήστες Ενοικιαστή μπορούν επίσης να ανακτηθούν με αυτόν τον τρόπο, αλλά θα λάβετε περισσότερες πληροφορίες αλληλεπιδρώντας με το API `/tenant-users/`.

Για `Secure SSO` παρακαλώ χρησιμοποιήστε το API `/sso-users/`.

Δεν μπορείτε να ενημερώσετε αυτούς τους τύπους χρηστών. Δημιούργησαν τον λογαριασμό τους μέσω του ιστότοπού σας, οπότε παρέχουμε κάποια βασική πρόσβαση μόνο για ανάγνωση, αλλά
δεν μπορείτε να κάνετε αλλαγές. Αν θέλετε να έχετε αυτόν τον τύπο ροής - πρέπει να ρυθμίσετε το `Secure SSO`.

Η δομή για το αντικείμενο `User` είναι η ακόλουθη:

[inline-code-attrs-start title = 'Δομή User'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
export interface User {
    /** This is also the id used as userId on comment objects. **/
    id: string
    username: string
    /** A link to the commenter's blog, for example. **/
    websiteUrl?: string
    email: string
    signUpDate: number
    createdFromUrlId: string
    createdFromTenantId: string
    avatarSrc?: string
    locale: FastCommentsLocale
    displayLabel?: string
    karma?: number
}
[inline-code-end]
