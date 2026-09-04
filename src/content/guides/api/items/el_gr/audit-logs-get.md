[api-resource-header-start name = 'AuditLog'; route = 'GET /api/v1/audit-logs'; creditsCost = 10; api-resource-header-end]

Αυτό το API χρησιμοποιεί σελιδοποίηση, που παρέχεται από τις παραμέτρους `skip`, `limit`, `before` και `after`. Τα AuditLogs επιστρέφονται σε σελίδες των `100` εξ ορισμού, έως μέγιστο `limit` των `200`, ταξινομημένα κατά `when` και `id`.

Κάθε `100` καταγραφές που επιστρέχονται έχουν κόστος πίστωσης `1`.

Από προεπιλογή, θα λάβετε μια λίστα με **τα πιο πρόσφατα στοιχεία πρώτα**. Με αυτόν τον τρόπο, μπορείτε να κάνετε polling ξεκινώντας από `skip=0`, σελιδοποιώντας μέχρι να βρείτε την τελευταία εγγραφή που έχετε καταναλώσει.

Μπορείτε επίσης να ταξινομήσετε από το παλαιότερο προς το νεότερο και να σελιδοποιήσετε μέχρι να μην υπάρχουν άλλες εγγραφές.

Η ταξινόμηση μπορεί να γίνει ορίζοντας το `order` είτε σε `ASC` είτε σε `DESC`. Η προεπιλογή είναι `DESC`.

Η ερώτηση κατά ημερομηνία είναι δυνατή μέσω των `before` και `after` ως χρονικές σφραγίδες με χιλιοστά του δευτερολέπτου. Τα `before` και `after` ΔΕΝ είναι περιληπτικά, και μπορεί να χρησιμοποιηθεί το καθένα ξεχωριστά.

## Εύρέυνα τι συνέβη σε ένα άτομο

Κάθε γεγονός καταγράφει ποιος το πραγματοποίησε (`username`, `userId`, `ip`) και, ξεχωριστά, σε τι πραγματοποιήθηκε. Το `targetLabel` είναι μια ετικέτα φιλική προς τον άνθρωπο για το αντικείμενο, για παράδειγμα `jsmith (jsmith@example.com)`, και το `targetId` είναι το αναγνωριστικό του. Χρησιμοποιήστε το `target` για αναζήτηση υποσυμβολοσειράς χωρίς διάκριση πεζών‑κεφαλαίων στην ετικέτα όταν γνωρίζετε το όνομα ή το email ενός ατόμου αλλά όχι το αναγνωριστικό του.

Οι διαγραφές καταγράφουν την ετικέτα τη στιγμή του γεγονότος, ώστε ένας διαγραμμένος χρήστης ή συντονιστής να μπορεί ακόμη να ταυτοποιηθεί μετά την κατάργηση της υποκείμενης εγγραφής.

## Διαχειριζόμενοι ενοικιαστές

Αν ο ενοικιαστής σας διαχειρίζεται άλλους ενοικιαστές, ορίστε `includeManagedTenants=true` για να επιστρέψετε γεγονότα από τον ενοικιαστή σας και κάθε ενοικιαστή που διαχειρίζεται σε μία απάντηση. Το `tenantId` κάθε επιστρεφόμενου log σας λέει από ποιον ενοικιαστή προέρχεται.

[inline-code-attrs-start title = 'Παράδειγμα cURL AuditLog'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/audit-logs?tenantId=demo&API_KEY=DEMO_API_SECRET&skip=0&order=ASC&before=123&after=456'
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Αιτήματος AuditLog'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface AuditLogsRequestQueryParams {
    tenantId: string
    API_KEY: string
    order?: 'ASC' | 'DESC'
    limit?: number
    skip?: number
    before?: number
    after?: number
    /** Only events performed by this username. **/
    username?: string
    /** Only events from this IP address. **/
    ip?: string
    /** Only events of this type. **/
    crudType?: 'c' | 'r' | 'u' | 'd' | 'login'
    /** Only events for this resource, e.g. User or Moderator. **/
    resourceName?: string
    /** Only events whose affected object has this id. **/
    targetId?: string
    /** Case-insensitive substring match on the affected object's label. **/
    target?: string
    /** Also return events from tenants this tenant manages. **/
    includeManagedTenants?: boolean
}
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Απόκρισης AuditLog'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface AuditLogsResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'invalid-limit' | 'invalid-skip'
    /** Included on failure. **/
    reason?: string
    /** The logs! **/
    auditLogs: AuditLog[]
}
[inline-code-end]