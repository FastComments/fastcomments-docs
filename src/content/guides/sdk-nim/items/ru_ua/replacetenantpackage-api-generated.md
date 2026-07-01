## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|--------------|----------|
| tenantId | string | Да |  |
| id | string | Нет |  |
| replaceTenantPackageBody | ReplaceTenantPackageBody | Нет |  |

## Ответ

Возвращает: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Пример

[inline-code-attrs-start title = 'replaceTenantPackage Пример'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (optResp, httpResp) = client.replaceTenantPackage(
  tenantId = "my-tenant-123",
  id = "pkg-456",
  replaceTenantPackageBody = ReplaceTenantPackageBody()
)
if optResp.isSome:
  let resp = optResp.get()
  discard resp
[inline-code-end]