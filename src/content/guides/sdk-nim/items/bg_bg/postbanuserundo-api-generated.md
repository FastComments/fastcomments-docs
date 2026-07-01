---
## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| banUserUndoParams | BanUserUndoParams | No |  |
| sso | string = "" | No |  |

## Отговор

Returns: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Пример

[inline-code-attrs-start title = 'postBanUserUndo Пример'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (apiResp, httpResp) = client.postBanUserUndo(
  tenantId = "my-tenant-123",
  banUserUndoParams = BanUserUndoParams(userId = "user-456"),
  sso = ""
)

if apiResp.isSome:
  let _ = apiResp.get()
[inline-code-end]

---