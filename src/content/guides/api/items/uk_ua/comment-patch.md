[api-resource-header-start name = 'Comment'; route = 'PATCH /api/v1/comments/:id'; creditsCost = 1; api-resource-header-end]

Цей API-ендпоінт надає можливість оновити один коментар.

Notes:

- Цей API може оновлювати віджет коментарів "live" якщо бажано (це збільшує базове `creditsCost` з `1` до `2`).
  - Це може зробити міграцію коментарів між сторінками "live" (зміна `urlId`).
  - Міграції коштують додаткові `2` кредити, оскільки сторінки попередньо обчислюються і це інтенсивно використовує CPU.
- На відміну від API створення, цей API НЕ автоматично створюватиме об'єкти користувачів у нашій системі, якщо вказано email.
- Коментарі, оновлені через цей API, все ще можуть перевірятися на спам за бажанням.
- Конфігурація, така як максимальна довжина коментаря, якщо налаштована через сторінку адміністрування Customization Rule, застосовуватиметься тут.
- Щоб дозволити користувачам оновлювати текст свого коментаря, ви можете просто вказати `comment` у тілі запиту. Ми згенеруємо відповідний `commentHTML`.
  - Якщо ви зазначите і `comment`, і `commentHTML`, ми не будемо автоматично генерувати HTML.
  - Якщо користувач додає згадки або хештеги у свій новий текст, вони все одно будуть оброблені так само, як у `POST` API.
- При оновленні `commenterEmail` у коментарі, краще також вказати `userId`. В іншому випадку потрібно переконатися, що користувач з цим email належить до вашого тенанта, інакше запит не пройде.  
- Якщо цільовий коментар заблоковано (`isLocked: true`), запит відхиляється з `code: 'locked'`. Спочатку розблокуйте коментар, оновіть його, потім знову заблокуйте за потреби.


[inline-code-attrs-start title = 'Мінімальний приклад cURL для PATCH коментаря'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET&isLive=true' \
  --header 'Content-Type: application/json' \
  --data '{
	"comment": "some-new-comment-text"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Структура PATCH-запиту для коментаря'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentPatchQueryParams {
    tenantId: string
    API_KEY: string
	/** Користувач, який виконує оновлення. За потреби можна використовувати для перевірки, чи має він право редагувати коментар.  **/
    contextUserId?: string
	/** Чи потрібно перевіряти, чи новий коментар виглядає як спам?  **/
    doSpamCheck?: 'true' | 'false'
	/** Чи повинен коментар відображатися "live" для користувачів, які переглядають екземпляри віджета коментарів з тим самим urlId. NOTE: Подвоює вартість в кредитах з 1 до 2. **/
    isLive?: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура відповіді PATCH для коментаря'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentPatchResponse {
    status: 'success' | 'failed'
    /** Додається у випадку невдачі. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'empty-comment' | 'comment-too-big' | 'hash-tags-readonly' | 'mentions-readonly' | 'invalid-user' | 'unauthorized' | 'invalid-date' | 'invalid-name' | 'invalid-name-is-email' | 'banned' | 'invalid-email' | 'invalid-input' | 'missing-id' | 'not-found' | 'locked'
    /** Додається у випадку невдачі. **/
    reason?: string
}
[inline-code-end]