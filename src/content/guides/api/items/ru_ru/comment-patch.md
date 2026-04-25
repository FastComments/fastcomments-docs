[api-resource-header-start name = 'Comment'; route = 'PATCH /api/v1/comments/:id'; creditsCost = 1; api-resource-header-end]

Этот API-эндпоинт предоставляет возможность обновить один комментарий.

Notes:

- Этот API может обновлять виджет комментариев "вживую" при желании (this increases base `creditsCost` from `1` to `2`).
  - Это может сделать миграцию комментариев между страницами "вживую" (changing `urlId`).
  - Миграции стоят дополнительные `2` кредита, так как страницы предрассчитываются и это ресурсоёмко для CPU.
- В отличие от create API, этот API НЕ будет автоматически создавать объекты пользователей в нашей системе, если указан адрес электронной почты.
- Комментарии, обновлённые через этот API, по-прежнему можно проверять на спам при желании.
- Конфигурации, такие как максимальная длина комментария, если настроены через страницу администрирования Customization Rule, будут применяться здесь.
- Чтобы позволить пользователям обновлять текст своего комментария, вы можете просто указать `comment` в теле запроса. Мы сгенерируем результирующий `commentHTML`.
  - Если вы зададите и `comment`, и `commentHTML`, мы не будем автоматически генерировать HTML.
  - Если пользователь добавит упоминания или хэштеги в новом тексте, они всё равно будут обрабатываться так же, как в `POST` API.
- При обновлении `commenterEmail` в комментарии лучше также указать `userId`. В противном случае вы должны убедиться, что пользователь с этим адресом электронной почты принадлежит вашему тенанту, иначе запрос завершится ошибкой.  
- Если целевой комментарий заблокирован (`isLocked: true`), запрос отклоняется с `code: 'locked'`. Сначала разблокируйте комментарий, обновите его, затем при желании снова заблокируйте.


[inline-code-attrs-start title = 'Минимальный пример PATCH cURL для комментария'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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
	/** Пользователь, выполняющий обновление. При желании может использоваться для проверки того, что он может редактировать комментарий.  **/
    contextUserId?: string
	/** Следует ли проверить, похож ли новый комментарий на спам?  **/
    doSpamCheck?: 'true' | 'false'
	/** Должен ли комментарий отображаться "вживую" пользователям, просматривающим экземпляры виджета комментариев с тем же urlId. NOTE: Doubles credit cost from 1 to 2. **/
    isLive?: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура ответа PATCH комментария'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentPatchResponse {
    status: 'success' | 'failed'
    /** Включается в случае неудачи. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'empty-comment' | 'comment-too-big' | 'hash-tags-readonly' | 'mentions-readonly' | 'invalid-user' | 'unauthorized' | 'invalid-date' | 'invalid-name' | 'invalid-name-is-email' | 'banned' | 'invalid-email' | 'invalid-input' | 'missing-id' | 'not-found' | 'locked'
    /** Включается в случае неудачи. **/
    reason?: string
}
[inline-code-end]