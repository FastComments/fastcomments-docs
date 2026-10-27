[api-resource-header-start name = 'Page Migrations'; route = 'POST /api/v1/pages/migrations/:undoId/undo'; creditsCost = 1; api-resource-header-end]

Ова рута поништава сваку миграцију из једног [`POST /api/v1/pages/migrations`](#pages-migrations-post) посла. Добијте `undoId` из завршеног посла миграције помоћу [`GET /api/v1/batch-jobs/:id`](#batch-job-get-by-id). `undoId` се може користити 90 дана, и само једном.

Поништавање се извршава као batch посао, исто као миграција, а одговор садржи његов `batchJobId`. Када се посао заврши, његов `pageMigrationUndoResult` извештава:

- `restored`: записи враћени на своје вредности пре миграције.
- `skipped`: записи остављени нетакнути јер су се променили након миграције, или више не постоје. Каснија измена се никада не преписује.
- `notRestorable`: записи које је миграција обрисала приликом спајања две странице, или уклањања дупликата реакција странице. Ови се не могу вратити.

Миграције се поништавају од најновијих према старијим, тако да се ланац попут `a` у `b` а затим `b` у `c` развија назад до `a`. Посао миграције који је пропао на половини и даље се може поништити, што обрће промене које је направио пре пада.

Стављање поништавања у ред није скупо. Када се посао заврши, кошта `1` кредит по `1000` враћених записа, са минимумом `1`.

[inline-code-attrs-start title = 'Пример cURL захтева за поништавање миграција страница'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/pages/migrations/my-undo-id/undo?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Структура захтева за поништавање миграција страница'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface UndoPageMigrationsQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура одговора за поништавање миграција страница'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface UndoPageMigrationsResponse {
    status: 'success' | 'failed'
    /**
     * Included on failure.
     * undo-not-found (404): no migration has this undoId, or it is older than 90 days.
     * already-undone (409) and undo-in-progress (409): the migration has been undone, or an undo of it is queued or running.
     **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'undo-not-found' | 'already-undone' | 'undo-in-progress' | 'internal'
    /** Included on failure. **/
    reason?: string
    /** Pass to GET /api/v1/batch-jobs/:id for progress and the result. **/
    batchJobId?: string
}
[inline-code-end]