[api-resource-header-start name = 'Page Migrations'; route = 'POST /api/v1/pages/migrations'; creditsCost = 1; api-resource-header-end]

Αυτή η διαδρομή μετακινεί σχόλια και σελίδες από ένα `urlId` ή domain σε άλλο, το ίδιο όπως το [Domain Migration tool](/guide-migrations.html#migrating-domains) στον πίνακα ελέγχου. Μπορούν να αποσταλούν έως 100 μεταναστεύσεις ανά αίτημα. Εκτελούνται με τη σειρά που δίνονται, έτσι η μεταφορά από `a` σε `b` ακολουθούμενη από `b` σε `c` λειτουργεί.

Κάθε μετανάστευση έχει ένα `type`:

- `urlId` μετακινεί τη σελίδα της οποίας το `urlId` είναι ακριβώς `from`. Το `post-1` δεν ταιριάζει με το `post-10`.
- `domain` βρίσκει/αντικαθιστά το `from` μέσα σε κάθε `urlId` και `url`, και ενημερώνει το `domain` κάθε σχολίου. Και οι δύο τιμές πρέπει να είναι ένα domain, όπως `old.example.com`.

Τα σχόλια, η σελίδα, οι αντιδράσεις της σελίδας, οι ψήφοι, οι ειδοποιήσεις και οι συνδρομές μετακινούνται όλα. Εάν υπάρχει ήδη μια σελίδα στο `to`, οι δύο σελίδες συγχωνεύονται.

Η μετανάστευση εκτελείται ως παρτίδα εργασίας. Η απάντηση περιέχει ένα `batchJobId`. Κάντε poll στο [`GET /api/v1/batch-jobs/:id`](#batch-job-get-by-id) για να παρακολουθήσετε την πρόοδό του. Μόλις η εργασία ολοκληρωθεί, έχει ένα αποτέλεσμα ανά μετανάστευση, και ένα `undoId` που μπορείτε να περάσετε στο [`POST /api/v1/pages/migrations/:undoId/undo`](#pages-migrations-undo-post) εντός 90 ημερών.

Το αίτημα επικυρώνεται πριν οτιδήποτε τοποθετηθεί στην ουρά. Εάν κάποια μετανάστευση είναι μη έγκυρη, ολόκληρο το αίτημα αποτυγχάνει με κωδικό `422` και δεν εκτελείται τίποτα.

Η τοποθέτηση μιας μετανάστευσης στην ουρά δεν κοστίζει τίποτα. Όταν η εργασία ολοκληρωθεί, κοστίζει `1` μονάδα ανά `1000` σχόλια που μεταφέρθηκαν, με ελάχιστο κόστος `1`.

[inline-code-attrs-start title = 'Παράδειγμα cURL Μετανάστευσης Σελίδων'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/pages/migrations?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"migrations": [
		{"from": "old-post-id", "to": "new-post-id", "type": "urlId"},
		{"from": "old.example.com", "to": "new.example.com", "type": "domain"}
	]
}'
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Αιτήματος Μετανάστευσης Σελίδων'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageMigrationsQueryParams {
    tenantId: string
    API_KEY: string
}

interface PageMigrationsBody {
    /** Run in the order given. At most 100. **/
    migrations: PageMigration[]
}

interface PageMigration {
    /** At most 1000 characters. Leading and trailing whitespace is removed. **/
    from: string
    /** Must be different from "from". At most 1000 characters. **/
    to: string
    /** "urlId" only matches the exact urlId. "domain" find/replaces the value inside every urlId and url. **/
    type: 'urlId' | 'domain'
}
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Απόκρισης Μετανάστευσης Σελίδων'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageMigrationsResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-migrations' | 'too-many-migrations' | 'invalid-migration' | 'internal'
    /** Included on failure. For invalid-migration, this names the index of the first invalid migration. **/
    reason?: string
    /** Pass to GET /api/v1/batch-jobs/:id for progress and the result. **/
    batchJobId?: string
}
[inline-code-end]

#### Σημειώσεις

- Οι εργασίες για έναν ενοικιαστή εκτελούνται μία τη φορά, πρώτα οι παλαιότερες. Μια δεύτερη μετανάστευση περιμένει να ολοκληρωθεί η πρώτη.
- `PATCH /api/v1/pages/:id` με ένα νέο `urlId` αλλάζει μόνο τη σελίδα, όχι τα σχόλιά της. Χρησιμοποιήστε αυτή τη διαδρομή για να μετακινήσετε τα σχόλια μιας σελίδας.

---