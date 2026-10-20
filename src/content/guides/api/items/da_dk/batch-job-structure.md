A `BatchJob` er en langvarig opgave, som f.eks. en import, en eksport eller en [page migration](#pages-migrations-post). Job er i kø, hvorefter de hentes og køres i baggrunden.

`processedCount` og `totalCount` viser fremdrift. Deres enhed afhænger af `type`:

- `migrate-pages`: migrationer færdiggjort, ud af de anmodede migrationer.
- `undo-page-migration`: poster gendannet. Der er ingen `totalCount`.
- `import`, `export` og andre job: rækker behandlet.

Strukturen for `BatchJob`-objektet er som følger:

[inline-code-attrs-start title = 'BatchJob Struktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface BatchJob {
    id: string
    /** For eksempel migrate-pages, undo-page-migration, migrate-domains, import, export, users-export eller copy-comments. **/
    type: string
    status: 'requested' | 'running' | 'success' | 'failed'
    createdAt: string
    completedAt?: string
    processedCount: number
    /** Samlet arbejde, når det er kendt på forhånd. **/
    totalCount?: number
    /** migrate-pages: én post pr. færdig migration, i anmodningsrækkefølge. **/
    pageMigrationResults?: Array<PageMigrationSuccess | PageMigrationFailure>
    /**
     * migrate-pages: sættes, når jobbet er afsluttet (success eller failed). Send til POST /api/v1/pages/migrations/:undoId/undo inden for 90 dage.
     * undo-page-migration: migrationen der bliver fortrudt.
     **/
    undoId?: string
    /** undo-page-migration: sættes, når jobbet er afsluttet. **/
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
    /** Poster sat tilbage til deres værdier fra før migrationen. **/
    restored: number
    /** Poster efterladt uændret fordi de ændrede sig efter migrationen, eller ikke længere eksisterer. **/
    skipped: number
    /** Poster den migration, der blev slettet under sammensmeltning af sider. Disse kan ikke genskabes. **/
    notRestorable: number
}
[inline-code-end]

#### Gentagelser

Hvis serveren, der kører et page migration-job, genstartes, sættes jobbet tilbage i køen og fortsætter efter den sidste færdige migration. Et job forsøges op til 3 gange, hvorefter dets `status` er `failed`.

---