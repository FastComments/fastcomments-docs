## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| commentId | string | Да |  |
| broadcastId | string | Не |  |
| sso | string | Не |  |

## Отговор

Връща: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Пример

[inline-code-attrs-start title = 'lockComment Пример'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.lockComment(tenantId = "news-tenant-42", commentId = "cmt-8f3a2b9d", broadcastId = "", sso = "")
if response.isSome:
  let apiResp = response.get()
  echo "Locked comment successfully for tenant news-tenant-42"
else:
  echo "Failed to lock comment, HTTP status: ", $httpResponse.status
[inline-code-end]

---