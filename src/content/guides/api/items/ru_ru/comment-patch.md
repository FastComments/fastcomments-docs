[api-resource-header-start name = 'Comment'; route = 'PATCH /api/v1/comments/:id'; creditsCost = 1; api-resource-header-end]

Этот API-эндпоинт предоставляет возможность обновления одного комментария.

Примечания:

- Этот API может обновлять виджет комментариев "в реальном времени", если нужно (это увеличит базовую `creditsCost` с `1` до `2`).
  - Это позволяет делать миграции комментариев между страницами "в реальном времени" (изменяя `urlId`).
  - Миграции стоят дополнительно `2` кредита, так как страницы предварительно вычисляются, и это интенсивно по использованию CPU.
- В отличие от API создания, этот API НЕ будет автоматически создавать объекты пользователя в нашей системе, если указан email.
- Комментарии, обновлённые через этот API, по-прежнему могут проверяться на спам при необходимости.
- Конфигурации, такие как максимальная длина комментария, если настроены через страницу администратора Customization Rule, будут применяться здесь.
- Чтобы позволить пользователям обновлять текст комментария, можно просто указать `comment` в теле запроса. Мы сгенерируем результирующий `commentHTML`.
  - Если вы укажете и `comment`, и `commentHTML`, мы не будем автоматически генерировать HTML.
  - Если пользователь добавит упоминания или хэштеги в новом тексте, они по-прежнему будут обрабатываться так же, как в `POST` API.
- При обновлении `commenterEmail` для комментария лучше также указать `userId`. В противном случае вы должны убедиться, что пользователь с этим email принадлежит вашему тенанту, иначе запрос завершится ошибкой.  


[inline-code-attrs-start title = 'Минимальный пример PATCH cURL для комментария'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET&isLive=true' \
  --header 'Content-Type: application/json' \
  --data '{
	"comment": "some-new-comment-text"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Структура PATCH-запроса комментария'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentPatchQueryParams {
    tenantId: string
    API_KEY: string
	/** Пользователь, выполняющий обновление. При желании можно использовать для проверки, что он может редактировать комментарий.  **/
    contextUserId?: string
	/** Проверять ли, похож ли новый комментарий на спам?  **/
    doSpamCheck?: 'true' | 'false'
	/** Должен ли комментарий появляться "в реальном времени" для пользователей, просматривающих экземпляры виджета комментариев с тем же urlId. ВНИМАНИЕ: удваивает стоимость в кредитах с 1 до 2. **/
    isLive?: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура PATCH-ответа комментария'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentPatchResponse {
    status: 'success' | 'failed'
    /** Включается в случае неудачи. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'empty-comment' | 'comment-too-big' | 'hash-tags-readonly' | 'mentions-readonly' | 'invalid-user' | 'unauthorized' | 'invalid-date' | 'invalid-name' | 'invalid-name-is-email' | 'banned' | 'invalid-email' | 'invalid-input' | 'missing-id' | 'not-found'
    /** Включается в случае неудачи. **/
    reason?: string
}
[inline-code-end]