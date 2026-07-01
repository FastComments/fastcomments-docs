## Параметри

| Ім'я | Тип | Обов'язково | Опис |
|------|------|-------------|------|
| tenantId | string | Так |  |
| id | string | Ні |  |

## Відповідь

Повертає: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Приклад

[inline-code-attrs-start title = 'Приклад deleteTenantPackage'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (apiResponse, httpResponse) = client.deleteTenantPackage(tenantId = "my-tenant-123", id = "pkg-456def")
if apiResponse.isSome:
  let emptyResponse = apiResponse.get()
[inline-code-end]