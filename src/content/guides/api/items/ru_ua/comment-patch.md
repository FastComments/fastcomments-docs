[api-resource-header-start name = 'Comment'; route = 'PATCH /api/v1/comments/:id'; creditsCost = 1; api-resource-header-end]

Этот API-эндпоинт предоставляет возможность обновить один комментарий.

Notes:

- Этот API может обновлять виджет комментариев "вживую" при необходимости (это увеличивает базовую стоимость `creditsCost` с `1` до `2`).
  - Это позволяет выполнять миграцию комментариев между страницами "вживую" (изменяя `urlId`).
  - Миграции стоят дополнительно `2` кредита, так как страницы предварительно вычисляются и это требует большой нагрузки на CPU.
- В отличие от API создания, этот API НЕ будет автоматически создавать объекты пользователей в нашей системе, если указан email.
- Комментарии, обновлённые через этот API, всё ещё могут быть проверены на спам при необходимости.
- Конфигурации, такие как максимальная длина комментария, если они настроены через страницу администрирования Customization Rule, будут применяться здесь.
- Чтобы позволить пользователям обновить текст их комментария, можно просто указать `comment` в теле запроса. Мы сгенерируем результирующий `commentHTML`.
  - Если вы зададите и `comment`, и `commentHTML`, мы не будем автоматически генерировать HTML.
  - Если пользователь добавит упоминания или хэштеги в новом тексте, они всё равно будут обработаны так же, как в `POST` API.
- При обновлении `commenterEmail` в комментарии, лучше также указать `userId`. В противном случае вы должны убедиться, что пользователь с этим email принадлежит вашему тенанту, иначе запрос завершится с ошибкой.  


[inline-code-attrs-start title = 'Минимальный пример cURL для PATCH комментария'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET&isLive=true' \
  --header 'Content-Type: application/json' \
  --data '{
	"comment": "some-new-comment-text"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Структура запроса PATCH комментария'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentPatchQueryParams {
    tenantId: string
    API_KEY: string
	/** Пользователь, выполняющий обновление. При необходимости может быть использован для проверки возможности редактирования комментария.  **/
    contextUserId?: string
	/** Проверять ли, выглядит ли новый комментарий как спам?  **/
    doSpamCheck?: 'true' | 'false'
	/** Появляться ли комментарий "вживую" для пользователей, просматривающих экземпляры виджета комментариев с таким же urlId. ПРИМЕЧАНИЕ: удваивает стоимость в кредитах с 1 до 2. **/
    isLive?: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура ответа PATCH комментария'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentPatchResponse {
    status: 'success' | 'failed'
    /** Указывается при ошибке. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'empty-comment' | 'comment-too-big' | 'hash-tags-readonly' | 'mentions-readonly' | 'invalid-user' | 'unauthorized' | 'invalid-date' | 'invalid-name' | 'invalid-name-is-email' | 'banned' | 'invalid-email' | 'invalid-input' | 'missing-id' | 'not-found'
    /** Указывается при ошибке. **/
    reason?: string
}
[inline-code-end]