## Parameter

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| bulkCreateHashTagsBody | BulkCreateHashTagsBody | No |  |

## Antwort

Rückgabe: [`Option[BulkCreateHashTagsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_bulk_create_hash_tags_response.nim)

## Beispiel

[inline-code-attrs-start title = 'addHashTagsBulk Beispiel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (optResp, httpResp) = client.addHashTagsBulk(
  tenantId = "my-tenant-123",
  bulkCreateHashTagsBody = BulkCreateHashTagsBody(
    hashTags = @["news", "technology"]
  )
)

if optResp.isSome:
  let resp = optResp.get()
[inline-code-end]

---