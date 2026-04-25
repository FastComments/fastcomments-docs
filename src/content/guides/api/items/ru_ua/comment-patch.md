[api-resource-header-start name = 'Comment'; route = 'PATCH /api/v1/comments/:id'; creditsCost = 1; api-resource-header-end]

Этот конечный метод API предоставляет возможность обновить один комментарий.

Notes:

- Этот API может обновлять виджет комментариев «live», если это необходимо (это увеличивает базовую стоимость `creditsCost` с `1` до `2`).
  - Это позволяет выполнять «живую» миграцию комментариев между страницами (изменяя `urlId`).
  - Миграции стоят дополнительно `2` кредита, так как страницы предварительно вычисляются и это требует высокой загрузки CPU.
- В отличие от API создания, этот API НЕ будет автоматически создавать объекты пользователей в нашей системе, если указан email.
- Комментарии, обновлённые через этот API, при желании всё ещё могут быть проверены на спам.
- Конфигурации, такие как максимальная длина комментария, если они настроены через страницу администратора Customization Rule, будут применяться здесь.
- Чтобы позволить пользователям обновлять текст своего комментария, вы можете просто указать `comment` в теле запроса. Мы сгенерируем результирующий `commentHTML`.
  - Если вы укажете и `comment`, и `commentHTML`, мы не будем автоматически генерировать HTML.
  - Если пользователь добавит упоминания или хэштеги в новом тексте, они всё равно будут обработаны так же, как в `POST` API.
- При обновлении `commenterEmail` в комментарии лучше также указать `userId`. В противном случае вы должны убедиться, что пользователь с этим email принадлежит вашему тенанту, иначе запрос завершится ошибкой.  
- Если целевой комментарий заблокирован (`isLocked: true`), запрос отклоняется с `code: 'locked'`. Сначала разблокируйте комментарий, обновите его, затем при необходимости заблокируйте снова.


[inline-code-attrs-start title = 'Минимальный пример PATCH cURL для комментария'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET&isLive=true' \
  --header 'Content-Type: application/json' \
  --data '{
	"comment": "some-new-comment-text"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Структура PATCH-запроса для комментария'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentPatchQueryParams {
    tenantId: string
    API_KEY: string
	/** Пользователь, выполняющий обновление. При необходимости может использоваться для проверки права редактирования комментария.  **/
    contextUserId?: string
	/** Нужно ли проверять, не выглядит ли новый комментарий как спам?  **/
    doSpamCheck?: 'true' | 'false'
	/** Должен ли комментарий отображаться «live» для пользователей, просматривающих экземпляры виджета комментариев с тем же urlId. ВНИМАНИЕ: Удваивает стоимость кредитов с 1 до 2. **/
    isLive?: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура ответа PATCH для комментария'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentPatchResponse {
    status: 'success' | 'failed'
    /** Включается в случае ошибки. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'empty-comment' | 'comment-too-big' | 'hash-tags-readonly' | 'mentions-readonly' | 'invalid-user' | 'unauthorized' | 'invalid-date' | 'invalid-name' | 'invalid-name-is-email' | 'banned' | 'invalid-email' | 'invalid-input' | 'missing-id' | 'not-found' | 'locked'
    /** Включается в случае ошибки. **/
    reason?: string
}
[inline-code-end]