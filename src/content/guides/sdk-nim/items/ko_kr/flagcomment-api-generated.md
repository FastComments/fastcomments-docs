## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 아니오 |  |
| userId | string | 아니오 |  |
| anonUserId | string | 아니오 |  |

## 응답

반환: [`Option[FlagCommentResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_flag_comment_response.nim)

## 예제

[inline-code-attrs-start title = 'flagComment 예제'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.flagComment(
  tenantId = "my-tenant-123",
  id = "cmt-98765",
  userId = "user-12345",
  anonUserId = ""
)

if response.isSome:
  let flagResp = response.get()
  echo "Flag response received"
else:
  echo "No flag response returned"
[inline-code-end]

---