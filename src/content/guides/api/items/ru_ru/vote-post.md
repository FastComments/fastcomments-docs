[api-resource-header-start name = 'Vote'; route = 'POST /api/v1/votes'; creditsCost = 1; api-resource-header-end]

Этот маршрут предоставляет возможность добавить один авторизованный `Vote`. Голоса могут быть `up` (+1) или `down` (-1).

[inline-code-attrs-start title = 'Пример cURL-запроса создания голоса'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/votes?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=user-id&commentId=comment-id&direction=up' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = 'Пример cURL-запроса создания анонимного голоса'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/votes?tenantId=demo&API_KEY=DEMO_API_SECRET&anonUserId=some-randomly-generated-identifier&commentId=comment-id&direction=up' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = 'Структура запроса для создания голоса'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Структура ответа при создании голоса'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotePostResponse {
    status: 'success' | 'failed'
    /** Включается при ошибке. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-user-id' | 'invalid-user-id' | 'invalid-comment-id' | 'invalid-direction' | 'duplicate' | 'voting-disabled'
    /** Включается при ошибке. **/
    reason?: string
    voteId?: string
}
[inline-code-end]

### Создание анонимных голосов

Анонимные голоса можно создать, указав `anonUserId` в параметрах запроса вместо `userId`.

Этот идентификатор не обязательно должен соответствовать объекту пользователя где-либо (отсюда — анонимность). Он просто является идентификатором
для сессии, поэтому вы можете снова получить голоса в той же сессии, чтобы проверить, был ли за комментарий
отдан голос.

Если у вас нет такой вещи, как "анонимные сессии", как у FastComments, - вы можете просто
задать это случайным идентификатором, например UUID (хотя мы ценим более короткие идентификаторы для экономии места).

### Другие примечания

- Этот API подчиняется настройкам на уровне тенанта. Например, если вы отключите голосование для данной страницы и попытаетесь создать голос через API, это завершится ошибкой с кодом `voting-disabled`.
- Этот API активен по умолчанию.
- Этот API обновит поле `votes` соответствующего `Comment`.