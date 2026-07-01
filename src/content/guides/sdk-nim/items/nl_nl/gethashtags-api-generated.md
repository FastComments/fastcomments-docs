## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|-----------|--------------|
| tenantId | string | Ja |  |
| page | float64 | Nee |  |

## Respons

Retourneert: [`Option[GetHashTagsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_hash_tags_response.nim)

## Voorbeeld

[inline-code-attrs-start title = 'getHashTags Voorbeeld'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (hashTagsOpt, httpResp) = client.getHashTags(tenantId = "my-tenant-123", page = 0.0)
if hashTagsOpt.isSome:
  let hashTags = hashTagsOpt.get()
  echo hashTags
else:
  echo "No hashtags found"
[inline-code-end]