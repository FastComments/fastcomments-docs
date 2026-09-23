[api-resource-header-start name = 'PollVote'; route = 'POST /api/v1/poll-votes'; creditsCost = 1; api-resource-header-end]

Καταγράφει μια ψήφο σε μια δημοσκόπηση.

Ένας ψηφοφόρος μπορεί να έχει το πολύ μία ψήφο ανά δημοσκόπηση. Η επανάκληση αυτής της κλήσης για τον ίδιο ψηφοφόρο μετακινεί την ψήφο του στη νέα επιλογή αντί να προσθέτει δεύτερη, και η ψήφος για την επιλογή που ήδη επέλεξε δεν κάνει τίποτα.

Η απάντηση περιλαμβάνει τη δημοσκόπηση, ώστε να λαμβάνετε τις ενημερωμένες μετρήσεις χωρίς δεύτερο αίτημα.

[inline-code-attrs-start title = 'Παράδειγμα cURL Δημιουργίας PollVote'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/poll-votes?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"commentId": "comment-id",
	"optionId": "the-option-id",
	"userId": "user-id"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Παράδειγμα cURL Δημιουργίας Ανώνυμου PollVote'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/poll-votes?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"commentId": "comment-id",
	"optionId": "the-option-id",
	"anonUserId": "some-randomly-generated-identifier",
	"ip": "203.0.113.4"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Αιτήματος Δημιουργίας PollVote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVoteCreateQueryParams {
    tenantId: string
    API_KEY: string
}

interface PollVoteCreateBody {
    commentId: string
    optionId: string
    /** One of userId or anonUserId is required. **/
    userId?: string
    anonUserId?: string
    /** The end user's IP, used for the anonymous rate limit. Defaults to the caller's IP. **/
    ip?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Απόκρισης Δημιουργίας PollVote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVoteCreateResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-comment-id' | 'missing-user-id' | 'invalid-user' | 'unauthorized' | 'poll-not-found' | 'poll-invalid-option' | 'poll-closed' | 'poll-login-required' | 'rate-limited'
    /** Included on failure. **/
    reason?: string
    pollVote: PollVote
    /** The poll with its updated counts. **/
    poll: CommentPoll
}
[inline-code-end]

### Ανώνυμες Ψήφοι

Ορίστε `anonUserId` αντί για `userId` για να καταγράψετε μια ψήφο για κάποιον που δεν είναι συνδεδεμένος. Αυτό το αναγνωριστικό δεν χρειάζεται να αντιστοιχεί σε χρήστη πουθενά· απλώς αναγνωρίζει τη συνεδρία, ώστε το ίδιο άτομο να μην μετράται δύο φορές.

Η ανώνυμη ψηφοφορία πρέπει να είναι ενεργοποιημένη για τον ιστότοπό σας. Εάν η ψηφοφορία περιορίζεται σε συνδεδεμένους χρήστες, μια ψήφος μόνο με `anonUserId` αποτυγχάνει με `poll-login-required`.

Οι ανώνυμες ψήφοι περιορίζονται επίσης ανά IP ανά δημοσκόπηση, για να αποτραπεί ένα άτομο να γεμίσει τη δημοσκόπηση καθαρίζοντας τη συνεδρία του. Στείλτε το `ip` του τελικού χρήστη ώστε το όριο να ισχύει για αυτόν αντί για τον διακομιστή σας.

### Άλλες Σημειώσεις

- Ένα `userId` πρέπει να είναι χρήστης που υπάρχει στον ιστότοπό σας. Οι ψήφοι για χρήστη που ανήκει σε άλλο ιστότοπο απορρίπτονται.
- Η ψηφοφορία σε κλειστή δημοσκόπηση αποτυγχάνει με `poll-closed`.
- Αυτό το API ενημερώνει τις μετρήσεις στη δημοσκόπηση και τις στέλνει σε συνδεδεμένα widget σε πραγματικό χρόνο.