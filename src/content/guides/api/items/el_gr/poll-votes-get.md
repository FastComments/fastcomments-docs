[api-resource-header-start name = 'PollVote'; route = 'GET /api/v1/poll-votes'; creditsCost = 1; api-resource-header-end]

Καταγράφει τις μεμονωμένες ψήφους πίσω από τις μετρήσεις μιας δημοσκόπησης, από την παλαιότερη προς τη νεότερη. Ένα credit ανά 100 ψήφους που επιστρέφονται.

Μια δημοσκόπηση ανήκει σε ένα σχόλιο, επομένως οι ψήφοι διαβάζονται μία δημοσκόπηση τη φορά και απαιτείται το `commentId`. Μπορείτε να περιορίσετε περαιτέρω με το `voterId` για να ελέγξετε πώς ψήφισε ένα άτομο, ή με το `optionId` για να εμφανίσετε όλους όσους επέλεξαν μια συγκεκριμένη επιλογή.

Μέγιστο αριθμό 1000 ψήφων επιστρέφονται ανά κλήση. Χρησιμοποιήστε το `skip` για σελιδοποίηση.

Η ρύθμιση `privacy` της δημοσκόπησης τηρείται: οι ψήφοι σε ανώνυμη δημοσκόπηση δεν μπορούν να διαβαστούν, και το αίτημα αποτυγχάνει με `poll-anonymous`. Δείτε τη δομή `PollVote` για λεπτομέρειες.

[inline-code-attrs-start title = 'Παράδειγμα cURL για Λήψη PollVotes'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/poll-votes?tenantId=demo&API_KEY=DEMO_API_SECRET&commentId=comment-id'
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Αιτήματος Λήψης PollVotes'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVotesGetQueryParams {
    tenantId: string
    API_KEY: string
    commentId: string
    voterId?: string
    optionId?: string
    skip?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Απόκρισης Λήψης PollVotes'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVotesGetResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-comment-id' | 'poll-not-found' | 'poll-anonymous'
    /** Included on failure. **/
    reason?: string
    pollVotes: PollVote[]
}
[inline-code-end]

### Καταμέτρηση Ψήφων ανά Επιλογή

Δεν χρειάζεται να τα προσθέσετε για να λάβετε τα αποτελέσματα – η δημοσκόπηση διαθέτει τις δικές της μετρήσεις. Διαβάστε τη δημοσκόπηση με `GET /api/v1/polls/:commentId` αντί αυτού, και χρησιμοποιήστε αυτό το API όταν χρειάζεται να γνωρίζετε ποιος ψήφισε.

### Κάθε Δημοσκόπηση Σε Μια Σελίδα

Δεν υπάρχει λίστα ψήφων για ολόκληρη τη σελίδα. Για να αναφέρετε μια ολόκληρη σελίδα, φέρετε τα σχόλιά της με `GET /api/v1/comments`, το οποίο επιστρέφει τη δημοσκόπηση κάθε σχολίου και τις μετρήσεις της, και στη συνέχεια διαβάστε τις ψήφους για τις δημοσκοπήσεις που σας ενδιαφέρουν.

---