## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 아니요 |  |
| blockFromCommentParams | BlockFromCommentParams | 아니요 |  |
| userId | string | 아니요 |  |
| anonUserId | string | 아니요 |  |

## 응답

반환: [`Option[BlockSuccess]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_block_success.nim)

## 예제

[inline-code-attrs-start title = 'blockUserFromComment 예제'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.blockUserFromComment(
  tenantId = "my-tenant-123",
  id = "cmt-7890",
  blockFromCommentParams = BlockFromCommentParams(
    reason = "Repeated abusive language",
    durationMinutes = 1440,
    notifyUser = true,
    tags = @["abuse", "automated"]
  ),
  userId = "user-456",
  anonUserId = ""
)

if response.isSome:
  let result = response.get()
  discard result
else:
  discard httpResponse
[inline-code-end]

---