---
## 매개변수

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| bulkCreateHashTagsBody | BulkCreateHashTagsBody | 아니오 |  |

## 응답

반환: [`Option[BulkCreateHashTagsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_bulk_create_hash_tags_response.nim)

## 예제

[inline-code-attrs-start title = 'addHashTagsBulk 예제'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.addHashTagsBulk(tenantId = "my-tenant-123", bulkCreateHashTagsBody = BulkCreateHashTagsBody(hashTags = @["news", "breaking", "politics"], replaceExisting = false))
if response.isSome:
  let result = response.get()
  echo "Bulk tags response:", result
else:
  echo "No response body, HTTP status:", httpResponse.statusCode
[inline-code-end]

---