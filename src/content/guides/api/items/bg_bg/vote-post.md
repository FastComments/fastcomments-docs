[api-resource-header-start name = 'Vote'; route = 'POST /api/v1/votes'; creditsCost = 1; api-resource-header-end]

Този маршрут предоставя възможност за добавяне на единичен оторизиран `Vote`. Гласовете могат да бъдат `up` (+1) или `down` (-1).

[inline-code-attrs-start title = 'Пример за създаване на Vote с cURL'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/votes?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=user-id&commentId=comment-id&direction=up' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = 'Пример за създаване на анонимен Vote с cURL'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/votes?tenantId=demo&API_KEY=DEMO_API_SECRET&anonUserId=some-randomly-generated-identifier&commentId=comment-id&direction=up' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = 'Структура на заявката за създаване на Vote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotePostQueryParams {
    tenantId: string
    API_KEY: string
    userId?: string
    anonUserId?: string
    commentId: string
    direction: 'up' | 'down'
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура на отговора за създаване на Vote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotePostResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-user-id' | 'invalid-user-id' | 'invalid-comment-id' | 'invalid-direction' | 'duplicate' | 'voting-disabled'
    /** Included on failure. **/
    reason?: string
    voteId?: string
}
[inline-code-end]

### Създаване на анонимни гласове

Анонимни гласове могат да бъдат създадени чрез задаване на `anonUserId` в параметрите на заявката вместо `userId`.

Този id не трябва да съответства на потребителски обект никъде (оттук анонимен). Това е просто идентификатор
за сесията, така че да можете да извлечете гласовете отново в същата сесия, за да проверите дали даден коментар е
бил гласуван.

Ако нямате такова нещо като "анонимни сесии" както FastComments - можете просто
да зададете това на случаен ID, като UUID (въпреки че оценяваме по-малки идентификатори за спестяване на място).

### Други бележки

- Този API се подчинява на настройките на ниво tenant. Например, ако деактивирате гласуването за дадена страница и се опитате да създадете глас чрез API, то ще се провали с код на грешка `voting-disabled`.
- Този API е на живо по подразбиране.
- Този API ще актуализира `votes` на съответния `Comment`.
