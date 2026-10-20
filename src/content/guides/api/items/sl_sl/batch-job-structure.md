A `BatchJob` je dolgotrajna naloga, kot je uvoz, izvoz ali [migracija strani](#pages-migrations-post). Naloge se postavijo v čakalno vrsto, nato se jih prevzame in izvede v ozadju.

`processedCount` in `totalCount` prikazujeta napredek. Njuna enota je odvisna od `type`:

- `migrate-pages`: končane migracije, od zahtevanih migracij.
- `undo-page-migration`: obnovljeni zapisi. `totalCount` ne obstaja.
- `import`, `export` in druge naloge: obdelane vrstice.

Struktura objekta `BatchJob` je naslednja:

[inline-code-attrs-start title = 'Struktura BatchJob'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface BatchJob {
    id: string
    /** Na primer migrate-pages, undo-page-migration, migrate-domains, import, export, users-export ali copy-comments. **/
    type: string
    status: 'requested' | 'running' | 'success' | 'failed'
    createdAt: string
    completedAt?: string
    processedCount: number
    /** Skupno delo, ko je znano vnaprej. **/
    totalCount?: number
    /** migrate-pages: en vnos na končano migracijo, v vrstnem redu zahteve. **/
    pageMigrationResults?: Array<PageMigrationSuccess | PageMigrationFailure>
    /**
     * migrate-pages: nastavi se, ko je naloga končana (uspešna ali neuspešna). Posredujte v POST /api/v1/pages/migrations/:undoId/undo v roku 90 dni.
     * undo-page-migration: migracija, ki se razveljavi.
     **/
    undoId?: string
    /** undo-page-migration: nastavi se, ko je naloga končana. **/
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
    // (no comment to translate)
    reason: string     // (no comment to translate)
}

interface PageMigrationUndoResult {
    /** Zapisi, ki so vrnjeni na svoje vrednosti pred migracijo. **/
    restored: number
    /** Zapisi, ki so ostali nespremenjeni, ker so se po migraciji spremenili ali več ne obstajajo. **/
    skipped: number
    /** Zapisi, ki so bili izbrisani med združevanjem strani. Ti jih ni mogoče povrniti. **/
    notRestorable: number
}
[inline-code-end]

#### Poskusi

Če se strežnik, ki izvaja nalogo migracije strani, ponovno zažene, se naloga vrne v čakalno vrsto in se nadaljuje po zadnji končani migraciji. Naloga se poskusi največ 3‑krat, po čemer je njen `status` `failed`.