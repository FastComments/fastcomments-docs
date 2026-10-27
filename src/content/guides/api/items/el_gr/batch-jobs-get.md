[api-resource-header-start name = 'BatchJob'; route = 'GET /api/v1/batch-jobs'; creditsCost = 1; api-resource-header-end]

Αυτή η διαδρομή επιστρέφει τις παρτίδες εργασιών του ενοικιαστή για κάθε τύπο, από τις πιο πρόσφατες πρώτες, 100 ανά σελίδα. Χρησιμοποιήστε `type` και `status` για να περιορίσετε τη λίστα και `skip` για σελιδοποίηση.

[inline-code-attrs-start title = 'Παράδειγμα cURL Λίστας BatchJob'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/batch-jobs?tenantId=demo&API_KEY=DEMO_API_SECRET&type=migrate-pages&status=running'
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Αιτήματος Λίστας BatchJob'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface BatchJobsQueryParams {
    tenantId: string
    API_KEY: string
    /** Μόνο εργασίες αυτού του τύπου, για παράδειγμα migrate-pages. **/
    type?: string
    status?: 'requested' | 'running' | 'success' | 'failed'
    /** Πόσες εργασίες να παραληφθούν, για σελιδοποίηση. Ένας μη-αρνητικός ακέραιος. **/
    skip?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Απόκρισης Λίστας BatchJob'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface BatchJobsResponse {
    status: 'success' | 'failed'
    /** Συμπεριλαμβάνεται σε περίπτωση αποτυχίας. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'invalid-skip' | 'internal'
    /** Συμπεριλαμβάνεται σε περίπτωση αποτυχίας. **/
    reason?: string
    /** Από τις πιο πρόσφατες πρώτες, το πολύ 100. **/
    batchJobs?: BatchJob[]
}
[inline-code-end]