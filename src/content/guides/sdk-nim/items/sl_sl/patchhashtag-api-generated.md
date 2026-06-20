## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tag | string | Ne |  |
| tenantId | string | Da |  |
| updateHashTagBody | UpdateHashTagBody | Ne |  |

## Odziv

Vrne: [`Option[UpdateHashTagResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_update_hash_tag_response.nim)

## Primer

[inline-code-attrs-start title = 'Primer patchHashTag'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.patchHashTag(tag = "breaking-news", tenantId = "my-tenant-123", updateHashTagBody = UpdateHashTagBody())
if response.isSome:
  let updatedHashTag = response.get()
  echo updatedHashTag
[inline-code-end]

---