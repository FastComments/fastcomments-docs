## 參數

| 名稱 | 類型 | 必需 | 說明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| options | GetApiIdsOptions | 否 |  |

## 回應

返回: [`Option[ModerationAPIGetCommentIdsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_moderation_api_get_comment_ids_response.nim)

## 範例

[inline-code-attrs-start title = 'getApiIds 範例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let opts = GetApiIdsOptions()
let (maybeResponse, httpResponse) = client.getApiIds(tenantId = "my-tenant-123", options = opts)
if maybeResponse.isSome:
  let response = maybeResponse.get()
  echo response
[inline-code-end]

---