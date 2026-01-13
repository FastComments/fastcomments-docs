---
## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|----------|-------------|
| tag | string | 否 |  |
| tenantId | string | 是 |  |
| updateHashTagBody | UpdateHashTagBody | 否 |  |

## 回應

回傳: [`Option[PatchHashTag_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_patch_hash_tag200response.nim)

## 範例

[inline-code-attrs-start title = 'patchHashTag 範例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.patchHashTag(tag = "politics", tenantId = "my-tenant-123", updateHashTagBody = UpdateHashTagBody())

if response.isSome:
  let updated = response.get()
  echo "Hashtag updated successfully"
else:
  echo "Failed to update hashtag, status:", httpResponse.status
[inline-code-end]

---