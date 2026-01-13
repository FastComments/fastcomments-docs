## 參數

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| bulkCreateHashTagsBody | BulkCreateHashTagsBody | No |  |

## 回應

回傳: [`Option[AddHashTagsBulk_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_add_hash_tags_bulk200response.nim)

## 範例

[inline-code-attrs-start title = 'addHashTagsBulk 範例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let bulkBody = BulkCreateHashTagsBody(
  tags = @["breaking", "world-news", "economy"],
  createdBy = "editor@dailynews.com",
  replaceExisting = false
)
let (response, httpResponse) = client.addHashTagsBulk(tenantId = "newsroom-tenant-42", bulkCreateHashTagsBody = bulkBody)
if response.isSome:
  let created = response.get()
  echo created
else:
  echo "AddHashTagsBulk failed:", httpResponse
[inline-code-end]

---