---
Здесь мы пройдемся по вызову API FastComments для настройки управления доступом.

Прежде чем начать, обратите внимание, что нам не нужно явно создавать структуру `Group`. Группы — это просто идентификаторы
добавляемые к `Users` и `Pages`. Добавление группы пользователю или странице автоматически "создает" группу.

Сначала давайте создадим двух пользователей, `User A` и `User B`, и добавим их в `Group X`:

[inline-code-attrs-start title = 'Пример cURL для создания User A'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Пример cURL для создания User B'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

Теперь давайте создадим `Page`. Назовем её `Confidential Page`, и пока ни один из этих пользователей не будет иметь к ней доступа, так как она будет
в группе `CONFIDENTIAL`:

[inline-code-attrs-start title = 'Пример cURL POST для страницы'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

Пользователи A и B в настоящее время **НЕ** имеют доступа к новой странице. Однако, поскольку они находятся в одной группе, `GROUP-X`, они могут `@mention` друг друга.

Давайте обновим `User B`, чтобы он теперь мог получить доступ к странице:

[inline-code-attrs-start title = 'Пример cURL для обновления User B'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/sso-users?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "groupIds": ["GROUP-X", "CONFIDENTIAL"]
}'
[inline-code-end]

`User B` теперь принадлежит обеим группам. Наши пользователи всё ещё могут `@mention` друг друга, но просматривать конфиденциальную страницу может только `User B`.

Сделаем так, чтобы `User B` мог просматривать только конфиденциальную страницу:

[inline-code-attrs-start title = 'Пример cURL для обновления User B'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/sso-users?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "groupIds": ["CONFIDENTIAL"]
}'
[inline-code-end]

Теперь он может просматривать конфиденциальную страницу, но ни один из наших пользователей не может `@mention` другого, так как они находятся в разных группах.

Однако любой пользователь, не являющийся частью контроля доступа, **сможет получить доступ к нашей странице**. Чтобы предотвратить это, убедитесь, что у ни одного SSO Users значение `groupIds` не установлено в null. Например, давайте создадим `User C`, который имеет доступ ко всему:

[inline-code-attrs-start title = 'Пример cURL для создания User C'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

Установив `groupIds` в null, мы указываем, что они не ограничены контролем доступа.

Теперь давайте создадим страницу, к которой есть доступ у всех:

[inline-code-attrs-start title = 'Пример cURL POST для страницы'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

Установив `accessibleByGroupIds` в null, мы указываем, что эта `Page` не контролируется через систему управления доступом, и оба пользователя могут получить к ней доступ.

На этом наше руководство по работе с API для управления доступом завершено.

---