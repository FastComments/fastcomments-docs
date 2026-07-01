## Параметри

| Назва | Тип | Обов'язковий | Опис |
|------|------|--------------|------|
| tenantId | string | Так |  |
| id | string | Ні |  |
| sure | string = "" | Ні |  |

## Відповідь

Повертає: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Приклад

[inline-code-attrs-start title = 'deleteTenant Приклад'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (respOpt, httpResp) = client.deleteTenant(tenantId = "my-tenant-123", id = "tenant-to-delete", sure = "yes")
if respOpt.isSome:
  let emptyResp = respOpt.get()
[inline-code-end]