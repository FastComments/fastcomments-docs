[api-resource-header-start name = 'Vote'; route = 'DELETE /api/v1/votes/:id'; creditsCost = 1; api-resource-header-end]

Αυτή η διαδρομή παρέχει τη δυνατότητα διαγραφής μιας μόνου `Vote`.

[inline-code-attrs-start title = 'Διαγραφή Vote cURL Παράδειγμα'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/votes/some-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Αιτήματος Διαγραφής Vote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VoteDeleteQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Απάντησης Διαγραφής Vote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VoteDeleteResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id'
    /** Included on failure. **/
    reason?: string
}
[inline-code-end]

Σημειώσεις:

- Αυτό το API υπακούει τις ρυθμίσεις σε επίπεδο ενοικιαστή. Για παράδειγμα, αν απενεργοποιήσετε την ψηφοφορία για μια δεδομένη σελίδα, και προσπαθήσετε να δημιουργήσετε μια ψήφο μέσω του API, θα αποτύχει με κωδικό σφάλματος `voting-disabled`.
- Αυτό το API είναι ζωντανό από προεπιλογή.
- Αυτό το API θα ενημερώσει τα `votes` του αντίστοιχου `Comment`.
