[api-resource-header-start name = 'AuditLog'; route = 'GET /api/v1/audit-logs'; creditsCost = 10; api-resource-header-end]

Αυτό το API χρησιμοποιεί σελιδοποίηση, που παρέχεται από τις παραμέτρους `skip`, `limit`, `before` και `after`. Τα AuditLogs επιστρέφονται σε σελίδες των `5000` εξ ορισμού, έως ένα μέγιστο `limit` των `10000`, ταξινομημένα κατά `when` και `id`. Οι σελίδες είναι μεγάλες επειδή αυτό το endpoint συνήθως χρησιμοποιείται για την εξαγωγή ιστορικού αντί για διαδραστική περιήγηση.

Κάθε `100` καταγεγραμμένα logs έχουν κόστος πίστωσης `1`.

Από προεπιλογή, θα λάβετε μια λίστα με **τα πιο πρόσφατα στοιχεία πρώτα**. Με αυτόν τον τρόπο, μπορείτε να κάνετε polling ξεκινώντας από `skip=0`, σελιδοποιώντας μέχρι να βρείτε την τελευταία εγγραφή που έχετε καταναλώσει.

Εναλλακτικά, μπορείτε να ταξινομήσετε από το παλαιότερο προς το νεότερο και να σελιδοποιήσετε μέχρι να μην υπάρχουν άλλες εγγραφές.

Η ταξινόμηση μπορεί να γίνει ορίζοντας το `order` είτε σε `ASC` είτε σε `DESC`. Η προεπιλογή είναι `DESC`.

Η ερώτηση κατά ημερομηνία είναι δυνατή μέσω των `before` και `after` ως χρονικές σφραγίδες με χιλιοστά του δευτερολέπτου. Τα `before` και `after` ΔΕΝ είναι περιληπτικά, και το καθένα μπορεί να χρησιμοποιηθεί μόνο του.

## Finding what happened to a person

Κάθε γεγονός καταγράφει ποιος το εκτέλεσε (`username`, `userId`, `ip`) και, ξεχωριστά, σε τι εκτελέστηκε. Το `targetLabel` είναι μια ετικέτα αναγνώσιμη από άνθρωπο για το αντικείμενο, για παράδειγμα `jsmith (jsmith@example.com)`, και το `targetId` είναι το αναγνωριστικό του. Χρησιμοποιήστε το `target` για αναζήτηση υποσυμβολοσειράς χωρίς διάκριση πεζών-κεφαλαίων στην ετικέτα όταν γνωρίζετε το όνομα ή το email ενός ατόμου αλλά όχι το αναγνωριστικό του.

Οι διαγραφές καταγράφουν την ετικέτα τη στιγμή του γεγονότος, ώστε ένας διαγραμμένος χρήστης ή συντονιστής να μπορεί ακόμη να ταυτοποιηθεί μετά την εξαφάνιση της υποκείμενης εγγραφής.

## Managed tenants

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
    /** Μέγιστο 10000. Προεπιλογή 5000. **/
    limit?: number
    skip?: number
    before?: number
    after?: number
    /** Μόνο γεγονότα που εκτελέστηκαν από αυτό το όνομα χρήστη. **/
    username?: string
    /** Μόνο γεγονότα από αυτή τη διεύθυνση IP. **/
    ip?: string
    /** Μόνο γεγονότα αυτού του τύπου. **/
    crudType?: 'c' | 'r' | 'u' | 'd' | 'login'
    /** Μόνο γεγονότα για αυτόν τον πόρο, π.χ. Χρήστη ή Συντονιστή. **/
    resourceName?: string
    /** Μόνο γεγονότα του οποίου το επηρεαζόμενο αντικείμενο έχει αυτό το αναγνωριστικό. **/
    targetId?: string
    /** Αναζήτηση υποσυμβολοσειράς χωρίς διάκριση πεζών-κεφαλαίων στην ετικέτα του επηρεαζόμενου αντικειμένου. **/
    target?: string
    /** Επίσης επιστρέψτε γεγονότα από ενοικιαστές που διαχειρίζεται αυτός ο ενοικιαστής. **/
    includeManagedTenants?: boolean
}
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Απόκρισης AuditLog'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface AuditLogsResponse {
    status: 'success' | 'failed'
    /** Περιλαμβάνεται σε περίπτωση αποτυχίας. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'invalid-limit' | 'invalid-skip'
    /** Περιλαμβάνεται σε περίπτωση αποτυχίας. **/
    reason?: string
    /** Τα αρχεία καταγραφής! **/
    auditLogs: AuditLog[]
}
[inline-code-end]