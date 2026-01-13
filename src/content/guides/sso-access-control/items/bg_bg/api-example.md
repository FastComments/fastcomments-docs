---
Тук ще преминем през извикване на FastComments API за настройване на контрол на достъпа.

Преди да започнем, отбележете, че не е необходимо да създаваме изрично структура `Group`. Групите са просто идентификатори
добавяни към `Users` и `Pages`. Добавянето на група към потребител или страница автоматично "създава" групата.

Първо, нека създадем двама потребители, `User A` и `User B`; ще ги поставим в `Group X`:

[inline-code-attrs-start title = 'Пример cURL за създаване на User A'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Пример cURL за създаване на User B'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

Сега нека създадем `Page`. Ще я наречем нашата `Confidential Page`, и към момента нито един от тези потребители няма да има достъп до нея, тъй като тя ще е в групата `CONFIDENTIAL`:

[inline-code-attrs-start title = 'Пример cURL POST за страница'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

Потребителите A и B в момента **НЯМАТ** достъп до новата страница. Въпреки това, тъй като са в една и съща група, `GROUP-X`, те могат да се `@mention` взаимно.

Нека актуализираме `User B`, така че да може да получи достъп до страницата:

[inline-code-attrs-start title = 'Пример cURL за актуализиране на User B'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/sso-users?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "groupIds": ["GROUP-X", "CONFIDENTIAL"]
}'
[inline-code-end]

`User B` вече принадлежи и към двете групи. Нашите потребители все още могат да се `@mention` взаимно, но само `User B` може да вижда нашата `Confidential Page`.

Нека направим така, че `User B` да може единствено да вижда `Confidential Page`:

[inline-code-attrs-start title = 'Пример cURL за актуализиране на User B'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/sso-users?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "groupIds": ["CONFIDENTIAL"]
}'
[inline-code-end]

Сега те могат да виждат конфиденциалната страница, но нито един от нашите потребители не може да се `@mention` взаимно, тъй като са в различни групи.

Въпреки това, всеки потребител, който не е част от контрола на достъпа, **ще може да получи достъп до нашата страница**. За да предотвратите това, уверете се, че нито един SSO Users няма `groupIds` зададени на null. Например, нека създадем `User C`, който има достъп до всичко:

[inline-code-attrs-start title = 'Пример cURL за създаване на User C'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

Като зададем `groupIds` на null, казваме, че те не са ограничени от контрола на достъпа.

Сега нека създадем страница, до която всички имат достъп:

[inline-code-attrs-start title = 'Пример cURL POST за страница'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

Като зададем `accessibleByGroupIds` на null, казваме, че тази `Page` не се контролира чрез контрол на достъпа, и и двамата потребители могат да имат достъп до нея.

Това завършва нашия преглед на API за Контрол на достъпа.

---