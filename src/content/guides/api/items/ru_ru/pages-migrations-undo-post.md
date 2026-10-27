[api-resource-header-start name = 'Page Migrations'; route = 'POST /api/v1/pages/migrations/:undoId/undo'; creditsCost = 1; api-resource-header-end]

Этот маршрут отменяет каждую миграцию из одного задания [`POST /api/v1/pages/migrations`](#pages-migrations-post). Получите `undoId` из завершённого задания миграции с помощью [`GET /api/v1/batch-jobs/:id`](#batch-job-get-by-id). `undoId` можно использовать в течение 90 дней и только один раз.

Отмена выполняется как пакетное задание, так же как и миграция, и ответ содержит его `batchJobId`. После завершения задания его `pageMigrationUndoResult` сообщает:

- `restored`: записи, возвращённые к значениям до миграции.
- `skipped`: записи, оставленные без изменений, потому что они изменились после миграции или больше не существуют. Позднее редактирование никогда не перезаписывается.
- `notRestorable`: записи, которые миграция удалила при объединении двух страниц или удалении дублирующих реакций страниц. Их нельзя восстановить.

Миграции отменяются в порядке от новых к старым, поэтому цепочка вроде `a` в `b`, а затем `b` в `c` разворачивается обратно к `a`. Задание миграции, которое завершилось с ошибкой, всё ещё можно отменить, что обратит изменения, сделанные до сбоя.

Постановка задачи отмены не стоит ничего. Когда задание завершается, стоимость составляет `1` кредит за каждые `1000` восстановленных записей, минимум `1`.

[inline-code-attrs-start title = 'Пример cURL для отмены миграций страниц'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/pages/migrations/my-undo-id/undo?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Структура запроса отмены миграций страниц'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface UndoPageMigrationsQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура ответа отмены миграций страниц'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface UndoPageMigrationsResponse {
    status: 'success' | 'failed'
    /**
     * Включено при ошибке.
     * undo-not-found (404): нет миграции с этим undoId, либо он старше 90 дней.
     * already-undone (409) и undo-in-progress (409): миграция уже отменена, либо её отмена находится в очереди или выполняется.
     **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'undo-not-found' | 'already-undone' | 'undo-in-progress' | 'internal'
    /** Включено при ошибке. **/
    reason?: string
    /** Передайте в GET /api/v1/batch-jobs/:id для получения прогресса и результата. **/
    batchJobId?: string
}
[inline-code-end]