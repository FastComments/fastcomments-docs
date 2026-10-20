[api-resource-header-start name = 'BatchJob'; route = 'GET /api/v1/batch-jobs/:id'; creditsCost = 1; api-resource-header-end]

Αυτή η διαδρομή επιστρέφει την κατάσταση, την πρόοδο και, όταν ολοκληρωθεί, το αποτέλεσμα μιας εργασίας batch. Χρησιμοποιήστε την για να παρακολουθήσετε μια εργασία που ξεκίνησε από [`POST /api/v1/pages/migrations`](#pages-migrations-post) ή [`POST /api/v1/pages/migrations/:undoId/undo`](#pages-migrations-undo-post).

Μια εργασία θεωρείται ολοκληρωμένη όταν η `status` της είναι `success` ή `failed`. Κάθε κλήση κοστίζει `1` μονάδα, γι' αυτό κάντε polling κάθε λίγα δευτερόλεπτα αντί συνεχώς.

[inline-code-attrs-start title = 'Παράδειγμα cURL για BatchJob με Id'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/batch-jobs/my-batch-job-id?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Αιτήματος BatchJob με Id'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface BatchJobQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Απόκρισης BatchJob με Id'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface BatchJobResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'not-found' | 'internal'
    /** Included on failure. **/
    reason?: string
    batchJob?: BatchJob
}
[inline-code-end]

[inline-code-attrs-start title = 'Παράδειγμα Ολοκληρωμένης Εργασίας Μεταφοράς Σελίδας'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
    "status": "success",
    "batchJob": {
        "id": "my-batch-job-id",
        "type": "migrate-pages",
        "status": "success",
        "createdAt": "2026-10-19T16:00:00.000Z",
        "completedAt": "2026-10-19T16:00:12.000Z",
        "processedCount": 2,
        "totalCount": 2,
        "pageMigrationResults": [
            {"from": "old-post-id", "to": "new-post-id", "type": "urlId", "status": "success", "commentsMigrated": 42},
            {"from": "old.example.com", "to":"new.example.com","type":"domain","status":"success","commentsMigrated":1250}
        ],
        "undoId": "my-undo-id"
    }
}
[inline-code-end]