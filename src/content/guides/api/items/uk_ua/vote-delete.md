[api-resource-header-start name = 'Vote'; route = 'DELETE /api/v1/votes/:id'; creditsCost = 1; api-resource-header-end]

Цей маршрут надає можливість видалити один `Vote`.

[inline-code-attrs-start title = 'Приклад cURL для видалення Vote'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/votes/some-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = 'Структура запиту на видалення Vote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VoteDeleteQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура відповіді на видалення Vote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VoteDeleteResponse {
    status: 'success' | 'failed'
    /** Включається у разі невдачі. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id'
    /** Включається у разі невдачі. **/
    reason?: string
}
[inline-code-end]

Примітки:

- Цей API дотримується налаштувань на рівні орендаря. Наприклад, якщо ви вимкнете голосування для певної сторінки, і ви спробуєте створити голос через API, це завершиться помилкою з кодом `voting-disabled`.
- Цей API за замовчуванням є активним.
- Цей API оновить `votes` відповідного `Comment`.