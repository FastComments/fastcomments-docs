## Parameters

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| tag | string | Не |  |
| deleteHashTagRequestBody | DeleteHashTagRequestBody | Не |  |

## Response

Връща: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Пример

[inline-code-attrs-start title = 'Пример за deleteHashTag'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (apiResp, httpResp) = client.deleteHashTag(tenantId = "my-tenant-123", tag = "sports", deleteHashTagRequestBody = DeleteHashTagRequestBody())
if apiResp.isSome:
  let emptyResp = apiResp.get()
[inline-code-end]

---