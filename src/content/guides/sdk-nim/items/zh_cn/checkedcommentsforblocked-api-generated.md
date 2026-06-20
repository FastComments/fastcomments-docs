## 参数

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| commentIds | string | 否 |  |
| sso | string | 否 |  |

## 响应

返回: [`Option[CheckBlockedCommentsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_check_blocked_comments_response.nim)

## 示例

[inline-code-attrs-start title = 'checkedCommentsForBlocked 示例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.checkedCommentsForBlocked(
  tenantId = "my-tenant-123",
  commentIds = "",
  sso = ""
)
if response.isSome:
  let blockedResp = response.get()
  echo "Received blocked comments response: ", blockedResp
else:
  echo "No response body; HTTP status: ", $httpResponse.status
[inline-code-end]

---