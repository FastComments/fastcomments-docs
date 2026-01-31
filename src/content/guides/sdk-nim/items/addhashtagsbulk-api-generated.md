## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| bulkCreateHashTagsBody | BulkCreateHashTagsBody | No |  |

## Response

Returns: [`Option[AddHashTagsBulk_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_add_hash_tags_bulk200response.nim)

## Example

[inline-code-attrs-start title = 'addHashTagsBulk Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let bulkBody = BulkCreateHashTagsBody(hashTags = @["world-news", "breaking", "technology"])
let (response, httpResponse) = client.addHashTagsBulk(tenantId = "news-tenant-42", bulkCreateHashTagsBody = bulkBody)
if response.isSome:
  let created = response.get()
  discard created
[inline-code-end]
