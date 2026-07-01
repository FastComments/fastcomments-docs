## Параметри

| Име | Тип | Задължително | Описание |
|------|------|--------------|----------|
| tenantId | string | Да |  |
| id | string | Не |  |

## Отговор

Връща: [`Option[GetTenantResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_tenant_response.nim)

## Пример

[inline-code-attrs-start title = 'Пример getTenant'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (tenantResponse, httpResponse) = client.getTenant(tenantId = "my-tenant-123", id = "config-001")
if tenantResponse.isSome:
  let tenant = tenantResponse.get()
  echo tenant
[inline-code-end]