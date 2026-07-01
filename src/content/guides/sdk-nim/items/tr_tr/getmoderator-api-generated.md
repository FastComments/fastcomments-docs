## Parametreler

| İsim | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | No |  |

## Yanıt

Döndürür: [`Option[GetModeratorResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_moderator_response.nim)

## Örnek

[inline-code-attrs-start title = 'getModerator Örneği'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (moderatorOpt, httpResponse) = client.getModerator(tenantId = "my-tenant-123", id = "moderator-456")
if moderatorOpt.isSome:
  let moderator = moderatorOpt.get()
  discard moderator
[inline-code-end]