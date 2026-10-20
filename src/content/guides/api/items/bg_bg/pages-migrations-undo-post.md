[api-resource-header-start name = 'Page Migrations'; route = 'POST /api/v1/pages/migrations/:undoId/undo'; creditsCost = 1; api-resource-header-end]

Този маршрут обръща всяка миграция от едно [`POST /api/v1/pages/migrations`](#pages-migrations-post) задание. Вземете `undoId` от завършеното миграционно задание с [`GET /api/v1/batch-jobs/:id`](#batch-job-get-by-id). `undoId` може да се използва в продължение на 90 дни и само веднъж.

Отмяната се изпълнява като пакетно задание, същото като миграцията, и отговорът съдържа неговия `batchJobId`. След като заданието завърши, неговият `pageMigrationUndoResult` докладва:

- `restored`: записи, върнати към стойностите им преди миграцията.
- `skipped`: записи, оставени непроменени, защото са се променили след миграцията или вече не съществуват. По‑късно редактиране никога не се презаписва.
- `notRestorable`: записи, които миграцията изтрива, докато се слива две страници или се премахват дублиращи се реакции на страници. Тези записи не могат да бъдат възстановени.

Миграциите се обръщат от най‑новата към най‑старата, така че верига като `a` към `b`, последвана от `b` към `c`, се развива обратно до `a`. Задание за миграция, което е провалено по средата, все още може да бъде отменено, като се върнат промените, направени преди провала.

Поставянето в опашка за отмяна не струва нищо. Когато заданието завърши, се таксува `1` кредит за всеки `1000` възстановени записа, с минимум `1`.

[inline-code-attrs-start title = 'Пример за cURL за отмяна на миграции на страници'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/pages/migrations/my-undo-id/undo?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Структура на заявка за отмяна на миграции на страници'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface UndoPageMigrationsQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура на отговор за отмяна на миграции на страници'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface UndoPageMigrationsResponse {
    status: 'success' | 'failed'
    /**
     * Включено при неуспех.
     * undo-not-found (404): няма миграция с този undoId, или е по-стара от 90 дни.
     * already-undone (409) и undo-in-progress (409): миграцията е отменена, или отмяната ѝ е поставена в опашка или се изпълнява.
     **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'undo-not-found' | 'already-undone' | 'undo-in-progress' | 'internal'
    /** Включено при неуспех. **/
    reason?: string
    /** Предайте на GET /api/v1/batch-jobs/:id за прогрес и резултат. **/
    batchJobId?: string
}
[inline-code-end]