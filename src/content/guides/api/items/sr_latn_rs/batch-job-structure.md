A `BatchJob` je zadatak koji traje dugo, poput uvoza, izvoza ili [migracije stranice](#pages-migrations-post). Poslovi se stavljaju u red, a zatim se preuzimaju i izvršavaju u pozadini.

`processedCount` i `totalCount` prikazuju napredak. Njihova jedinica zavisi od `type`:

-a `migrate-pages`: migracije završene, od zahtevanih migracija.  
- `undo-page-migration`: zapisi vraćeni. Nema `totalCount`.  
- `import`, `export` i drugi poslovi: obrađeni redovi.

Struktura objekta `BatchJob` je sledeća:

[inline-code-attrs-start title = 'Struktura BatchJob'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface BatchJob {
    id: string
    /** Na primer migrate-pages, undo-page-migration, migrate-domains, import, export, users-export ili copy-comments. **/
    type: string
    status: 'requested' | 'running' | 'success' | 'failed'
    createdAt: string
    completedAt?: string
    processedCount: number
    /** Ukupni rad, kada je poznat unapred. **/
    totalCount?: number
    /** migrate-pages: jedan unos po završnoj migraciji, po redosledu zahteva. **/
    pageMigrationResults?: Array<PageMigrationSuccess | PageMigrationFailure>
    /**
     * migrate-pages: postavljeno kada posao završi (uspeh ili neuspeh). Prosledite na POST /api/v1/pages/migrations/:undoId/undo u roku od 90 dana.
     * undo-page-migration: migracija koja se poništava.
     **/
    undoId?: string
    /** undo-page-migration: postavljeno kada posao završi. **/
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
    /** Zapisi vraćeni na svoje vrednosti pre migracije. **/
    restored: number
    /** Zapisi ostavljeni nepromenjeni jer su se promenili nakon migracije, ili više ne postoje. **/
    skipped: number
    /** Zapisi migracije obrisani prilikom spajanja stranica. Ovi se ne mogu vratiti. **/
    notRestorable: number
}
[inline-code-end]

#### Pokušaji ponovnog izvršavanja

Ako se server koji izvršava posao migracije stranice restartuje, posao se vraća u red i nastavlja nakon poslednje završene migracije. Posao se pokušava najviše 3 puta, nakon čega njegov `status` postaje `failed`.

---