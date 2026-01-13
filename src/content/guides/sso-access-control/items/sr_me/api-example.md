Овде ћемо проћи кроз позивање FastComments API-ја да бисмо подесили контролу приступа.

Пре него што почнемо, имајте у виду да не морамо експлицитно да креирамо структуру `Group`. Групе су једноставно идентификатори додати `Users` и `Pages`. Додавањем групе кориснику или страници аутоматски „креира“ група.

Прво, хајде да креирамо два корисника, `User A` и `User B`; поставићемо их у `Group X`:

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

Сада хајде да креирамо `Page`. Назваћемо је нашу `Confidential Page`, и до сада ниједан од ових корисника неће имати приступ јер ће она бити у групи `CONFIDENTIAL`:

[inline-code-attrs-start title = 'Пример cURL POST захтева за страницу'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

Корисници A и B тренутно **НЕ ИМАЈУ** приступ новој страници. Међутим, пошто су у истој групи, `GROUP-X`, могу се међусобно `@mention`-овати.

Ажурирајмо `User B` тако да сада може приступити страници:

[inline-code-attrs-start title = 'Пример cURL захтева за ажурирање корисника B'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/sso-users?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "groupIds": ["GROUP-X", "CONFIDENTIAL"]
}'
[inline-code-end]

`User B` сада припада обема групама. Наши корисници и даље могу међусобно да се `@mention`-ују, али само `User B` може прегледати нашу повјерљиву страницу.

Поставимо да `User B` може само прегледати повјерљиву страницу:

[inline-code-attrs-start title = 'Пример cURL захтева за ажурирање корисника B'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/sso-users?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "groupIds": ["CONFIDENTIAL"]
}'
[inline-code-end]

Сада он може прегледати повјерљиву страницу, али ниједан од наших корисника више не може међусобно да се `@mention`-ује, јер су у различитим групама.

Међутим, сваки корисник који није дио контроле приступа **ће моћи да приступи нашој страници**. Да бисте то спречили, обезбедите да ниједан SSO корисник нема `groupIds` постављен на null. На пример, хајде да креирамо `User C`, који има приступ свему:

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

Постављањем `groupIds` на null кажемо да они нису ограничени контролом приступа.

Сада хајде да креирамо страницу којој сви имају приступ:

[inline-code-attrs-start title = 'Пример cURL POST захтева за страницу'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

Постављањем `accessibleByGroupIds` на null кажемо да ова `Page` није контролисана преко контроле приступа, и обоје корисника могу да јој приступе.

Овиме завршавамо наш преглед API-ја за контролу приступа.