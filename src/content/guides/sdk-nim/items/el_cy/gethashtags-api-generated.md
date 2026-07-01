## Παράμετροι

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| page | float64 | No |  |

## Απάντηση

Επιστρέφει: [`Option[GetHashTagsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_hash_tags_response.nim)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getHashTags'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (hashTagsOpt, httpResp) = client.getHashTags(tenantId = "my-tenant-123", page = 0.0)
if hashTagsOpt.isSome:
  let hashTags = hashTagsOpt.get()
  echo hashTags
else:
  echo "No hashtags found"
[inline-code-end]

---