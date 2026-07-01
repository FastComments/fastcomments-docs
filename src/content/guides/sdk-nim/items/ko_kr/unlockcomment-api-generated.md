## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| broadcastId | string | No |  |
| sso | string = "" | No |  |

## 응답

반환: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## 예시

[inline-code-attrs-start title = 'unLockComment 예시'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (maybeEmpty, httpResp) = client.unLockComment(tenantId = "my-tenant-123", commentId = "comment-456", broadcastId = "", sso = "")
if maybeEmpty.isSome:
  let emptyResp = maybeEmpty.get()
  echo "Comment unlocked"
else:
  echo "Failed to unlock comment"
[inline-code-end]