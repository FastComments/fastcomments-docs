## 參數

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tag | string | 否 |  |
| tenantId | string | 是 |  |
| updateHashTagBody | UpdateHashTagBody | 否 |  |

## 回應

回傳: [`Option[UpdateHashTagResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_update_hash_tag_response.nim)

## 範例

[inline-code-attrs-start title = 'patchHashTag 範例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.patchHashTag(tag = "breaking-news", tenantId = "my-tenant-123", updateHashTagBody = UpdateHashTagBody())
if response.isSome:
  let updatedHashTag = response.get()
  echo updatedHashTag
[inline-code-end]

---