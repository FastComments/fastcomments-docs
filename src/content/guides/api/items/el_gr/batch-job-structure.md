A `BatchJob` είναι μια μακράς διάρκειας εργασία, όπως μια εισαγωγή, μια εξαγωγή ή μια [μεταφορά σελίδας](#pages-migrations-post). Οι εργασίες τοποθετούνται σε ουρά, στη συνέχεια αναλαμβάνονται και εκτελούνται στο παρασκήνιο.

`processedCount` και `totalCount` δείχνουν την πρόοδο. Η μονάδα τους εξαρτάται από το `type`:

- `migrate-pages`: ολοκληρωμένες μεταφορές, από τις αιτηθείσες μεταφορές.
- `undo-page-migration`: εγγραφές που επαναφέρθηκαν. Δεν υπάρχει `totalCount`.
- `import`, `export` και άλλες εργασίες: επεξεργασμένες γραμμές.

Η δομή για το αντικείμενο `BatchJob` είναι η εξής:

[inline-code-attrs-start title = 'Δομή BatchJob'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface BatchJob {
    id: string
    /** Για παράδειγμα migrate-pages, undo-page-migration, migrate-domains, import, export, users-export ή copy-comments. **/
    type: string
    status: 'requested' | 'running' | 'success' | 'failed'
    createdAt: string
    completedAt?: string
    processedCount: number
    /** Συνολική εργασία, όταν είναι γνωστή εκ των προτέρων. **/
    totalCount?: number
    /** migrate-pages: μία καταχώρηση ανά ολοκληρωμένη μεταφορά, με σειρά αιτήματος. **/
    pageMigrationResults?: Array<PageMigrationSuccess | PageMigrationFailure>
    /**
     * migrate-pages: ορίζεται μόλις η εργασία ολοκληρωθεί (επιτυχία ή αποτυχία). Περνάτε στο POST /api/v1/pages/migrations/:undoId/undo εντός 90 ημερών.
     * undo-page-migration: η μεταφορά που αναιρείται.
     **/
    undoId?: string
    /** undo-page-migration: ορίζεται μόλις η εργασία ολοκληρωθεί. **/
    pageMigrationUndoResult?: PageMigrationUndoResult
}

interface PageMigrationSuccess {
    from: string
    to: string
    type: 'urlId' | 'domain'
    status: 'success'
    commentsMigrated: number
}

interface PageMigrationFailure {
    from: string
    to: string
    type: 'urlId' | 'domain'
    status: 'failed'
    code: string
    reason: string
}

interface PageMigrationUndoResult {
    /** Εγγραφές που επαναφέρθηκαν στις τιμές τους πριν τη μεταφορά. **/
    restored: number
    /** Εγγραφές που άφησαν αμετάβλητες επειδή άλλαξαν μετά τη μεταφορά, ή δεν υπάρχουν πλέον. **/
    skipped: number
    /** Εγγραφές της μεταφοράς που διαγράφηκαν κατά τη συγχώνευση σελίδων. Αυτές δεν μπορούν να επαναφερθούν. **/
    notRestorable: number
}
[inline-code-end]

#### Προσπάθειες

Εάν ο διακομιστής που εκτελεί μια εργασία μεταφοράς σελίδας επανεκκινήσει, η εργασία τοποθετείται ξανά στην ουρά και συνεχίζει μετά την τελευταία ολοκληρωμένη μεταφορά. Μια εργασία δοκιμάζεται έως 3 φορές, μετά από τις οποίες το `status` της είναι `failed`.