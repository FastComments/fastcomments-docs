## Параметри

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| id | string | Не |  |
| sure | string = "" | Не |  |

## Отговор

Returns: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Пример

[inline-code-attrs-start title = 'deleteTenant Пример'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (respOpt, httpResp) = client.deleteTenant(tenantId = "my-tenant-123", id = "tenant-to-delete", sure = "yes")
if respOpt.isSome:
  let emptyResp = respOpt.get()
[inline-code-end]