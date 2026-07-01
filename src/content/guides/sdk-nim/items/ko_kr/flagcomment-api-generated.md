## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | No |  |
| options | FlagCommentOptions | No |  |

## 응답

반환: [`Option[FlagCommentResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_flag_comment_response.nim)

## 예시

[inline-code-attrs-start title = 'flagComment 예시'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let options = FlagCommentOptions(reason: "spam content", note: "Automated posting detected", isSpam: true, categories: @["spam"])
let (flagRes, httpRes) = client.flagComment(tenantId = "my-tenant-123", id = "cmt-789", options = options)
if flagRes.isSome:
  let res = flagRes.get()
  discard res
[inline-code-end]

---