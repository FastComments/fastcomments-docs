## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| urlId | string | 예 |  |
| userId | string | 아니오 |  |
| anonUserId | string | 아니오 |  |

## 응답

반환: [`Option[GetVotesForUserResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_votes_for_user_response.nim)

## 예제

[inline-code-attrs-start title = 'getVotesForUser 예제'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getVotesForUser(
  tenantId = "my-tenant-123",
  urlId = "news/article-title",
  userId = "user-789",
  anonUserId = ""
)
if response.isSome:
  let votes = response.get()
  echo "User votes retrieved"
else:
  echo "No votes found"
[inline-code-end]

---