## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tag | string | Nein |  |
| tenantId | string | Ja |  |
| updateHashTagBody | UpdateHashTagBody | Nein |  |

## Antwort

Gibt zurück: [`Option[UpdateHashTagResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_update_hash_tag_response.nim)

## Beispiel

[inline-code-attrs-start title = 'patchHashTag Beispiel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.patchHashTag(tag = "breaking-news", tenantId = "my-tenant-123", updateHashTagBody = UpdateHashTagBody())
if response.isSome:
  let updatedHashTag = response.get()
  echo updatedHashTag
[inline-code-end]

---