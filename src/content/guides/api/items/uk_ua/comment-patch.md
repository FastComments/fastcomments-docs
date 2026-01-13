[api-resource-header-start name = 'Comment'; route = 'PATCH /api/v1/comments/:id'; creditsCost = 1; api-resource-header-end]

Цей кінцевий пункт API надає можливість оновити один коментар.

Notes:

- Цей API може оновлювати віджет коментарів "live", якщо потрібно (це збільшує базовий `creditsCost` з `1` до `2`).
  - Це дозволяє робити міграцію коментарів між сторінками "live" (зміна `urlId`).
  - Міграції коштують додаткових `2` кредити, оскільки сторінки попередньо обчислюються і це інтенсивно використовує CPU.
- На відміну від API створення, цей API НЕ буде автоматично створювати об'єкти користувача в нашій системі, якщо надано email.
- Коментарі, оновлені через цей API, все ще можуть перевірятися на спам за бажанням.
- Налаштування, такі як максимальна довжина коментаря, якщо вони налаштовані через сторінку адміністратора Customization Rule, застосовуватимуться тут.
- Щоб дозволити користувачам оновлювати текст свого коментаря, ви можете просто вказати `comment` у тілі запиту. Ми згенеруємо відповідний `commentHTML`.
  - Якщо ви визначите і `comment`, і `commentHTML`, ми не будемо автоматично генерувати HTML.
  - Якщо користувач додасть згадки або хештеги у свій новий текст, вони все одно будуть оброблені так само, як у `POST` API.
- Під час оновлення `commenterEmail` у коментарі найкраще також вказати `userId`. Інакше ви повинні переконатися, що користувач з цим email належить вашому tenant, інакше запит зазнає невдачі.  


[inline-code-attrs-start title = 'Мінімальний приклад PATCH-запиту cURL для коментаря'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET&isLive=true' \
  --header 'Content-Type: application/json' \
  --data '{
	"comment": "some-new-comment-text"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Структура PATCH-запиту коментаря'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentPatchQueryParams {
    tenantId: string
    API_KEY: string
	/** Користувач, який виконує оновлення. За потреби може використовуватися для перевірки, чи може він редагувати коментар.  **/
    contextUserId?: string
	/** Потрібно перевіряти, чи новий коментар схожий на спам?  **/
    doSpamCheck?: 'true' | 'false'
	/** Чи має коментар з'являтися "live" для користувачів, які переглядають екземпляри віджета коментарів з тим самим urlId. ПРИМІТКА: Подвоює вартість у кредитах з 1 до 2. **/
    isLive?: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура відповіді PATCH для коментаря'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentPatchResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'empty-comment' | 'comment-too-big' | 'hash-tags-readonly' | 'mentions-readonly' | 'invalid-user' | 'unauthorized' | 'invalid-date' | 'invalid-name' | 'invalid-name-is-email' | 'banned' | 'invalid-email' | 'invalid-input' | 'missing-id' | 'not-found'
    /** Included on failure. **/
    reason?: string
}
[inline-code-end]

---