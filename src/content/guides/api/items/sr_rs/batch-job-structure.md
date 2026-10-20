A `BatchJob` је дуготрајни задатак, као што је увоз, извоз или [page migration](#pages-migrations-post). Послови се стављају у ред, а затим се преузимају и извршавају у позадини.

`processedCount` и `totalCount` приказују напредак. Њихова јединица зависи од `type`:

- `migrate-pages`: завршене миграције, од укупно захтеваних миграција.
- `undo-page-migration`: враћени записи. Нема `totalCount`.
- `import`, `export` и остали послови: обрађени редови.

Структура за `BatchJob` објекат је следећа:

[inline-code-attrs-start title = 'Структура BatchJob'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface BatchJob {
    id: string
    /** На пример migrate-pages, undo-page-migration, migrate-domains, import, export, users-export или copy-comments. **/
    type: string
    status: 'requested' | 'running' | 'success' | 'failed'
    createdAt: string
    completedAt?: string
    processedCount: number
    /** Укупан рад, када је унапред познат. **/
    totalCount?: number
    /** migrate-pages: један унос по завршеној миграцији, у редоследу захтева. **/
    pageMigrationResults?: Array<PageMigrationSuccess | PageMigrationFailure>
    /**
     * migrate-pages: поставља се када се посао заврши (успешно или неуспешно). Проследите у POST /api/v1/pages/migrations/:undoId/undo у року од 90 дана.
     * undo-page-migration: миграција која се поништава.
     **/
    undoId?: string
    /** undo-page-migration: поставља се када се посао заврши. **/
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
    /** Записи враћени на своје вредности пре миграције. **/
    restored: number
    /** Записи остављени нетакнути јер су се променили након миграције, или више не постоје. **/
    skipped: number
    /** Записи миграције обрисани приликом спајања страница. Ови се не могу вратити. **/
    notRestorable: number
}
[inline-code-end]

#### Поновни покушаји

Ако сервер који извршава посао миграције странице поново покрене, посао се враћа у ред и наставља након последње завршене миграције. Посао се покушава до 3 пута, након чега је његов `status` `failed`.