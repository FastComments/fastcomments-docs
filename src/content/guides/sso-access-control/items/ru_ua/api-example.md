Здесь мы пройдемся по вызову API FastComments для настройки контроля доступа.

Прежде чем начать, обратите внимание, что нам не нужно явно создавать структуру `Group`. Группы — это просто идентификаторы
добавляемые к `Users` и `Pages`. Добавление группы к пользователю или странице автоматически "создает" группу.

Сначала создадим двух пользователей, `User A` и `User B`, поместим их в `Group X`:

[inline-code-attrs-start title = 'Пример cURL запроса для создания User A'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Пример cURL запроса для создания User B'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

Теперь создадим `Page`. Назовем её `Confidential Page`, и пока ни у кого из этих пользователей не будет к ней доступа, так как она будет
в группе `CONFIDENTIAL`:

[inline-code-attrs-start title = 'Пример cURL POST запроса для Page'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

Обновим `User B`, чтобы он теперь имел доступ к странице:

[inline-code-attrs-start title = 'Пример cURL запроса для обновления User B'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/sso-users?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "groupIds": ["GROUP-X", "CONFIDENTIAL"]
}'
[inline-code-end]

`User B` теперь принадлежит обеим группам. Наши пользователи по-прежнему могут `@mention` друг друга, но только `User B` может просматривать нашу конфиденциальную страницу.

Сделаем так, чтобы `User B` мог просматривать только конфиденциальную страницу:

[inline-code-attrs-start title = 'Пример cURL запроса для обновления User B'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/sso-users?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "groupIds": ["CONFIDENTIAL"]
}'
[inline-code-end]

Теперь он может просматривать конфиденциальную страницу, но ни один из наших пользователей не может `@mention` другого, так как они находятся в разных группах.

Однако любой пользователь, который не является частью контроля доступа, **сможет получить доступ к нашей странице**. Чтобы предотвратить это, убедитесь, что у SSO-пользователей нет значения `groupIds`, установленного в null. Например, создадим `User C`, который имеет доступ ко всему:

[inline-code-attrs-start title = 'Пример cURL запроса для создания User C'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

Установив `groupIds` в null, мы говорим, что они не ограничены контролем доступа.

Теперь создадим страницу, к которой у всех будет доступ:

[inline-code-attrs-start title = 'Пример cURL POST запроса для Page'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

Установив `accessibleByGroupIds` в null, мы говорим, что эта `Page` не контролируется через контроль доступа, и оба пользователя могут получить к ней доступ.

На этом завершается наш обзор использования API для контроля доступа.