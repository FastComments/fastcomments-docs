A `BatchJob` je zadatak dugog trajanja, poput uvoza, izvoza ili [page migration](#pages-migrations-post). Poslovi se stavljaju u red, a zatim se preuzimaju i izvršavaju u pozadini.

`processedCount` i `totalCount` prikazuju napredak. Njihova jedinica ovisi o `type`:

- `migrate-pages`: migracije završene, od traženih migracija.
- `undo-page-migration`: zapisi obnovljeni. Nema `totalCount`.
- `import`, `export` i ostali poslovi: obrađeni redci.

Struktura objekta `BatchJob` je sljedeća:

[inline-code-attrs-start title = 'Struktura BatchJob'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface BatchJob {
    id: string
    /** Na primjer: migrate-pages, undo-page-migration, migrate-domains, import, export, users-export ili copy-comments. **/
    type: string
    status: 'requested' | 'running' | 'success' | 'failed'
    createdAt: string
    completedAt?: string
    processedCount: number
    /** Ukupni rad, kada je poznat unaprijed. **/
    totalCount?: number
    /** migrate-pages: jedan unos po završetoj migraciji, redoslijedom zahtjeva. **/
    pageMigrationResults?: Array<PageMigrationSuccess | PageMigrationFailure>
    /**
     * migrate-pages: postavi kada je posao završen (uspjeh ili neuspjeh). Proslijedi na POST /api/v1/pages/migrations/:undoId/undo unutar 90 dana.
     * undo-page-migration: migracija koja se poništava.
     **/
    undoId?: string
    /** undo-page-migration: postavi kada je posao završen. **/
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
    /** Zapisi vraćeni na njihove vrijednosti prije migracije. **/
    restored: number
    /** Zapisi ostavljeni nepromijenjeni jer su se promijenili nakon migracije, ili više ne postoje. **/
    skipped: number
    /** Zapisi migracije izbrisani prilikom spajanja stranica. Ovi se ne mogu vratiti. **/
    notRestorable: number
}
[inline-code-end]

#### Ponovni pokušaji

Ako se poslužitelj koji izvršava zadatak migracije stranice ponovno pokrene, zadatak se vraća u red i nastavlja nakon posljednje završene migracije. Zadaci se pokušavaju najviše 3 puta, nakon čega je njihov `status` `failed`.