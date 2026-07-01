## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| id | string | Нет |  |
| updateTenantPackageBody | UpdateTenantPackageBody | Нет |  |

## Ответ

Возвращает: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Пример

[inline-code-attrs-start title = 'updateTenantPackage Пример'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let body = UpdateTenantPackageBody()
let (optResp, httpResp) = client.updateTenantPackage(
  tenantId = "my-tenant-123",
  id = "premium-plan",
  updateTenantPackageBody = body
)
if optResp.isSome:
  let empty = optResp.get()
[inline-code-end]