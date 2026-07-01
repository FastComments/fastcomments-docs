## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| commentId | string | Oui |  |
| broadcastId | string | Non |  |
| sso | string = "" | Non |  |

## Réponse

Renvoie : [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Exemple

[inline-code-attrs-start title = 'Exemple lockComment'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (lockResult, httpRes) = client.lockComment(
  tenantId = "my-tenant-123",
  commentId = "comment-456",
  broadcastId = "",
  sso = "")

if lockResult.isSome:
  let resp = lockResult.get()
  discard resp
[inline-code-end]