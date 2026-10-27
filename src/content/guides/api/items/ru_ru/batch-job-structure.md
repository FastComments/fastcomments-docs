A `BatchJob` — это длительная задача, например импорт, экспорт или [миграция страниц](#pages-migrations-post). Задачи ставятся в очередь, затем берутся и выполняются в фоновом режиме.

`processedCount` и `totalCount` показывают прогресс. Их единица измерения зависит от `type`:

- `migrate-pages`: завершённые миграции из запрошенных миграций.
- `undo-page-migration`: восстановленные записи. `totalCount` отсутствует.
- `import`, `export` и другие задачи: обработанные строки.

Структура объекта `BatchJob` выглядит следующим образом:

[inline-code-attrs-start title = 'Структура BatchJob'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface BatchJob {
    id: string
    /** Например migrate-pages, undo-page-migration, migrate-domains, import, export, users-export или copy-comments. **/
    type: string
    status: 'requested' | 'running' | 'success' | 'failed'
    createdAt: string
    completedAt?: string
    processedCount: number
    /** Общее количество работы, если оно известно заранее. **/
    totalCount?: number
    /** migrate-pages: одна запись на каждую завершённую миграцию, в порядке запроса. **/
    pageMigrationResults?: Array<PageMigrationSuccess | PageMigrationFailure>
    /**
     * migrate-pages: устанавливается после завершения задачи (успешно или с ошибкой). Передайте в POST /api/v1/pages/migrations/:undoId/undo в течение 90 дней.
     * undo-page-migration: миграция, которую нужно отменить.
     **/
    undoId?: string
    /** undo-page-migration: устанавливается после завершения задачи. **/
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
    /** Записи, возвращённые к их значениям до миграции. **/
    restored: number
    /** Записи, оставленные без изменений, потому что они изменились после миграции или больше не существуют. **/
    skipped: number
    /** Записи миграции, удалённые при объединении страниц. Их нельзя восстановить. **/
    notRestorable: number
}
[inline-code-end]

#### Повторы

Если сервер, выполняющий задачу миграции страниц, перезапускается, задача возвращается в очередь и продолжается после последней завершённой миграции. Задача пытается выполниться до 3 раз, после чего её `status` становится `failed`.