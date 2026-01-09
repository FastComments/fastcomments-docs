## Parametreler

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| id | string | Hayır |  |
| userId | string | Hayır |  |
| anonUserId | string | Hayır |  |

## Yanıt

Döndürür: [`Option[FlagComment_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_flag_comment200response.nim)

## Örnek

[inline-code-attrs-start title = 'flagComment Örneği'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.flagComment(tenantId = "my-tenant-123", id = "cmt-98765", userId = "user-8342", anonUserId = "")
if response.isSome:
  let flagged = response.get()
  echo "Flagged comment response: ", flagged
else:
  echo "Flag comment failed: ", httpResponse
[inline-code-end]

---