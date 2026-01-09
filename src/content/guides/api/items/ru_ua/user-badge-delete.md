---
Этот конечный пункт API позволяет удалить назначение значка пользователя.

Example Request:

[inline-code-attrs-start title = 'Пример DELETE-запроса'; type = 'bash'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
curl -X DELETE "https://fastcomments.com/api/v1/user-badges/badge123?tenantId=demo&API_KEY=DEMO_API_SECRET"
[inline-code-end]

Example Response:

[inline-code-attrs-start title = 'Ответ'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "success"
}
[inline-code-end]

Possible Error Responses:

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
---