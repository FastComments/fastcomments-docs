## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 아니요 |  |
| redirectURL | string | 아니요 |  |

## 응답

반환: [`Option[FlagCommentPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_flag_comment_public200response.nim)

## 예제

[inline-code-attrs-start title = 'sendLoginLink 예제'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.sendLoginLink(tenantId = "fastcomments-tenant-42", id = "user-9876", redirectURL = "https://news.example.com/articles/2026/fastcomments-login")
if response.isSome:
  let loginResp = response.get()
  echo "Login link sent successfully"
else:
  echo "Failed to send login link"
[inline-code-end]

---