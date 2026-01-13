[api-resource-header-start name = 'Comment'; route = 'POST /api/v1/comments'; creditsCost = 1; api-resource-header-end]

Этот API-эндпойнт предоставляет возможность создавать комментарии.

Типичные случаи использования: кастомные интерфейсы, интеграции или импорты.

Примечания:

- Этот API может обновлять виджет комментариев "в реальном времени", если нужно (это увеличивает `creditsCost` с `1` до `2`).
- Этот API автоматически создаст объекты пользователей в нашей системе, если указан адрес электронной почты.
- Попытка сохранить два комментария с разными адресами электронной почты, но с одинаковым именем пользователя приведёт к ошибке для второго комментария.
- Если вы указываете `parentId`, и дочерний комментарий имеет `notificationSentForParent` as false, **мы отправим уведомления для родительского комментария**. Это делается каждый час (мы группируем уведомления вместе, чтобы уменьшить число отправляемых писем).
- Если вы хотите отправлять приветственные письма при создании пользователей или письма для подтверждения комментариев, установите `sendEmails` в `true` в параметрах запроса.
- Комментарии, созданные через этот API, будут отображаться на страницах Analytics и Moderation в административном приложении.
- "bad words" по-прежнему маскируются в именах комментаторов и тексте комментария, если настройка включена.
- Комментарии, созданные через этот API, по-прежнему можно проверять на спам при необходимости.
- Настройки, такие как максимальная длина комментария, если они настроены через страницу администрирования Customization Rule, будут применяться и здесь.

Минимальные данные, необходимые для отправки и отображения в виджете комментариев, выглядят следующим образом:

[inline-code-attrs-start title = 'Минимальный пример POST cURL для комментария'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&API_KEY=DEMO_API_SECRET&isLive=true' \
  --header 'Content-Type: application/json' \
  --data '{
	"approved": true,
	"comment": "some-comment",
	"commenterName": "some-commenter-name",
	"date": 1622644382148,
	"urlId": "some-place",
	"url": "https://exmaple.com/some-page",
	"verified": true
}'
[inline-code-end]

Более реалистичный запрос может выглядеть так:

[inline-code-attrs-start title = 'Пример POST cURL для комментария'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&API_KEY=DEMO_API_SECRET&isLive=true&doSpamCheck=true' \
  --header 'Content-Type: application/json' \
  --data '{
	"approved": true,
	"avatarSrc": "https://static.fastcomments.com/1605337537848-DSC_0841.JPG",
	"comment": "some-comment",
	"commenterName": "some-commenter-name",
	"commenterEmail": "fordperfect@spaceship.com",
	"date": 1622644382148,
	"isSpam": false,
	"locale": "en_us",
	"notificationSentForParent": true,
	"notificationSentForParentTenant": true,
	"reviewed": true,
	"urlId": "some-place",
	"url": "https://exmaple.com/some-page",
	"verified": true,
	"votes": 1,
	"votesUp": 2,
	"votesDown": 1,
	"ip": "123.456.789.000"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Структура POST-запроса комментария'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentPostQueryParams {
    tenantId: string
    API_KEY: string
    doSpamCheck?: 'true' | 'false'
	/** Должен ли комментарий отображаться "в реальном времени" для пользователей, просматривающих экземпляры виджета комментариев с тем же urlId. ПРИМЕЧАНИЕ: Удваивает стоимость в кредите с 1 до 2. **/
    isLive?: 'true' | 'false'
    sendEmails?: 'true' | 'false'
    populateNotifications?: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура ответа POST запроса комментария'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentPostResponse {
    status: 'success' | 'failed'
    /** Включается в случае неудачи. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'empty-comment' | 'comment-too-big' | 'hash-tags-readonly' | 'mentions-readonly' | 'invalid-user' | 'unauthorized' | 'invalid-date' | 'invalid-name' | 'invalid-name-is-email' | 'banned' | 'invalid-email';
    /** Включается в случае неудачи. **/
    reason?: string
    /** Созданный комментарий. **/
    comment?: Comment
    /** Связанный пользователь, который мог существовать ранее или быть создан в ходе этого запроса. **/
    user?: User
}
[inline-code-end]