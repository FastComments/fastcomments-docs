[api-resource-header-start name = 'Vote'; route = 'DELETE /api/v1/votes/:id'; creditsCost = 1; api-resource-header-end]

Этот маршрут предоставляет возможность удалить один `Vote`.

[inline-code-attrs-start title = 'Пример запроса cURL для удаления Vote'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/votes/some-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = 'Структура запроса на удаление Vote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VoteDeleteQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура ответа при удалении Vote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VoteDeleteResponse {
    status: 'success' | 'failed'
    /** Включается при ошибке. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id'
    /** Включается при ошибке. **/
    reason?: string
}
[inline-code-end]

Примечания:

- Этот API соблюдает настройки на уровне арендатора. Например, если вы отключите голосование для определённой страницы, и вы попытаетесь создать голос через API, это завершится ошибкой с кодом `voting-disabled`.
- Этот API по умолчанию активен.
- Этот API обновит `votes` соответствующего `Comment`.