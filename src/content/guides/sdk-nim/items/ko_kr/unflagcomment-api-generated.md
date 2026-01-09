## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 아니요 |  |
| userId | string | 아니요 |  |
| anonUserId | string | 아니요 |  |

## 응답

반환: [`Option[FlagComment_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_flag_comment200response.nim)

## 예제

[inline-code-attrs-start title = 'unFlagComment 예제'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.unFlagComment(
  tenantId = "my-tenant-123",
  id = "flag-789",
  userId = "",
  anonUserId = ""
)

if response.isSome:
  let flagResponse = response.get()
  echo "Comment unflagged successfully"
[inline-code-end]

---