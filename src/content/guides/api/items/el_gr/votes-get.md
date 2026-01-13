[api-resource-header-start name = 'Vote'; route = 'GET /api/v1/votes'; creditsCost = 10; api-resource-header-end]

Οι ψήφοι πρέπει να ανακτηθούν κατά `urlId`.

### Τύποι Ψήφων

Υπάρχουν τρεις τύποι ψήφων:

- Επαληθευμένες Ψήφοι, που εφαρμόζονται στο αντίστοιχο σχόλιο. Μπορείτε να τις δημιουργήσετε μέσω αυτού του API.
- Επαληθευμένες Ψήφοι, που **αναμένουν** επαλήθευση, και επομένως δεν έχουν ακόμα εφαρμοστεί στο σχόλιο. Αυτές δημιουργούνται όταν ένας χρήστης χρησιμοποιεί τον μηχανισμό *σύνδεση για ψηφοφορία* του FastComments.com.
- Ανώνυμες Ψήφοι, που εφαρμόζονται στο αντίστοιχο σχόλιο. Αυτές δημιουργούνται μαζί με τα ανώνυμα σχόλια.

Αυτές επιστρέφονται σε ξεχωριστές λίστες στο API για μείωση της σύγχυσης.

[inline-code-attrs-start title = 'Votes cURL Παράδειγμα'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/votes?tenantId=demo&API_KEY=DEMO_API_SECRET&urlId=test'
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Αιτήματος Votes'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotesRequestQueryParams {
    tenantId: string
    API_KEY: string
    urlId: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Απάντησης Votes'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotesResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id'
    /** Included on failure. **/
    reason?: string
    /** Authorized, verified votes, applied to their corresponding comments. **/
    appliedAuthorizedVotes: Vote[]
    /** Anonymous votes, applied to their corresponding comments. **/
    appliedAnonymousVotes: Vote[]
    /** Votes pending verification, not yet applied to their corresponding comments. **/
    pendingVotes: Vote[]
}
[inline-code-end]

#### Σημειώσεις Ανώνυμων Ψήφων

Σημειώστε ότι οι ανώνυμες ψήφοι που δημιουργήθηκαν μέσω αυτού του API θα εμφανιστούν στη λίστα `appliedAuthorizedVotes`. Θεωρούνται εξουσιοδοτημένες αφού δημιουργήθηκαν μέσω του API με κλειδί API.

Η δομή `appliedAnonymousVotes` είναι για ψήφους που δημιουργήθηκαν χωρίς email, κλειδί API, κλπ.
