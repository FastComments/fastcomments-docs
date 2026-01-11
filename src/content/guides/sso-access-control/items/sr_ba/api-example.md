Овдје ћемо проћи кроз позивање FastComments API-ја да поставимо контролу приступа.

Пре него што почнемо, имајте на уму да не морамо експлицитно креирати структуру `Group`. Groups су једноставно идентификатори
додати `Users` и `Pages`. Додавање групе кориснику или страници аутоматски "креира" групу.

Прво, хајде да креирамо два корисника, `User A` и `User B`, поставићемо их у `Group X`:

[inline-code-attrs-start title = 'Пример cURL захтева за креирање корисника A'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Пример cURL захтева за креирање корисника B'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

Сада хајде да креирамо једну `Page`. Назваћемо је нашу `Confidential Page`, и до сада ниједан од ових корисника неће имати приступ, јер ће она бити у групи `CONFIDENTIAL`:

[inline-code-attrs-start title = 'Пример cURL POST захтева за креирање странице'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

`User A` и `User B` тренутно **НЕМАЈУ** приступ новој страници. Међутим, пошто су у истој групи, `GROUP-X`, они могу `@mention` један другог.

Ажурирајмо `User B` тако да он/она сада може приступити страници:

[inline-code-attrs-start title = 'Пример cURL захтева за ажурирање корисника B'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/sso-users?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "groupIds": ["GROUP-X", "CONFIDENTIAL"]
}'
[inline-code-end]

`User B` сада припада обема групама. Наши корисници и даље могу `@mention` један другог, али само `User B` може видјети нашу поверљиву страницу.

Учинимо да `User B` може само видјети поверљиву страницу:

[inline-code-attrs-start title = 'Пример cURL захтева за ажурирање корисника B'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/sso-users?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "groupIds": ["CONFIDENTIAL"]
}'
[inline-code-end]

Сада они могу видјети поверљиву страницу, али ниједан од наших корисника не може `@mention` другог, јер су у различитим групама.

Међутим, сваки корисник који није дио контроле приступа **ће моћи приступити нашој страници**. Да бисте то спречили, осигурајте да ниједан SSO User нема своје `groupIds` постављено на null. На пример, хајде да креирамо `User C`, који има приступ свему:

[inline-code-attrs-start title = 'Пример cURL захтева за креирање корисника C'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

Постављањем `groupIds` на null, кажемо да они нису ограничени контролом приступа.

Сада хајде да креирамо страницу којој сви имају приступ:

[inline-code-attrs-start title = 'Пример cURL POST захтева за креирање странице'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

Постављањем `accessibleByGroupIds` на null кажемо да ова `Page` није контролисана преко контроле приступа, и оба корисника могу јој приступити.

Овим завршавамо наш преглед FastComments API-ја за контролу приступа.