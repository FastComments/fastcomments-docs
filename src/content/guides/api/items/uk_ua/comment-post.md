[api-resource-header-start name = 'Comment'; route = 'POST /api/v1/comments'; creditsCost = 1; api-resource-header-end]

Цей API-ендпоінт надає можливість створювати коментарі.

Типові варіанти використання: кастомні інтерфейси, інтеграції або імпорти.

Примітки:

- Цей API може оновлювати віджет коментарів "live" за бажанням (це збільшує `creditsCost` з `1` до `2`).
- Якщо вказано email, цей API автоматично створить об'єкти користувачів у нашій системі.
- Спроба зберегти два коментарі з різними email, але однаковим іменем користувача, призведе до помилки для другого коментаря. 
- Якщо ви вказуєте `parentId`, і дочірній коментар має `notificationSentForParent` зі значенням false, **ми відправимо повідомлення для батьківського коментаря**. Це робиться щогодини (ми групуємо повідомлення, щоб зменшити кількість відправлених електронних листів).
- Якщо ви хочете надсилати вітальні листи при створенні користувачів або листи для верифікації коментарів, встановіть `sendEmails` в `true` у параметрах запиту.
- Коментарі, створені через цей API, відображатимуться на сторінках Аналітики та Модерації в адмін-додатку.
- Якщо налаштування увімкнено, "bad words" все ще маскуються в іменах коментаторів та в тексті коментаря.
- Коментарі, створені через цей API, все ще можуть перевірятися на спам за бажанням.
- Налаштування, такі як максимальна довжина коментаря, якщо вони вказані через сторінку правил кастомізації в адмін-панелі, застосовуватимуться тут.

Мінімальні дані, необхідні для відправки і відображення у віджеті коментарів, такі:

[inline-code-attrs-start title = 'Мінімальний приклад POST cURL для коментаря'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

Більш реальний запит може виглядати так:

[inline-code-attrs-start title = 'Приклад POST cURL запиту для коментаря'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Структура POST-запиту коментаря'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentPostQueryParams {
    tenantId: string
    API_KEY: string
    doSpamCheck?: 'true' | 'false'
	/** Чи повинен коментар з'являтися "live" для користувачів, які переглядають екземпляри віджета коментарів з тим же urlId. ПРИМІТКА: Подвоює вартість у кредитах з 1 до 2. **/
    isLive?: 'true' | 'false'
    sendEmails?: 'true' | 'false'
    populateNotifications?: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура відповіді POST для коментаря'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentPostResponse {
    status: 'success' | 'failed'
    /** Повертається у разі помилки. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'empty-comment' | 'comment-too-big' | 'hash-tags-readonly' | 'mentions-readonly' | 'invalid-user' | 'unauthorized' | 'invalid-date' | 'invalid-name' | 'invalid-name-is-email' | 'banned' | 'invalid-email';
    /** Повертається у разі помилки. **/
    reason?: string
    /** Створений коментар. **/
    comment?: Comment
    /** Пов'язаний користувач — може вже існувати або бути щойно створеним. **/
    user?: User
}
[inline-code-end]