A `BatchJob` — це довготривале завдання, таке як імпорт, експорт або [перенесення сторінки](#pages-migrations-post). Завдання ставляться в чергу, а потім беруться і виконуються у фоні.

`processedCount` та `totalCount` показують прогрес. Їхня одиниця вимірювання залежить від `type`:

- `migrate-pages`: завершені перенесення, з загальної кількості запитаних перенесень.
- `undo-page-migration`: відновлені записи. `totalCount` відсутній.
- `import`, `export` та інші завдання: оброблені рядки.

Структура об’єкта `BatchJob` виглядає наступним чином:

[inline-code-attrs-start title = 'Структура BatchJob'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface BatchJob {
    id: string
    /** Наприклад migrate-pages, undo-page-migration, migrate-domains, import, export, users-export або copy-comments. **/
    type: string
    status: 'requested' | 'running' | 'success' | 'failed'
    createdAt: string
    completedAt?: string
    processedCount: number
    /** Загальна кількість роботи, якщо вона відома заздалегідь. **/
    totalCount?: number
    /** migrate-pages: один запис на кожне завершене перенесення, у порядку запиту. **/
    pageMigrationResults?: Array<PageMigrationSuccess | PageMigrationFailure>
    /**
     * migrate-pages: встановлюється після завершення завдання (успішно або з помилкою). Передайте у POST /api/v1/pages/migrations/:undoId/undo протягом 90 днів.
     * undo-page-migration: перенесення, яке скасовується.
     **/
    undoId?: string
    /** undo-page-migration: встановлюється після завершення завдання. **/
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
    /** Записи, повернуті до їхніх значень до перенесення. **/
    restored: number
    /** Записи, залишені без змін, оскільки вони змінилися після перенесення або більше не існують. **/
    skipped: number
    /** Записи про перенесення, видалені під час об’єднання сторінок. Їх неможливо відновити. **/
    notRestorable: number
}
[inline-code-end]

#### Повторні спроби

Якщо сервер, який виконує завдання перенесення сторінки, перезапускається, завдання повертається в чергу і продовжується після останнього завершеного перенесення. Завдання пробується до 3 разів, після чого його `status` стає `failed`.

---