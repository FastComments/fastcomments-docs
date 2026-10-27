[api-resource-header-start name = 'Page Migrations'; route = 'POST /api/v1/pages/migrations/:undoId/undo'; creditsCost = 1; api-resource-header-end]

Αυτή η διαδρομή αντιστρέφει κάθε μετανάστευση από μία εργασία [`POST /api/v1/pages/migrations`](#pages-migrations-post). Λάβετε το `undoId` από την ολοκληρωμένη εργασία μετανάστευσης με το [`GET /api/v1/batch-jobs/:id`](#batch-job-get-by-id). Ένα `undoId` μπορεί να χρησιμοποιηθεί για 90 ημέρες και μόνο μία φορά.

Η ενέργεια undo εκτελείται ως παρτίδα εργασίας, όπως και η μετανάστευση, και η απάντηση περιέχει το `batchJobId`. Μόλις ολοκληρωθεί η εργασία, το `pageMigrationUndoResult` της αναφέρει:

- `restored`: εγγραφές που επανήλθαν στις τιμές τους πριν τη μετανάστευση.
- `skipped`: εγγραφές που άφησαν αμετάβλητες επειδή άλλαξαν μετά τη μετανάστευση, ή δεν υπάρχουν πλέον. Μια μεταγενέστερη επεξεργασία δεν αντικαθίσταται ποτέ.
- `notRestorable`: εγγραφές που η μετανάστευση διέγραψε κατά τη συγχώνευση δύο σελίδων ή κατά την αφαίρεση διπλών αντιδράσεων σελίδας. Αυτές δεν μπορούν να επαναφερθούν.

Οι μετανάστευσεις αντιστρέφονται από τις πιο πρόσφατες πρώτα, έτσι μια αλυσίδα όπως `a` προς `b` ακολουθούμενη από `b` προς `c` ξετυλίγεται πίσω στο `a`. Μια εργασία μετανάστευσης που απέτυχε ενδιάμεσα μπορεί ακόμη να αναιρεθεί, κάτι που αντιστρέφει τις αλλαγές που έκανε πριν αποτύχει.

Η τοποθέτηση ενός undo στην ουρά δεν κοστίζει τίποτα. Όταν η εργασία ολοκληρωθεί, κοστίζει `1` μονάδα ανά `1000` εγγραφές που επανήλθαν, με ελάχιστο κόστος `1`.

[inline-code-attrs-start title = 'Παράδειγμα cURL Αντιστροφής Μεταναστεύσεων Σελίδας'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/pages/migrations/my-undo-id/undo?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Αιτήματος Αντιστροφής Μεταναστεύσεων Σελίδας'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface UndoPageMigrationsQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Δομή Απόκρισης Αντιστροφής Μεταναστεύσεων Σελίδας'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface UndoPageMigrationsResponse {
    status: 'success' | 'failed'
    /**
     * Included on failure.
     * undo-not-found (404): no migration has this undoId, or it is older than 90 days.
     * already-undone (409) and undo-in-progress (409): the migration has been undone, or an undo of it is queued or running.
     **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'undo-not-found' | 'already-undone' | 'undo-in-progress' | 'internal'
    /** Included on failure. **/
    reason?: string
    /** Pass to GET /api/v1/batch-jobs/:id for progress and the result. **/
    batchJobId?: string
}
[inline-code-end]