## 매개변수

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| commentIds | string | 아니요 |  |
| sso | string | 아니요 |  |

## 응답

반환: [`Option[CheckBlockedCommentsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_check_blocked_comments_response.nim)

## 예제

[inline-code-attrs-start title = 'checkedCommentsForBlocked 예제'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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