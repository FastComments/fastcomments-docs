## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| urlId | string | Да |  |
| sso | string = "" | Не |  |

## Отговор

Връща: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Пример

[inline-code-attrs-start title = 'Пример за putReopenThread'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (apiRespOpt, httpResp) = client.putReopenThread(
  tenantId = "my-tenant-123",
  urlId = "news/article-title",
  sso = ""
)

if apiRespOpt.isSome:
  let emptyResp = apiRespOpt.get()
  discard
[inline-code-end]