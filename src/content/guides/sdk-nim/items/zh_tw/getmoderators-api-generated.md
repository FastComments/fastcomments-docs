## 參數

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| skip | float64 | No |  |

## 回應

返回：[`Option[GetModeratorsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_moderators_response.nim)

## 範例

[inline-code-attrs-start title = 'getModerators 範例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (moderatorsOpt, httpResp) = client.getModerators(tenantId = "my-tenant-123", skip = 0.0)
if moderatorsOpt.isSome:
  let moderators = moderatorsOpt.get()
  echo moderators
[inline-code-end]