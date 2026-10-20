A `BatchJob` to długotrwałe zadanie, takie jak import, eksport lub [migracja stron](#pages-migrations-post). Zadania są kolejkowane, a następnie pobierane i uruchamiane w tle.

`processedCount` i `totalCount` pokazują postęp. Ich jednostka zależy od `type`:

- `migrate-pages`: zakończone migracje, w stosunku do żądanych migracji.
- `undo-page-migration`: przywrócone rekordy. Nie ma `totalCount`.
- `import`, `export` i inne zadania: przetworzone wiersze.

Struktura obiektu `BatchJob` jest następująca:

[inline-code-attrs-start title = 'Struktura BatchJob'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface BatchJob {
    id: string
    /** For example migrate-pages, undo-page-migration, migrate-domains, import, export, users-export or copy-comments. **/
    type: string
    status: 'requested' | 'running' | 'success' | 'failed'
    createdAt: string
    completedAt?: string
    processedCount: number
    /** Total work, when it is known up front. **/
    totalCount?: number
    /** migrate-pages: one entry per finished migration, in request order. **/
    pageMigrationResults?: Array<PageMigrationSuccess | PageMigrationFailure>
    /**
     * migrate-pages: set once the job has finished (success or failed). Pass to POST /api/v1/pages/migrations/:undoId/undo within 90 days.
     * undo-page-migration: the migration being undone.
     **/
    undoId?: string
    /** undo-page-migration: set once the job has finished. **/
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
    /** Records put back to their values from before the migration. **/
    restored: number
    /** Records left alone because they changed after the migration, or no longer exist. **/
    skipped: number
    /** Records the migration deleted while merging pages. These cannot be brought back. **/
    notRestorable: number
}
[inline-code-end]

#### Ponowne próby

Jeśli serwer uruchamiający zadanie migracji stron zostanie zrestartowany, zadanie zostaje ponownie umieszczone w kolejce i kontynuuje po ostatniej zakończonej migracji. Zadanie jest podejmowane maksymalnie 3 razy, po czym jego `status` jest `failed`.