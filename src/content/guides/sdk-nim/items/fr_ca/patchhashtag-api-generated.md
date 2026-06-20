## Paramètres

| Nom | Type | Requis | Description |
|------|------|----------|-------------|
| tag | string | Non |  |
| tenantId | string | Oui |  |
| updateHashTagBody | UpdateHashTagBody | Non |  |

## Réponse

Renvoie : [`Option[UpdateHashTagResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_update_hash_tag_response.nim)

## Exemple

[inline-code-attrs-start title = 'Exemple de patchHashTag'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.patchHashTag(tag = "breaking-news", tenantId = "my-tenant-123", updateHashTagBody = UpdateHashTagBody())
if response.isSome:
  let updatedHashTag = response.get()
  echo updatedHashTag
[inline-code-end]

---