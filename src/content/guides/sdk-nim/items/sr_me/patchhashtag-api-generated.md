## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Da |  |
| tag | string | Ne |  |
| updateHashTagBody | UpdateHashTagBody | Ne |  |

## Odgovor

Vraća: [`Option[UpdateHashTagResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_update_hash_tag_response.nim)

## Primjer

[inline-code-attrs-start title = 'patchHashTag primjer'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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