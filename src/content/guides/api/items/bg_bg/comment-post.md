[api-resource-header-start name = 'Comment'; route = 'POST /api/v1/comments'; creditsCost = 1; api-resource-header-end]

Тази API крайна точка предоставя възможност за създаване на коментари.

Често срещани случаи на употреба са персонализирани потребителски интерфейси, интеграции или импортиране.

Забележки:

- Този API може да актуализира уиджета за коментари "на живо" ако желаете (това увеличава `creditsCost` от `1` на `2`).
- Този API автоматично ще създаде потребителски обекти в нашата система, ако е предоставен имейл.
- Опитът да запазите два коментара с различни имейли, но с едно и също потребителско име, ще доведе до грешка за втория коментар.
- Ако посочвате `parentId` и дъщерният коментар има `notificationSentForParent` като false, **ние ще изпратим известия за родителския коментар**. Това се прави на всеки час (групираме известията заедно, за да намалим броя на изпратените имейли).
- Ако искате да изпращате приветствени имейли при създаване на потребители или имейли за потвърждение на коментари, задайте `sendEmails` на `true` в параметрите на заявката.
- Коментарите, създадени чрез този API, ще се показват в страниците за анализ и модерация на административното приложение.
- "лошите думи" все още се маскират в имената на коментиращите и текста на коментарите, ако настройката е включена.
- Коментарите, създадени чрез този API, все още могат да бъдат проверявани за спам, ако желаете.
- Конфигурацията като максимална дължина на коментара, ако е конфигурирана чрез административната страница за правила за персонализация, ще се прилага тук.

Минималните данни, необходими за изпращане, които ще се показват в уиджета за коментари, са следните:

[inline-code-attrs-start title = 'Пример за минимален POST на коментар с cURL'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

По-реалистична заявка може да изглежда така:

[inline-code-attrs-start title = 'Пример за POST на коментар с cURL'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Структура на заявката за POST на коментар'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentPostQueryParams {
    tenantId: string
    API_KEY: string
    doSpamCheck?: 'true' | 'false'
	/** Whether the comment should appear "live" to users viewing instances of the comment widget with the same urlId. NOTE: Doubles credit cost from 1 to 2. **/
    isLive?: 'true' | 'false'
    sendEmails?: 'true' | 'false'
    populateNotifications?: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура на отговора за POST на коментар'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentPostResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'empty-comment' | 'comment-too-big' | 'hash-tags-readonly' | 'mentions-readonly' | 'invalid-user' | 'unauthorized' | 'invalid-date' | 'invalid-name' | 'invalid-name-is-email' | 'banned' | 'invalid-email';
    /** Included on failure. **/
    reason?: string
    /** The created comment. **/
    comment?: Comment
    /** The associated user, which may or may not have already existed. **/
    user?: User
}
[inline-code-end]
