## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| commentId | string | 예 |  |
| broadcastId | string | 아니오 |  |
| sso | string | 아니오 |  |

## 응답

반환: [`Option[LockComment_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_lock_comment200response.nim)

## 예제

[inline-code-attrs-start title = 'unLockComment 예제'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.unLockComment(tenantId = "my-tenant-123", commentId = "cmt-7f9a3b2d", broadcastId = "", sso = "")
if response.isSome:
  let lockResult = response.get()
  echo "Unlock response: ", $lockResult
else:
  echo "Unlock failed, HTTP response: ", $httpResponse
[inline-code-end]

---