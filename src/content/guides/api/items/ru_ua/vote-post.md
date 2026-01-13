[api-resource-header-start name = 'Vote'; route = 'POST /api/v1/votes'; creditsCost = 1; api-resource-header-end]

Этот маршрут предоставляет возможность добавить один авторизованный `Vote`. Голос может быть `up` (+1) или `down` (-1).

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

[inline-code-attrs-start title = 'Структура запроса создания голоса'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

Анонимные голоса можно создать, передав `anonUserId` в параметрах запроса вместо `userId`.

Этот идентификатор не обязательно должен соответствовать объекту пользователя где-либо (отсюда — анонимность). Это просто идентификатор сессии, чтобы вы могли повторно получить голоса в той же сессии и проверить, был ли за комментарий отдан голос.

Если у вас нет «анонимных сессий», как у FastComments, вы можете просто присвоить этому значение случайного идентификатора, например UUID (хотя мы предпочитаем более короткие идентификаторы для экономии места).

### Прочие заметки

- Этот API следует настройкам на уровне тенанта. Например, если вы отключите голосование для данной страницы и попытаетесь создать голос через API, это завершится с ошибкой с кодом `voting-disabled`.
- Этот API активен по умолчанию.
- Этот API обновит `votes` соответствующего `Comment`.

---