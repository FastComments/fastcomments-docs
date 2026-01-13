Ένα αντικείμενο `HashTag` αντιπροσωπεύει ένα tag που μπορεί να αφεθεί από έναν χρήστη. Τα HashTags μπορούν να χρησιμοποιηθούν για σύνδεση με εξωτερικό περιεχόμενο ή για
σύνδεση σχετικών σχολίων μεταξύ τους.

Η δομή για το αντικείμενο `HashTag` είναι η ακόλουθη:

[inline-code-attrs-start title = 'Δομή HashTag'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface HashTag {
    /** Should start with the "#" or desired character. **/
    tag: string
    /** An optional URL that the hashtag can point to. Instead of filtering comments by hashtag, the UI will redirect to this upon click. **/
    url?: string
    /** READONLY **/
    createdAt: string
}
[inline-code-end]

Σημειώσεις:

- Σε ορισμένα API endpoints θα δείτε ότι το hashtag χρησιμοποιείται στο URL. Θυμηθείτε να κωδικοποιήσετε τις τιμές URI. Για παράδειγμα, το `#` θα πρέπει αντ' αυτού να αναπαρίσταται ως `%23`.
- Ορισμένα από αυτά τα πεδία είναι επισημασμένα ως `READONLY` - αυτά επιστρέφονται από το API αλλά δεν μπορούν να οριστούν.
