[api-resource-header-start name = 'Page Migrations'; route = 'POST /api/v1/pages/migrations/:undoId/undo'; creditsCost = 1; api-resource-header-end]

Цей маршрут скасовує кожну міграцію з одного завдання [`POST /api/v1/pages/migrations`](#pages-migrations-post). Отримайте `undoId` з завершеного завдання міграції за допомогою [`GET /api/v1/batch-jobs/:id`](#batch-job-get-by-id). `undoId` можна використовувати протягом 90 днів, і лише один раз.

Скасування виконується як пакетне завдання, так само як і міграція, і відповідь містить його `batchJobId`. Після завершення завдання його `pageMigrationUndoResult` повідомляє:

- `restored`: записи, повернені до їхніх значень до міграції.
- `skipped`: записи, залишені без змін, оскільки вони змінилися після міграції або більше не існують. Пізніше редагування ніколи не перезаписується.
- `notRestorable`: записи, які міграція видали під час об’єднання двох сторінок або видалення дублікатів реакцій сторінки. Їх неможливо відновити.

Міграції скасовуються у зворотному порядку, спочатку найновіші, тому ланцюжок типу `a` до `b`, а потім `b` до `c` розмотується назад до `a`. Завдання міграції, яке завершилось з помилкою на половині, все ще можна скасувати, що поверне зміни, які воно внесло до збою.

Додавання скасування в чергу не коштує нічого. Коли завдання завершується, вартість становить `1` кредит за `1000` відновлених записів, мінімум `1`.

[inline-code-attrs-start title = 'Приклад cURL для скасування міграцій сторінок'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/pages/migrations/my-undo-id/undo?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Структура запиту скасування міграцій сторінок'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface UndoPageMigrationsQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура відповіді скасування міграцій сторінок'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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