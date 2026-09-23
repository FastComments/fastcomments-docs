[api-resource-header-start name = 'Poll'; route = 'DELETE /api/v1/polls/:commentId'; creditsCost = 1; api-resource-header-end]

Αφαιρεί μια ψηφοφορία από το σχόλιό της, μαζί με κάθε ψήφο που έχει κατατεθεί. Το ίδιο το σχόλιο παραμένει αμετάβλητο.

Η διαγραφή του σχολίου αφαιρεί επίσης την ψηφοφορία και τις ψήφους του, επομένως αυτό απαιτείται μόνο όταν θέλετε να διατηρήσετε το σχόλιο.

[inline-code-attrs-start title = 'Παράδειγμα cURL Διαγραφής Ψηφοφορίας'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/polls/comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Αιτήματος Διαγραφής Ψηφοφορίας'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollDeleteQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Απόκρισης Διαγραφής Ψηφοφορίας'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollDeleteResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'poll-not-found'
    /** Included on failure. **/
    reason?: string
}
[inline-code-end]