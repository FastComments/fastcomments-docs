[api-resource-header-start name = 'Poll'; route = 'PUT /api/v1/polls/:commentId'; creditsCost = 1; api-resource-header-end]

Συνημίζει μια δημοσκόπηση σε ένα υπάρχον σχόλιο ή ορίζει την πλήρη κατάσταση της δημοσκόπησης που ήδη υπάρχει.

Το σώμα είναι η πλήρης δημοσκόπηση, και οι επιλογές που στέλνετε γίνονται οι επιλογές της δημοσκόπησης, με αυτή τη σειρά. Κάθε επιλογή ταιριάζει με το `id` της:

- Μια επιλογή που αποστέλλεται με το `id` μιας υπάρχουσας επιλογής διατηρεί αυτήν την επιλογή και τις ψήφους της. Η ετικέτα και η θέση της ενημερώνονται σύμφωνα με αυτά που στείλατε.
- Μια επιλογή που αποστέλλεται χωρίς `id` προστίθεται, χωρίς ψήφους.
- Μια υπάρχουσα επιλογή που παραλείπετε αφαιρείται, μαζί με τις ψήφους που έχουν κατατεθεί σε αυτήν. Το `totalVotes` μειώνεται κατά το ίδιο ποσό.

Έτσι, για να προσθέσετε μια επιλογή, στείλτε τις τρέχουσες επιλογές με τα ids τους συν την καινούρια χωρίς id. Για να αφαιρέσετε μια επιλογή, στείλτε τη λίστα χωρίς αυτήν. Τα ids των επιλογών βρίσκονται στη δημοσκόπηση που επιστρέφεται από `GET /api/v1/polls/:commentId`.

Αν δεν στείλετε καθόλου ids, αντικαθιστάται κάθε επιλογή και διαγράφονται όλες οι ψήφοι που έχουν ήδη κατατεθεί στη δημοσκόπηση. Εάν η δημοσκόπηση έχει ψήφους, απαιτείται `replaceVotes=true`, και χωρίς αυτό το API απαντά με `replace-votes-required`.

Τα άλλα πεδία επίσης αντικαθίστανται: η παράλειψη των `closesAt`, `privacy` ή `requireVoteToSeeResults` τα επαναφέρει στην προεπιλογή τους. Για να αλλάξετε ένα μόνο πεδίο και να αφήσετε τα υπόλοιπα όπως είναι, χρησιμοποιήστε `PATCH /api/v1/polls/:commentId`.

[inline-code-attrs-start title = 'Παράδειγμα cURL για Poll Put'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PUT \
  --url 'https://fastcomments.com/api/v1/polls/comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"question": "Which release should we cut first?",
	"options": [
		{"id": "existing-option-id", "label": "The bugfix release"},
		{"label": "The feature release"}
	],
	"closesAt": "2026-12-31T00:00:00.000Z"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Αιτήματος Poll Put'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollPutQueryParams {
    tenantId: string
    API_KEY: string
    /** Απαιτείται για να μην διατηρηθούν κανένα από τα υπάρχοντα ids επιλογών όταν η δημοσκόπηση έχει ψήφους, καθώς αυτό τις διαγράφει όλες. **/
    replaceVotes?: boolean
}

interface PollPutOption {
    /** Το id μιας υπάρχουσας επιλογής, για να τη διατηρήσετε και τις ψήφους της. Παραλείψτε το για να προσθέσετε μια νέα επιλογή. **/
    id?: string | null
    label: string
}

interface PollPutBody {
    question: string
    /** Η πλήρης, διατεταγμένη λίστα. Οι υπάρχουσες επιλογές που παραλείπονται αφαιρούνται μαζί με τις ψήφους τους. **/
    options: PollPutOption[]
    /** Πρέπει να είναι στο μέλλον όταν το σχόλιο δεν έχει ακόμη δημοσκόπηση. Παραλείψτε το για μια δημοσκόπηση που παραμένει ανοιχτή. **/
    closesAt?: string | null
    /** 0 ανώνυμα (η προεπιλογή), 1 διαχειριστές και συντονιστές, 2 όλοι. **/
    privacy?: 0 | 1 | 2 | null
    requireVoteToSeeResults?: boolean | null
}
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Απόκρισης Poll Put'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollPutResponse {
    status: 'success' | 'failed'
    /** Συμπεριλαμβάνεται σε περίπτωση αποτυχίας. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'not-found' | 'polls-disabled' | 'poll-invalid' | 'replace-votes-required' | 'poll-privacy-locked' | 'locked'
    /** Συμπεριλαμβάνεται σε περίπτωση αποτυχίας. **/
    reason?: string
    poll: CommentPoll
}
[inline-code-end]

### Άλλες Σημειώσεις

- Ένα `id` που δεν υπάρχει στη δημοσκόπηση, ή το ίδιο `id` που δίνεται δύο φορές, αποτυγχάνει με `poll-invalid`. Ένα σχόλιο χωρίς δημοσκόπηση δεν έχει ακόμη ids επιλογών, έτσι κάθε επιλογή που του αποστέλλεται πρέπει να παραλείπει το `id`.
- Η ιδιωτικότητα της δημοσκόπησης μπορεί να περιοριστεί αλλά όχι να επεκταθεί μόλις έχει ψήφους.
- Αυτό το API ακολουθεί τις ρυθμίσεις του ιστότοπού σας. Εάν οι δημοσκοπήσεις δεν είναι ενεργοποιημένες για τον ιστότοπο ή τη σελίδα, αποτυγχάνει με `polls-disabled`.
- Ένα κλειδωμένο σχόλιο δεν μπορεί να αλλάξει τη δημοσκόπηση του και αποτυγχάνει με `locked`.
- Τα συνδεδεμένα widget ενημερώνονται σε πραγματικό χρόνο, ώστε οι θεατές να βλέπουν τη νέα δημοσκόπηση χωρίς επαναφόρτωση.