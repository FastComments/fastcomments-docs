[api-resource-header-start name = 'PollVote'; route = 'DELETE /api/v1/poll-votes/:id'; creditsCost = 1; api-resource-header-end]

Ανακαλεί μια ψήφο. Η επιλογή στην οποία δόθηκε η ψήφος επαναφέρει τον μετρητή της, και ο ψηφοφόρος μπορεί να ψηφίσει ξανά.

[inline-code-attrs-start title = 'Παράδειγμα cURL Διαγραφής PollVote'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/poll-votes/my-id?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Αιτήματος Διαγραφής PollVote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVoteDeleteQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Απόκρισης Διαγραφής PollVote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVoteDeleteResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'not-found' | 'poll-not-found'
    /** Included on failure. **/
    reason?: string
    /** The poll with its updated counts. **/
    poll: CommentPoll
}
[inline-code-end]

### Άλλες Σημειώσεις

- Η διαγραφή της ίδιας ψήφου δύο φορές επιστρέφει `not-found` τη δεύτερη φορά, και οι μετρήσεις παραμένουν αμετάβλητες.  
- Αν η δημοσκόπηση αντικαταστάθηκε από τη στιγμή που ψηφίστηκε η ψήφος, η ψήφος αφαιρείται αλλά δεν αλλάζει καμία μέτρηση, επειδή η αντικατάσταση ξεκίνησε από το μηδέν.