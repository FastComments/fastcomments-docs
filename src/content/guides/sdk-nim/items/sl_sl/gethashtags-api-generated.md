## Parameters

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| page | float64 | No |  |

## Odgovor

Vrne: [`Option[GetHashTagsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_hash_tags_response.nim)

## Primer

[inline-code-attrs-start title = 'getHashTags Primer'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (hashTagsOpt, httpResp) = client.getHashTags(tenantId = "my-tenant-123", page = 0.0)
if hashTagsOpt.isSome:
  let hashTags = hashTagsOpt.get()
  echo hashTags
else:
  echo "No hashtags found"
[inline-code-end]