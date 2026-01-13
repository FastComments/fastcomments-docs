[api-resource-header-start name = 'AuditLog'; route = 'GET /api/v1/audit-logs'; creditsCost = 10; api-resource-header-end]

Αυτό το API χρησιμοποιεί σελιδοποίηση, που παρέχεται από τις παραμέτρους `skip`, `before` και `after`. Τα AuditLogs επιστρέφονται σε σελίδες των `1000`, ταξινομημένα κατά `when` και `id`.

Η ανάκτηση κάθε `1000` logs έχει κόστος credit `10`.

Από προεπιλογή, θα λάβετε μια λίστα με **τα νεότερα στοιχεία πρώτα**. Με αυτόν τον τρόπο, μπορείτε να κάνετε polling ξεκινώντας με `skip=0`, σελιδοποιώντας μέχρι να βρείτε την τελευταία εγγραφή που έχετε καταναλώσει.

Εναλλακτικά, μπορείτε να ταξινομήσετε από τα παλαιότερα στα νεότερα, και να σελιδοποιήσετε μέχρι να μην υπάρχουν άλλες εγγραφές.

Η ταξινόμηση μπορεί να γίνει ρυθμίζοντας το `order` είτε σε `ASC` είτε σε `DESC`. Η προεπιλογή είναι `ASC`.

Η αναζήτηση κατά ημερομηνία είναι δυνατή μέσω `before` και `after` ως timestamps με milliseconds. Τα `before` και `after` ΔΕΝ είναι inclusive.

[inline-code-attrs-start title = 'AuditLog cURL Παράδειγμα'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/audit-logs?tenantId=demo&API_KEY=DEMO_API_SECRET&skip=0&order=ASC&before=123&after=456'
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Αιτήματος AuditLog'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentsRequestQueryParams {
    tenantId: string
    API_KEY: string
    order?: 'ASC' | 'DESC'
    skip?: number
    before?: number
    after?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Απάντησης AuditLog'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentsResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Included on failure. **/
    reason?: string
    /** The logs! **/
    auditLogs: AuditLog[]
}
[inline-code-end]
