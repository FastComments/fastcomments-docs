## 매개변수

| 이름 | Type | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| commentId | string | 예 |  |
| isFlagged | bool | 아니요 |  |
| sso | string | 아니요 |  |

## 응답

반환: [`Option[FlagCommentPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_flag_comment_public200response.nim)

## 예제

[inline-code-attrs-start title = 'flagCommentPublic 예제'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.flagCommentPublic(
  tenantId = "my-tenant-123",
  commentId = "comment-98765",
  isFlagged = false,
  sso = ""
)
if response.isSome:
  let flagResult = response.get()
  discard flagResult
[inline-code-end]