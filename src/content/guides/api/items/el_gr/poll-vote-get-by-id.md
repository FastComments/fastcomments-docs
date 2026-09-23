[api-resource-header-start name = 'PollVote'; route = 'GET /api/v1/poll-votes/:id'; creditsCost = 1; api-resource-header-end]

Διαβάζει μια μοναδική ψήφο δημοσκόπησης με βάση το αναγνωριστικό της.

Μια ψήφος σε ανώνυμη δημοσκόπηση δεν μπορεί να διαβαστεί, και το αίτημα αποτυγχάνει με `poll-anonymous`. Δείτε τη δομή `PollVote` για το πώς εφαρμόζεται η ρύθμιση `privacy` της δημοσκόπησης.

[inline-code-attrs-start title = 'Παράδειγμα cURL για PollVote Get'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/poll-votes/my-id?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Αιτήματος Get για PollVote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVoteGetQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Απόκρισης Get για PollVote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVoteGetResponse {
    status: 'success' | 'failed'
    /** Συμπεριλαμβάνεται σε περίπτωση αποτυχίας. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unauthorized' | 'not-found' | 'poll-not-found' | 'poll-anonymous'
    /** Συμπεριλαμβάνεται σε περίπτωση αποτυχίας. **/
    reason?: string
    pollVote: PollVote
}
[inline-code-end]