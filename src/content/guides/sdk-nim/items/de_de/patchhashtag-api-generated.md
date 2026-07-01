## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|--------------|--------------|
| tenantId | string | Ja |  |
| tag | string | Nein |  |
| updateHashTagBody | UpdateHashTagBody | Nein |  |

## Antwort

Rückgabe: [`Option[UpdateHashTagResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_update_hash_tag_response.nim)

## Beispiel

[inline-code-attrs-start title = 'patchHashTag Beispiel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let updateBody = UpdateHashTagBody()
let (optResp, httpResp) = client.patchHashTag(
  tenantId = "my-tenant-123",
  tag = "news",
  updateHashTagBody = updateBody
)
if optResp.isSome:
  let resp = optResp.get()
  echo resp
else:
  echo "No response"
[inline-code-end]