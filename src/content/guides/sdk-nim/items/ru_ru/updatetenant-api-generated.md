## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| id | string | Нет |  |
| updateTenantBody | UpdateTenantBody | Нет |  |

## Ответ

Возвращает: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Пример

[inline-code-attrs-start title = 'Пример updateTenant'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let updateBody = UpdateTenantBody(
  name: "My Tenant",
  description: "Tenant for news site",
  isActive: true,
  allowedDomains: @["example.com", "news.com"]
)

let (apiResp, httpResp) = client.updateTenant(
  tenantId = "my-tenant-123",
  id = "tenant-456",
  updateTenantBody = updateBody
)

if apiResp.isSome:
  let emptyResp = apiResp.get()
[inline-code-end]