## 매개변수

| Name | Type | Required | Description |
|------|------|----------|-------------|
| commentId | string | 예 |  |
| sso | string | 아니오 |  |

## 응답

반환: [`Option[GetCommentBanStatusResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_comment_ban_status_response.nim)

## 예제

[inline-code-attrs-start title = 'getCommentBanStatus 예제'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getCommentBanStatus(commentId = "cmt-987654321", sso = "")

if response.isSome:
  let banStatus = response.get()
  echo "Ban status for comment cmt-987654321: ", banStatus
else:
  echo "No ban status returned for comment cmt-987654321"
[inline-code-end]

---