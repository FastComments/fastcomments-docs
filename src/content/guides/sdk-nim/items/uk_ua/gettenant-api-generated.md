## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| id | string | Ні |  |

## Відповідь

Повертає: [`Option[GetTenantResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_tenant_response.nim)

## Приклад

[inline-code-attrs-start title = 'Приклад getTenant'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getTenant(tenantId = "my-tenant-123", id = "tenant-789")
if response.isSome:
  let tenant = response.get()
  discard tenant
else:
  discard httpResponse
[inline-code-end]