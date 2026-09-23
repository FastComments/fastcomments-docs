[api-resource-header-start name = 'Poll'; route = 'GET /api/v1/polls/:commentId'; creditsCost = 1; api-resource-header-end]

Διαβάζει την ψηφοφορία που είναι συνδεδεμένη με ένα σχόλιο, με τις τρέχουσες μετρήσεις ψήφων.

Οι ψηφοφορίες επιστρέφονται επίσης στο ίδιο το σχόλιο από τα API σχολίων, οπότε χρησιμοποιήστε αυτό όταν θέλετε μόνο τα αποτελέσματα και όχι ολόκληρο το σχόλιο.

[inline-code-attrs-start title = 'Παράδειγμα cURL για Λήψη Ψηφοφορίας'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/polls/comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Αιτήματος Λήψης Ψηφοφορίας'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollGetQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Απόκρισης Λήψης Ψηφοφορίας'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollGetResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'poll-not-found'
    /** Included on failure. **/
    reason?: string
    poll: CommentPoll
}
[inline-code-end]

Ένα σχόλιο που δεν έχει ψηφοφορία, ένα σχόλιο που έχει διαγραφεί, και ένα αναγνωριστικό σχολίου που δεν υπάρχει, όλα ανταποκρίνονται με τον ίδιο τρόπο, με `poll-not-found`.

---