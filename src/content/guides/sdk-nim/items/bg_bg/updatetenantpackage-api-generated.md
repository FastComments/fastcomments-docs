---
## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| id | string | Не |  |
| updateTenantPackageBody | UpdateTenantPackageBody | Не |  |

## Отговор

Връща: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Пример

[inline-code-attrs-start title = 'Пример за updateTenantPackage'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let packageBody = UpdateTenantPackageBody(
  name: "Pro Plan",
  priceCents: 1999,
  active: true,
  features: @["priority-support", "advanced-moderation"]
)

let (response, httpResponse) = client.updateTenantPackage(
  tenantId = "my-tenant-123",
  id = "pkg-789",
  updateTenantPackageBody = packageBody
)

if response.isSome:
  let apiEmpty = response.get()
  echo "Tenant package updated successfully, HTTP status: " & $httpResponse.status
[inline-code-end]

---