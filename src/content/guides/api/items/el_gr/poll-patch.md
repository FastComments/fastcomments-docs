[api-resource-header-start name = 'Poll'; route = 'PATCH /api/v1/polls/:commentId'; creditsCost = 1; api-resource-header-end]

Επεξεργάζεται μια δημοσκόπηση χωρίς να διαταράσσει τις ψήφους της. Χρησιμοποιήστε το για να διορθώσετε ένα τυπογραφικό λάθος στην ερώτηση ή σε μια επιλογή, για να κλείσετε ή
να ξαναανοίξετε τη δημοσκόπηση, ή για να αλλάξετε ποιος μπορεί να δει ποιος ψήφισε.

Οι επιλογές αναφέρονται με το `id` τους, και ένα `PATCH` μετονομάζει αυτές που ορίζετε. Για να προσθέσετε, αφαιρέσετε ή αναδιατάξετε
τις επιλογές, στείλτε τη πλήρη λίστα επιλογών στο `PUT /api/v1/polls/:commentId`: οι επιλογές που στέλνετε με τα ids τους διατηρούν
τις ψήφους τους επίσης.

Κάθε πεδίο είναι προαιρετικό, αλλά πρέπει να δοθεί τουλάχιστον ένα.

[inline-code-attrs-start title = 'Παράδειγμα cURL για Ενημέρωση Δημοσκόπησης'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/polls/comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"question": "Which release should we cut first?",
	"options": [{"id": "the-option-id", "label": "The bugfix release"}]
}'
[inline-code-end]

[inline-code-attrs-start title = 'Παράδειγμα cURL για Κλείσιμο Δημοσκόπησης Τώρα'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/polls/comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"closesAt": "2020-01-01T00:00:00.000Z"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Αιτήματος Ενημέρωσης Δημοσκόπησης'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollPatchQueryParams {
    tenantId: string
    API_KEY: string
}

interface PollPatchBody {
    question?: string | null
    /** Relabels existing options. Every id given must already be on the poll. **/
    options?: { id: string, label: string }[] | null
    /** A date in the past closes the poll now. null reopens a closed poll. **/
    closesAt?: string | null
    /** 0 anonymous, 1 admins and moderators, 2 everyone. **/
    privacy?: 0 | 1 | 2 | null
    requireVoteToSeeResults?: boolean | null
}
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Απόκρισης Ενημέρωσης Δημοσκόπησης'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollPatchResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'poll-not-found' | 'poll-invalid' | 'poll-privacy-locked' | 'locked'
    /** Included on failure. **/
    reason?: string
    poll: CommentPoll
}
[inline-code-end]

### Άλλες Σημειώσεις

- Η ονομασία ενός id επιλογής που δεν υπάρχει στη δημοσκόπηση αποτυγχάνει με `poll-invalid` αντί να μην κάνει τίποτα σιωπηρά.
- Οι ετικέτες πρέπει να παραμένουν μοναδικές μέσα στη δημοσκόπηση, λαμβάνοντας υπόψη τις επιλογές που δεν αλλάζετε.
- Σε αντίθεση με τη δημιουργία μιας δημοσκόπησης, το `closesAt` μπορεί να είναι στο παρελθόν εδώ - αυτός είναι ο τρόπος για να κλείσετε μια δημοσκόπηση αμέσως.
- Η ιδιωτικότητα της δημοσκόπησης μπορεί να περιοριστεί αλλά όχι να επεκταθεί μόλις έχει ψήφους.
- Ένα κλειδωμένο σχόλιο δεν μπορεί να αλλάξει τη δημοσκόπηση του, και αποτυγχάνει με `locked`.