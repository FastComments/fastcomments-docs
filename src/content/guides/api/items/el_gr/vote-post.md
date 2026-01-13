[api-resource-header-start name = 'Vote'; route = 'POST /api/v1/votes'; creditsCost = 1; api-resource-header-end]

Αυτή η διαδρομή παρέχει τη δυνατότητα προσθήκης μιας μόνου εξουσιοδοτημένης `Vote`. Οι ψήφοι μπορούν να είναι `up` (+1) ή `down` (-1).

[inline-code-attrs-start title = 'Δημιουργία Vote cURL Παράδειγμα'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/votes?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=user-id&commentId=comment-id&direction=up' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = 'Δημιουργία Ανώνυμης Vote cURL Παράδειγμα'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/votes?tenantId=demo&API_KEY=DEMO_API_SECRET&anonUserId=some-randomly-generated-identifier&commentId=comment-id&direction=up' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Αιτήματος Δημιουργίας Vote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotePostQueryParams {
    tenantId: string
    API_KEY: string
    userId?: string
    anonUserId?: string
    commentId: string
    direction: 'up' | 'down'
}
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Απάντησης Δημιουργίας Vote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotePostResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-user-id' | 'invalid-user-id' | 'invalid-comment-id' | 'invalid-direction' | 'duplicate' | 'voting-disabled'
    /** Included on failure. **/
    reason?: string
    voteId?: string
}
[inline-code-end]

### Δημιουργία Ανώνυμων Ψήφων

Οι ανώνυμες ψήφοι μπορούν να δημιουργηθούν ορίζοντας το `anonUserId` στις παραμέτρους ερωτήματος αντί του `userId`.

Αυτό το id δεν χρειάζεται να αντιστοιχεί σε αντικείμενο χρήστη οπουδήποτε (εξ ου και ανώνυμο). Είναι απλά ένα αναγνωριστικό
για τη συνεδρία, ώστε να μπορείτε να ανακτήσετε ψήφους ξανά στην ίδια συνεδρία, για να ελέγξετε αν ένα σχόλιο έχει
ψηφιστεί.

Αν δεν έχετε κάτι σαν "ανώνυμες συνεδρίες" όπως το FastComments - μπορείτε απλά
να το ορίσετε σε ένα τυχαίο ID, όπως ένα UUID (αν και εκτιμούμε μικρότερα αναγνωριστικά για εξοικονόμηση χώρου).

### Άλλες Σημειώσεις

- Αυτό το API υπακούει τις ρυθμίσεις σε επίπεδο ενοικιαστή. Για παράδειγμα, αν απενεργοποιήσετε την ψηφοφορία για μια δεδομένη σελίδα, και προσπαθήσετε να δημιουργήσετε μια ψήφο μέσω του API, θα αποτύχει με κωδικό σφάλματος `voting-disabled`.
- Αυτό το API είναι ζωντανό από προεπιλογή.
- Αυτό το API θα ενημερώσει τα `votes` του αντίστοιχου `Comment`.
