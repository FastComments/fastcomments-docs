## Parameters

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 아니오 |  |
| options | UnFlagCommentOptions | 아니오 |  |

## 응답

반환: [`Option[FlagCommentResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_flag_comment_response.nim)

## 예시

[inline-code-attrs-start title = 'unFlagComment 예시'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (flagRespOpt, httpResp) = client.unFlagComment(tenantId = "my-tenant-123", id = "comment-456", options = UnFlagCommentOptions())
if flagRespOpt.isSome:
  let flagResp = flagRespOpt.get()
[inline-code-end]

---