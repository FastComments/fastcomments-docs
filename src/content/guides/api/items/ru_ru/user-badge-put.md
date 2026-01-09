Этот endpoint позволяет обновить назначение бейджа пользователя.

В настоящее время единственное свойство, которое можно обновить — `displayedOnComments`, которое управляет тем, отображается ли бейдж на комментариях пользователя.

Пример запроса:

[inline-code-attrs-start title = 'Пример запроса PUT'; type = 'bash'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
curl -X PUT "https://fastcomments.com/api/v1/user-badges/badge123?tenantId=demo&API_KEY=DEMO_API_SECRET" \
-H "Content-Type: application/json" \
-d '{
  "displayedOnComments": false
}'
[inline-code-end]

Пример ответа:

[inline-code-attrs-start title = 'Ответ'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "success"
}
[inline-code-end]

Возможные ответы с ошибками:

[inline-code-attrs-start title = 'Ошибка: Отсутствует Tenant ID'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "failed",
  "code": "missing-tenant-id",
  "reason": "Tenant ID (query param: tenantId) is missing."
}
[inline-code-end]

[inline-code-attrs-start title = 'Ошибка: Отсутствует ID'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "failed",
  "code": "missing-id",
  "reason": "The User Badge ID (url param: id) is missing."
}
[inline-code-end]

[inline-code-attrs-start title = 'Ошибка: Не найдено'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "failed",
  "code": "not-found",
  "reason": "The user badge badge123 was not found."
}
[inline-code-end]