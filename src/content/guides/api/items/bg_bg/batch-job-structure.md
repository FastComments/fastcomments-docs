`BatchJob` е дълготрайна задача, като импорт, експорт или [миграция на страници](#pages-migrations-post). Задачите се поставят в опашка, след което се вземат и изпълняват във фонов режим.

`processedCount` и `totalCount` показват напредъка. Единицата им зависи от `type`:

- `migrate-pages`: завършени миграции, от общия брой заявени миграции.
- `undo-page-migration`: възстановени записи. Няма `totalCount`.
- `import`, `export` и други задачи: обработени редове.

Структурата на обекта `BatchJob` е следната:

[inline-code-attrs-start title = 'Структура на BatchJob'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface BatchJob {
    id: string
    /** Например migrate-pages, undo-page-migration, migrate-domains, import, export, users-export или copy-comments. **/
    type: string
    status: 'requested' | 'running' | 'success' | 'failed'
    createdAt: string
    completedAt?: string
    processedCount: number
    /** Общият обем работа, когато е известен предварително. **/
    totalCount?: number
    /** migrate-pages: един запис за всяка завършена миграция, в реда на заявката. **/
    pageMigrationResults?: Array<PageMigrationSuccess | PageMigrationFailure>
    /**
     * migrate-pages: задава се, след като задачата завърши (успешно или неуспешно). Предайте към POST /api/v1/pages/migrations/:undoId/undo в рамките на 90 дни.
     * undo-page-migration: миграцията, която се отменя.
     **/
    undoId?: string
    /** undo-page-migration: задава се, след като задачата завърши. **/
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
    /** Записи, върнати към стойностите им преди миграцията. **/
    restored: number
    /** Записи, оставени непроменени, защото са се променили след миграцията или вече не съществуват. **/
    skipped: number
    /** Записи от миграцията, изтрити при обединяване на страници. Те не могат да бъдат възстановени. **/
    notRestorable: number
}
[inline-code-end]

#### Повторни опити

Ако сървърът, изпълняващ задача за миграция на страници, се рестартира, задачата се поставя отново в опашката и се продължава след последната завършена миграция. Задачата се опитва до 3 пъти, след което нейният `status` е `failed`.

---