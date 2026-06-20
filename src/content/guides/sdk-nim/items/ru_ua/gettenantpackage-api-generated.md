## Параметры

| Имя | Тип | Обязательный | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| id | string | Нет |  |

## Ответ

Возвращает: [`Option[GetTenantPackageResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_tenant_package_response.nim)

## Пример

[inline-code-attrs-start title = 'Пример getTenantPackage'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getTenantPackage(tenantId = "my-tenant-123", id = "premium-2026")
if response.isSome:
  let pkg = response.get()
  echo "Retrieved tenant package:"
  echo pkg
else:
  echo "Tenant package not found"
[inline-code-end]

---