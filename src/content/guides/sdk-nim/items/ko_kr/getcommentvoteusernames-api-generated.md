## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|------|-------------|
| tenantId | string | 예 |  |
| commentId | string | 예 |  |
| dir | int | 아니요 |  |
| sso | string | 아니요 |  |

## 응답

반환: [`Option[GetCommentVoteUserNamesSuccessResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_comment_vote_user_names_success_response.nim)

## 예제

[inline-code-attrs-start title = 'getCommentVoteUserNames 예제'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getCommentVoteUserNames(tenantId = "my-tenant-123", commentId = "cmt-987654", dir = 0, sso = "")
if response.isSome:
  let success: GetCommentVoteUserNamesSuccessResponse = response.get()
  discard success
[inline-code-end]

---