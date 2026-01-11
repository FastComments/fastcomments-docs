Тут ми пройдемо процес виклику API FastComments для налаштування контролю доступу.

Перш ніж почати, зауважте, що нам не потрібно явно створювати структуру `Group`. Групи — це просто ідентифікатори
додані до `Users` та `Pages`. Додавання групи до користувача або сторінки автоматично "creates" the group.

Спочатку створимо двох користувачів, `User A` та `User B`; розмістимо їх у `Group X`:

[inline-code-attrs-start title = 'Приклад cURL-запиту для створення користувача A'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/sso-users?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"id": "user-a",
	"username": "User A",
	"email": "usera@example.com",
    "groupIds": ["GROUP-X"]
}'
[inline-code-end]

[inline-code-attrs-start title = 'Приклад cURL-запиту для створення користувача B'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/sso-users?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"id": "user-b",
	"username": "User B",
	"email": "userb@example.com",
    "groupIds": ["GROUP-X"]
}'
[inline-code-end]

Тепер створимо `Page`. Назвемо її `Confidential Page`, і наразі жоден із цих користувачів не матиме до неї доступу, оскільки вона належатиме групі `CONFIDENTIAL`:

[inline-code-attrs-start title = 'Приклад cURL POST для сторінки'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/pages?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"title": "Confidential Page",
	"url": "https://mysite.com/confidential",
	"urlId": "https://mysite.com/confidential",
	"accessibleByGroupIds": ["CONFIDENTIAL"]
}'
[inline-code-end]

Користувачі A і B наразі **НЕ МАЮТЬ** доступу до нової сторінки. Однак, оскільки вони в одній групі, `GROUP-X`, вони можуть `@mention` один одного.

Оновимо `User B`, щоб він тепер міг отримати доступ до сторінки:

[inline-code-attrs-start title = 'Приклад cURL для оновлення користувача B'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/sso-users?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "groupIds": ["GROUP-X", "CONFIDENTIAL"]
}'
[inline-code-end]

`User B` тепер належить до обох груп. Наші користувачі досі можуть `@mention` один одного, але тільки `User B` може переглядати нашу конфіденційну сторінку.

Зробимо так, щоб `User B` міг бачити тільки конфіденційну сторінку:

[inline-code-attrs-start title = 'Приклад cURL для оновлення користувача B'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/sso-users?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "groupIds": ["CONFIDENTIAL"]
}'
[inline-code-end]

Тепер він може переглядати конфіденційну сторінку, але жоден з наших користувачів не може `@mention` один одного, оскільки вони в різних групах.

Однак будь-який користувач, який не є частиною контролю доступу, **зможе отримати доступ до нашої сторінки**. Щоб уникнути цього, переконайтеся, що жоден SSO Users не має `groupIds`, встановленого в null. Наприклад, створімо `User C`, який має доступ до всього:

[inline-code-attrs-start title = 'Приклад cURL-запиту для створення користувача C'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/sso-users?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"id": "user-c",
	"username": "User C",
	"email": "userc@example.com",
    "groupIds": null
}'
[inline-code-end]

Встановивши `groupIds` в null, ми вказуємо, що вони не обмежені контролем доступу.

Тепер створімо сторінку, до якої мають доступ усі:

[inline-code-attrs-start title = 'Приклад cURL POST для сторінки'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/pages?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"title": "Public Page",
	"url": "https://mysite.com/public",
	"urlId": "https://mysite.com/public",
	"accessibleByGroupIds": null
}'
[inline-code-end]

Встановивши `accessibleByGroupIds` в null, ми вказуємо, що ця `Page` не контролюється через контроль доступу, і обидва користувачі можуть отримати до неї доступ.

Це завершує наш огляд API щодо контролю доступу.